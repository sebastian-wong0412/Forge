import { Navigate, Route, Routes } from "react-router-dom";
import { Layout } from "./components/Layout";
import { OnboardingGate } from "./components/OnboardingGate";
import { ExampleProvider } from "./example/ExampleProvider";
import { CycleDetailPage } from "./pages/CycleDetailPage";
import { CyclesPage } from "./pages/CyclesPage";
import { ObjectiveDetailPage } from "./pages/ObjectiveDetailPage";
import { ProjectDetailPage } from "./pages/ProjectDetailPage";
import { SettingsPage } from "./pages/SettingsPage";
import { TaskDetailPage } from "./pages/TaskDetailPage";
import { TodayPage } from "./pages/TodayPage";
import { WelcomePage } from "./pages/WelcomePage";

export function App() {
  return (
    <ExampleProvider>
      <OnboardingGate>
        <Routes>
          <Route path="/welcome" element={<WelcomePage />} />
          <Route element={<Layout />}>
            <Route path="/" element={<Navigate to="/today" replace />} />
            <Route path="/today" element={<TodayPage />} />
            <Route path="/cycles" element={<CyclesPage />} />
            <Route path="/cycles/:cycleId" element={<CycleDetailPage />} />
            <Route path="/objectives/:objectiveId" element={<ObjectiveDetailPage />} />
            <Route path="/projects/:projectId" element={<ProjectDetailPage />} />
            <Route path="/tasks/:taskId" element={<TaskDetailPage />} />
            <Route path="/settings" element={<SettingsPage />} />
          </Route>
        </Routes>
      </OnboardingGate>
    </ExampleProvider>
  );
}
