import { NavLink, Outlet } from "react-router-dom";
import { useT } from "../i18n";

export function Layout() {
  const t = useT();

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="brand">Forge</div>
        <nav className="nav" aria-label={t("nav.main")}>
          <NavLink to="/today" className={({ isActive }) => `nav-link${isActive ? " active" : ""}`}>
            {t("nav.today")}
          </NavLink>
          <NavLink to="/cycles" className={({ isActive }) => `nav-link${isActive ? " active" : ""}`}>
            {t("nav.cycles")}
          </NavLink>
        </nav>
        <nav className="nav nav-footer" aria-label={t("nav.settings")}>
          <NavLink
            to="/settings"
            className={({ isActive }) => `nav-link${isActive ? " active" : ""}`}
          >
            {t("nav.settings")}
          </NavLink>
        </nav>
      </aside>
      <main className="main">
        <Outlet />
      </main>
    </div>
  );
}
