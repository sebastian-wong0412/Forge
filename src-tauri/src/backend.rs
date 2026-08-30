use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use tauri::Manager;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

pub const DESKTOP_BIND_ADDR: &str = "127.0.0.1:17340";
pub const DESKTOP_API_URL: &str = "http://127.0.0.1:17340";

const CREATE_NO_WINDOW: u32 = 0x0800_0000;
const HEALTH_TIMEOUT: Duration = Duration::from_secs(20);

pub struct BackendState {
    child: Mutex<Option<Child>>,
    #[cfg(windows)]
    _job: Mutex<Option<crate::job::KillOnCloseJob>>,
}

impl BackendState {
    fn new() -> Self {
        Self {
            child: Mutex::new(None),
            #[cfg(windows)]
            _job: Mutex::new(None),
        }
    }

    pub fn shutdown(&self) {
        if let Ok(mut child) = self.child.lock() {
            if let Some(mut process) = child.take() {
                let _ = process.kill();
                let _ = process.wait();
            }
        }
    }
}

pub fn start(app: &tauri::App) -> Result<(), String> {
    let data_dir = app_data_dir(app)?;
    std::fs::create_dir_all(&data_dir).map_err(|err| format!("create data dir: {err}"))?;

    let database_path = data_dir.join(database_file_name());
    let log_path = data_dir.join("forge-server.log");
    let sidecar = sidecar_path()?;

    app.manage(BackendState::new());

    if health_ok() {
        return Ok(());
    }

    let mut command = Command::new(&sidecar);
    command
        .env("FORGE_DATABASE_PATH", &database_path)
        .env("FORGE_BIND_ADDR", DESKTOP_BIND_ADDR)
        .env("FORGE_LOG_LEVEL", "info")
        .current_dir(&data_dir)
        .stdin(Stdio::null());

    command.stdout(stdio_to_log(&log_path));
    command.stderr(stdio_to_log(&log_path));

    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);

    let child = command
        .spawn()
        .map_err(|err| format!("start backend `{}`: {err}", sidecar.display()))?;

    #[cfg(windows)]
    let job = crate::job::KillOnCloseJob::new().and_then(|job| {
        job.assign(&child)?;
        Ok(job)
    });

    {
        let state = app.state::<BackendState>();
        let mut slot = state
            .child
            .lock()
            .map_err(|_| "backend state lock poisoned".to_string())?;
        *slot = Some(child);

        #[cfg(windows)]
        if let Ok(job) = job {
            if let Ok(mut job_slot) = state._job.lock() {
                *job_slot = Some(job);
            }
        }
    }

    wait_until_healthy()
}

pub fn stop(app: &tauri::AppHandle) {
    if let Some(state) = app.try_state::<BackendState>() {
        state.shutdown();
    }
}

fn app_data_dir(app: &tauri::App) -> Result<PathBuf, String> {
    app.path()
        .app_local_data_dir()
        .map_err(|err| format!("resolve local app data: {err}"))
}

fn database_file_name() -> &'static str {
    if cfg!(debug_assertions) {
        "forge-dev.db"
    } else {
        "forge.db"
    }
}

fn sidecar_path() -> Result<PathBuf, String> {
    let exe = std::env::current_exe().map_err(|err| format!("current exe: {err}"))?;
    let exe_dir = exe
        .parent()
        .ok_or_else(|| "desktop executable has no parent directory".to_string())?;

    let adjacent = exe_dir.join(sidecar_file_name(None));
    if adjacent.exists() {
        return Ok(adjacent);
    }

    let triple = env!("TARGET_TRIPLE");
    let with_triple = exe_dir.join(sidecar_file_name(Some(triple)));
    if with_triple.exists() {
        return Ok(with_triple);
    }

    let dev_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("binaries")
        .join(sidecar_file_name(Some(triple)));
    if dev_path.exists() {
        return Ok(dev_path);
    }

    Err(format!(
        "backend executable not found (looked for `{}`, `{}`, `{}`)",
        adjacent.display(),
        with_triple.display(),
        dev_path.display()
    ))
}

fn sidecar_file_name(triple: Option<&str>) -> String {
    let stem = match triple {
        Some(triple) => format!("forge-server-{triple}"),
        None => "forge-server".to_string(),
    };
    if cfg!(windows) {
        format!("{stem}.exe")
    } else {
        stem
    }
}

fn stdio_to_log(path: &Path) -> Stdio {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map(Stdio::from)
        .unwrap_or_else(|_| Stdio::null())
}

fn wait_until_healthy() -> Result<(), String> {
    let deadline = Instant::now() + HEALTH_TIMEOUT;
    while Instant::now() < deadline {
        if health_ok() {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    Err(format!(
        "backend did not become ready at {DESKTOP_API_URL} within {}s",
        HEALTH_TIMEOUT.as_secs()
    ))
}

fn health_ok() -> bool {
    let Ok(addr) = DESKTOP_BIND_ADDR.parse::<SocketAddr>() else {
        return false;
    };
    let Ok(mut stream) = TcpStream::connect_timeout(&addr, Duration::from_millis(300)) else {
        return false;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_millis(400)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(400)));
    if stream
        .write_all(b"GET /health HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .is_err()
    {
        return false;
    }
    let mut body = String::new();
    let _ = stream.read_to_string(&mut body);
    body.contains("\"status\":\"ok\"") || body.contains("\"status\": \"ok\"")
}
