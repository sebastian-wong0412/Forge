import { NavLink, Outlet } from "react-router-dom";

export function Layout() {
  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="brand">Forge</div>
        <nav className="nav" aria-label="主导航">
          <NavLink to="/today" className={({ isActive }) => `nav-link${isActive ? " active" : ""}`}>
            今日
          </NavLink>
          <NavLink to="/cycles" className={({ isActive }) => `nav-link${isActive ? " active" : ""}`}>
            周期
          </NavLink>
          <span className="nav-disabled" aria-disabled="true">
            设置
            <span className="nav-note">即将推出</span>
          </span>
        </nav>
      </aside>
      <main className="main">
        <Outlet />
      </main>
    </div>
  );
}
