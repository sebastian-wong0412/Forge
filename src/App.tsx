import { Navigate, Route, Routes } from "react-router-dom";
import { Layout } from "./components/Layout";
import { CycleDetailPage } from "./pages/CycleDetailPage";
import { CyclesPage } from "./pages/CyclesPage";
import { ObjectiveDetailPage } from "./pages/ObjectiveDetailPage";
import { ProjectDetailPage } from "./pages/ProjectDetailPage";
import { TaskDetailPage } from "./pages/TaskDetailPage";
import { TodayPage } from "./pages/TodayPage";

export function App() {
  return (
    <Routes>
      <Route element={<Layout />}>
        <Route path="/" element={<Navigate to="/today" replace />} />
        <Route path="/today" element={<TodayPage />} />
        <Route path="/cycles" element={<CyclesPage />} />
        <Route path="/cycles/:cycleId" element={<CycleDetailPage />} />
        <Route path="/objectives/:objectiveId" element={<ObjectiveDetailPage />} />
        <Route path="/projects/:projectId" element={<ProjectDetailPage />} />
        <Route path="/tasks/:taskId" element={<TaskDetailPage />} />
      </Route>
    </Routes>
  );
}
