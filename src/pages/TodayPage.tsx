import { useState } from "react";
import { getCycles, getToday } from "../api";
import { ErrorState } from "../components/ErrorState";
import { LoadingState } from "../components/LoadingState";
import { ScheduleDialog } from "../components/ScheduleDialog";
import { TodayView } from "../components/TodayView";
import { useOptionalExample } from "../example/ExampleProvider";
import { useLoad } from "../hooks/useLoad";
import { useTaskMutations } from "../hooks/useTaskMutations";
import { useT } from "../i18n";
import { localCalendarDate } from "../lib/dates";
import { filterToday, visibleCycles } from "../lib/exampleWorkspace";
import { loadProjectTitles, loadWorkableProjects, uniqueProjectIds } from "../lib/todayProjects";

export function TodayPage() {
  const t = useT();
  const example = useOptionalExample();
  const localToday = localCalendarDate();
  const [date, setDate] = useState(localToday);
  const { data, error, loading, reload } = useLoad(() => getToday(date), [date]);
  const cycles = useLoad(getCycles, []);
  const mutations = useTaskMutations(reload);
  const visibleToday = data && example ? filterToday(data, example.state) : data;
  const visibleCycleList = example
    ? visibleCycles(cycles.data ?? [], example.state)
    : (cycles.data ?? []);
  const projectIdsKey = visibleToday ? uniqueProjectIds(visibleToday).sort().join(",") : "";
  const cycleKey = visibleCycleList.map((cycle) => `${cycle.id}:${cycle.updated_at}`).join(",");
  const projectTitles = useLoad(
    () => (projectIdsKey ? loadProjectTitles(projectIdsKey.split(",")) : Promise.resolve({})),
    [projectIdsKey],
  );
  const workableProjects = useLoad(
    () =>
      visibleCycleList.length === 0
        ? Promise.resolve([])
        : loadWorkableProjects(visibleCycleList),
    [cycleKey],
  );

  if (loading && !data) {
    return <LoadingState label={t("today.loading")} />;
  }
  if (error && !data) {
    return <ErrorState message={error} onRetry={() => void reload()} />;
  }
  if (!visibleToday) {
    return <ErrorState message={t("error.todayLoadFailed")} onRetry={() => void reload()} />;
  }

  return (
    <>
      {mutations.error ? <ErrorState message={mutations.error} /> : null}
      <TodayView
        today={visibleToday}
        localToday={localToday}
        cycles={visibleCycleList}
        projects={workableProjects.data ?? []}
        projectTitles={projectTitles.data ?? undefined}
        busyId={mutations.busyId}
        onDateChange={setDate}
        onStart={mutations.start}
        onComplete={mutations.complete}
        onCancel={mutations.cancel}
        onSchedule={mutations.setScheduling}
        onUnschedule={mutations.unschedule}
      />
      {mutations.scheduling ? (
        <ScheduleDialog
          task={mutations.scheduling}
          onClose={() => mutations.setScheduling(null)}
          onSave={mutations.saveSchedule}
        />
      ) : null}
    </>
  );
}
