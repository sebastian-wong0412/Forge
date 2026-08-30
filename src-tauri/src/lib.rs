mod backend;
#[cfg(windows)]
mod job;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = match tauri::Builder::default()
        .setup(|app| {
            backend::start(app).map_err(|err| -> Box<dyn std::error::Error> { err.into() })?;
            Ok(())
        })
        .build(tauri::generate_context!())
    {
        Ok(app) => app,
        Err(err) => {
            eprintln!("Forge failed to start: {err}");
            std::process::exit(1);
        }
    };

    app.run(|app_handle, event| {
        if let tauri::RunEvent::Exit = event {
            backend::stop(app_handle);
        }
    });
}
