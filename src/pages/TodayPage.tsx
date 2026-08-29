import { useState } from "react";
import { getCycles, getToday } from "../api";
import { ErrorState } from "../components/ErrorState";
import { LoadingState } from "../components/LoadingState";
import { ScheduleDialog } from "../components/ScheduleDialog";
import { TodayView } from "../components/TodayView";
import { useLoad } from "../hooks/useLoad";
import { useTaskMutations } from "../hooks/useTaskMutations";
import { localCalendarDate } from "../lib/dates";

export function TodayPage() {
  const localToday = localCalendarDate();
  const [date, setDate] = useState(localToday);
  const { data, error, loading, reload } = useLoad(() => getToday(date), [date]);
  const cycles = useLoad(getCycles, []);
  const mutations = useTaskMutations(reload);

  if (loading && !data) {
    return <LoadingState label="正在加载今日…" />;
  }
  if (error && !data) {
    return <ErrorState message={error} onRetry={() => void reload()} />;
  }
  if (!data) {
    return <ErrorState message="无法加载今日任务。" onRetry={() => void reload()} />;
  }

  return (
    <>
      {mutations.error ? <ErrorState message={mutations.error} /> : null}
      <TodayView
        today={data}
        localToday={localToday}
        hasCycles={(cycles.data?.length ?? 0) > 0}
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
