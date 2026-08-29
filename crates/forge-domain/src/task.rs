use time::{Date, OffsetDateTime};

use crate::DomainError;
use crate::ids::{ProjectId, TaskId};
use crate::status::TaskStatus;
use crate::title::Title;
use crate::util::empty_to_none;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TodayBucket {
    Scheduled,
    Overdue,
    UnscheduledInProgress,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    id: TaskId,
    project_id: ProjectId,
    title: Title,
    description: Option<String>,
    status: TaskStatus,
    scheduled_on: Option<Date>,
    completed_at: Option<OffsetDateTime>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl Task {
    #[must_use]
    pub fn create(
        project_id: ProjectId,
        title: Title,
        description: Option<String>,
        scheduled_on: Option<Date>,
        now: OffsetDateTime,
    ) -> Self {
        Self {
            id: TaskId::new(),
            project_id,
            title,
            description: empty_to_none(description),
            status: TaskStatus::Todo,
            scheduled_on,
            completed_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: TaskId,
        project_id: ProjectId,
        title: Title,
        description: Option<String>,
        status: TaskStatus,
        scheduled_on: Option<Date>,
        completed_at: Option<OffsetDateTime>,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            project_id,
            title,
            description,
            status,
            scheduled_on,
            completed_at,
            created_at,
            updated_at,
        }
    }

    pub fn update(
        &mut self,
        title: Title,
        description: Option<String>,
        now: OffsetDateTime,
    ) -> Result<(), DomainError> {
        if self.status.is_terminal() {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: self.status.as_str(),
            });
        }
        self.title = title;
        self.description = empty_to_none(description);
        self.updated_at = now;
        Ok(())
    }

    pub fn start(&mut self, now: OffsetDateTime) -> Result<(), DomainError> {
        if self.status != TaskStatus::Todo {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: TaskStatus::InProgress.as_str(),
            });
        }
        self.status = TaskStatus::InProgress;
        self.updated_at = now;
        Ok(())
    }

    pub fn complete(&mut self, now: OffsetDateTime) -> Result<(), DomainError> {
        if self.status != TaskStatus::InProgress {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: TaskStatus::Done.as_str(),
            });
        }
        self.status = TaskStatus::Done;
        self.completed_at = Some(now);
        self.updated_at = now;
        Ok(())
    }

    pub fn cancel(&mut self, now: OffsetDateTime) -> Result<(), DomainError> {
        if !matches!(self.status, TaskStatus::Todo | TaskStatus::InProgress) {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: TaskStatus::Cancelled.as_str(),
            });
        }
        self.status = TaskStatus::Cancelled;
        self.updated_at = now;
        Ok(())
    }

    pub fn schedule(
        &mut self,
        scheduled_on: Option<Date>,
        now: OffsetDateTime,
    ) -> Result<(), DomainError> {
        if self.status.is_terminal() {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: self.status.as_str(),
            });
        }
        self.scheduled_on = scheduled_on;
        self.updated_at = now;
        Ok(())
    }

    #[must_use]
    pub fn id(&self) -> TaskId {
        self.id
    }

    #[must_use]
    pub fn project_id(&self) -> ProjectId {
        self.project_id
    }

    #[must_use]
    pub fn title(&self) -> &Title {
        &self.title
    }

    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    #[must_use]
    pub fn status(&self) -> TaskStatus {
        self.status
    }

    #[must_use]
    pub fn scheduled_on(&self) -> Option<Date> {
        self.scheduled_on
    }

    #[must_use]
    pub fn completed_at(&self) -> Option<OffsetDateTime> {
        self.completed_at
    }

    #[must_use]
    pub fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }

    #[must_use]
    pub fn updated_at(&self) -> OffsetDateTime {
        self.updated_at
    }
}

/// Classifies a task for a requested calendar date.
///
/// Completed tasks use the UTC calendar date of `completed_at`.
#[must_use]
pub fn today_bucket(task: &Task, date: Date) -> Option<TodayBucket> {
    match task.status() {
        TaskStatus::Done => {
            if task
                .completed_at()
                .is_some_and(|completed_at| completed_at.date() == date)
            {
                Some(TodayBucket::Completed)
            } else {
                None
            }
        }
        TaskStatus::Cancelled => None,
        TaskStatus::Todo | TaskStatus::InProgress => match task.scheduled_on() {
            Some(scheduled_on) if scheduled_on == date => Some(TodayBucket::Scheduled),
            Some(scheduled_on) if scheduled_on < date => Some(TodayBucket::Overdue),
            None if task.status() == TaskStatus::InProgress => {
                Some(TodayBucket::UnscheduledInProgress)
            }
            _ => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use time::macros::{date, datetime};

    use super::*;

    const NOW: OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);
    const TODAY: Date = date!(2026 - 08 - 30);
    const TOMORROW: Date = date!(2026 - 08 - 31);
    const YESTERDAY: Date = date!(2026 - 08 - 29);

    fn task() -> Task {
        Task::create(
            ProjectId::new(),
            Title::parse("Do it").unwrap(),
            None,
            None,
            NOW,
        )
    }

    fn task_on(scheduled_on: Option<Date>) -> Task {
        Task::create(
            ProjectId::new(),
            Title::parse("Do it").unwrap(),
            None,
            scheduled_on,
            NOW,
        )
    }

    #[test]
    fn valid_transitions() {
        let mut task = task();
        task.start(NOW).unwrap();
        assert_eq!(task.status(), TaskStatus::InProgress);
        task.complete(NOW).unwrap();
        assert_eq!(task.status(), TaskStatus::Done);
        assert_eq!(task.completed_at(), Some(NOW));
    }

    #[test]
    fn cancel_from_todo_and_in_progress() {
        let mut todo = task();
        todo.cancel(NOW).unwrap();
        assert_eq!(todo.status(), TaskStatus::Cancelled);
        assert!(!todo.status().is_active());

        let mut started = task();
        started.start(NOW).unwrap();
        started.cancel(NOW).unwrap();
        assert_eq!(started.status(), TaskStatus::Cancelled);
    }

    #[test]
    fn terminal_states_reject_further_changes() {
        let mut done = task();
        done.start(NOW).unwrap();
        done.complete(NOW).unwrap();
        assert!(done.start(NOW).is_err());
        assert!(done.complete(NOW).is_err());
        assert!(done.cancel(NOW).is_err());
        assert!(
            done.update(Title::parse("Nope").unwrap(), None, NOW)
                .is_err()
        );

        let mut cancelled = task();
        cancelled.cancel(NOW).unwrap();
        assert!(cancelled.start(NOW).is_err());
        assert!(cancelled.complete(NOW).is_err());
    }

    #[test]
    fn create_without_scheduled_on() {
        let task = task();
        assert_eq!(task.scheduled_on(), None);
        assert_eq!(task.status(), TaskStatus::Todo);
    }

    #[test]
    fn create_with_scheduled_on() {
        let task = task_on(Some(TODAY));
        assert_eq!(task.scheduled_on(), Some(TODAY));
        assert_eq!(task.status(), TaskStatus::Todo);
    }

    #[test]
    fn schedule_todo() {
        let mut task = task();
        task.schedule(Some(TODAY), NOW).unwrap();
        assert_eq!(task.scheduled_on(), Some(TODAY));
        assert_eq!(task.status(), TaskStatus::Todo);
    }

    #[test]
    fn reschedule_todo() {
        let mut task = task_on(Some(TODAY));
        task.schedule(Some(TOMORROW), NOW).unwrap();
        assert_eq!(task.scheduled_on(), Some(TOMORROW));
        assert_eq!(task.status(), TaskStatus::Todo);
    }

    #[test]
    fn unschedule_todo() {
        let mut task = task_on(Some(TODAY));
        task.schedule(None, NOW).unwrap();
        assert_eq!(task.scheduled_on(), None);
        assert_eq!(task.status(), TaskStatus::Todo);
    }

    #[test]
    fn schedule_in_progress() {
        let mut task = task();
        task.start(NOW).unwrap();
        task.schedule(Some(TODAY), NOW).unwrap();
        assert_eq!(task.scheduled_on(), Some(TODAY));
        assert_eq!(task.status(), TaskStatus::InProgress);
    }

    #[test]
    fn terminal_task_cannot_schedule() {
        let mut done = task();
        done.start(NOW).unwrap();
        done.complete(NOW).unwrap();
        let err = done.schedule(Some(TODAY), NOW).unwrap_err();
        assert!(matches!(err, DomainError::InvalidStatusTransition { .. }));
        assert_eq!(done.scheduled_on(), None);

        let mut cancelled = task();
        cancelled.cancel(NOW).unwrap();
        assert!(cancelled.schedule(Some(TODAY), NOW).is_err());
    }

    #[test]
    fn terminal_task_cannot_unschedule() {
        let mut done = task_on(Some(TODAY));
        done.start(NOW).unwrap();
        done.complete(NOW).unwrap();
        let err = done.schedule(None, NOW).unwrap_err();
        assert!(matches!(err, DomainError::InvalidStatusTransition { .. }));
        assert_eq!(done.scheduled_on(), Some(TODAY));

        let mut cancelled = task_on(Some(TODAY));
        cancelled.cancel(NOW).unwrap();
        assert!(cancelled.schedule(None, NOW).is_err());
        assert_eq!(cancelled.scheduled_on(), Some(TODAY));
    }

    #[test]
    fn start_preserves_scheduled_on() {
        let mut task = task_on(Some(TODAY));
        task.start(NOW).unwrap();
        assert_eq!(task.scheduled_on(), Some(TODAY));
    }

    #[test]
    fn complete_preserves_scheduled_on() {
        let mut task = task_on(Some(TODAY));
        task.start(NOW).unwrap();
        task.complete(NOW).unwrap();
        assert_eq!(task.scheduled_on(), Some(TODAY));
        assert_eq!(task.status(), TaskStatus::Done);
    }

    #[test]
    fn cancel_preserves_scheduled_on() {
        let mut task = task_on(Some(TODAY));
        task.cancel(NOW).unwrap();
        assert_eq!(task.scheduled_on(), Some(TODAY));
        assert_eq!(task.status(), TaskStatus::Cancelled);
    }

    #[test]
    fn todo_scheduled_today_is_scheduled() {
        assert_eq!(
            today_bucket(&task_on(Some(TODAY)), TODAY),
            Some(TodayBucket::Scheduled)
        );
    }

    #[test]
    fn in_progress_scheduled_today_is_scheduled() {
        let mut task = task_on(Some(TODAY));
        task.start(NOW).unwrap();
        assert_eq!(today_bucket(&task, TODAY), Some(TodayBucket::Scheduled));
    }

    #[test]
    fn todo_scheduled_yesterday_is_overdue() {
        assert_eq!(
            today_bucket(&task_on(Some(YESTERDAY)), TODAY),
            Some(TodayBucket::Overdue)
        );
    }

    #[test]
    fn in_progress_scheduled_yesterday_is_overdue() {
        let mut task = task_on(Some(YESTERDAY));
        task.start(NOW).unwrap();
        assert_eq!(today_bucket(&task, TODAY), Some(TodayBucket::Overdue));
    }

    #[test]
    fn todo_scheduled_tomorrow_is_none() {
        assert_eq!(today_bucket(&task_on(Some(TOMORROW)), TODAY), None);
    }

    #[test]
    fn in_progress_scheduled_tomorrow_is_none() {
        let mut task = task_on(Some(TOMORROW));
        task.start(NOW).unwrap();
        assert_eq!(today_bucket(&task, TODAY), None);
    }

    #[test]
    fn in_progress_without_schedule_is_unscheduled_in_progress() {
        let mut task = task();
        task.start(NOW).unwrap();
        assert_eq!(
            today_bucket(&task, TODAY),
            Some(TodayBucket::UnscheduledInProgress)
        );
    }

    #[test]
    fn todo_without_schedule_is_none() {
        assert_eq!(today_bucket(&task(), TODAY), None);
    }

    #[test]
    fn done_completed_on_requested_utc_date_is_completed() {
        let mut task = task();
        task.start(NOW).unwrap();
        let completed_at = datetime!(2026-08-30 15:00:00 UTC);
        task.complete(completed_at).unwrap();
        assert_eq!(today_bucket(&task, TODAY), Some(TodayBucket::Completed));
    }

    #[test]
    fn done_completed_on_another_utc_date_is_none() {
        let mut task = task();
        task.start(NOW).unwrap();
        task.complete(datetime!(2026-08-31 15:00:00 UTC)).unwrap();
        assert_eq!(today_bucket(&task, TODAY), None);
    }

    #[test]
    fn cancelled_is_none() {
        let mut task = task_on(Some(TODAY));
        task.cancel(NOW).unwrap();
        assert_eq!(today_bucket(&task, TODAY), None);
    }

    #[test]
    fn scheduled_done_is_completed_when_completed_at_matches() {
        let mut task = task_on(Some(TODAY));
        task.start(NOW).unwrap();
        task.complete(datetime!(2026-08-30 09:00:00 UTC)).unwrap();
        assert_eq!(task.scheduled_on(), Some(TODAY));
        assert_eq!(today_bucket(&task, TODAY), Some(TodayBucket::Completed));
        assert_eq!(today_bucket(&task, TOMORROW), None);
    }

    #[test]
    fn completed_bucket_uses_utc_midnight_boundary() {
        let mut before = task();
        before.start(NOW).unwrap();
        before.complete(datetime!(2026-08-30 23:59:59 UTC)).unwrap();
        assert_eq!(today_bucket(&before, TODAY), Some(TodayBucket::Completed));
        assert_eq!(today_bucket(&before, TOMORROW), None);

        let mut after = task();
        after.start(NOW).unwrap();
        after.complete(datetime!(2026-08-31 00:00:00 UTC)).unwrap();
        assert_eq!(today_bucket(&after, TODAY), None);
        assert_eq!(today_bucket(&after, TOMORROW), Some(TodayBucket::Completed));
    }
}
