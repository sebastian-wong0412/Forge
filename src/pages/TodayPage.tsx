import { useState } from "react";
import { getCycles, getToday } from "../api";
import { ErrorState } from "../components/ErrorState";
import { LoadingState } from "../components/LoadingState";
import { ScheduleDialog } from "../components/ScheduleDialog";
import { TodayView } from "../components/TodayView";
import { useLoad } from "../hooks/useLoad";
import { useTaskMutations } from "../hooks/useTaskMutations";
import { useT } from "../i18n";
import { localCalendarDate } from "../lib/dates";

export function TodayPage() {
  const t = useT();
  const localToday = localCalendarDate();
  const [date, setDate] = useState(localToday);
  const { data, error, loading, reload } = useLoad(() => getToday(date), [date]);
  const cycles = useLoad(getCycles, []);
  const mutations = useTaskMutations(reload);

  if (loading && !data) {
    return <LoadingState label={t("today.loading")} />;
  }
  if (error && !data) {
    return <ErrorState message={error} onRetry={() => void reload()} />;
  }
  if (!data) {
    return <ErrorState message={t("error.todayLoadFailed")} onRetry={() => void reload()} />;
  }

  return (
    <>
      {mutations.error ? <ErrorState message={mutations.error} /> : null}
      <TodayView
        today={data}
        localToday={localToday}
        cycles={cycles.data ?? []}
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
