export type IsoDate = string;
export type Rfc3339 = string;

export type CycleStatus = "planning" | "active" | "closed" | "archived";
export type ObjectiveStatus = "draft" | "active" | "completed" | "archived";
export type KeyResultStatus = "draft" | "active" | "completed" | "archived";
export type ProjectStatus = "draft" | "active" | "completed" | "archived";
export type TaskStatus = "todo" | "in_progress" | "done" | "cancelled";

export interface ApiErrorBody {
  error: {
    code: string;
    message: string;
  };
}

export interface Cycle {
  id: string;
  name: string;
  start_on: IsoDate;
  end_on: IsoDate;
  status: CycleStatus;
  created_at: Rfc3339;
  updated_at: Rfc3339;
}

export interface Objective {
  id: string;
  cycle_id: string;
  title: string;
  description: string | null;
  status: ObjectiveStatus;
  start_on: IsoDate | null;
  end_on: IsoDate | null;
  created_at: Rfc3339;
  updated_at: Rfc3339;
}

export interface KeyResult {
  id: string;
  objective_id: string;
  title: string;
  description: string | null;
  status: KeyResultStatus;
  start_value: number;
  current_value: number;
  target_value: number | null;
  progress: number | null;
  unit: string | null;
  created_at: Rfc3339;
  updated_at: Rfc3339;
}

export interface CheckIn {
  id: string;
  key_result_id: string;
  value: number;
  note: string | null;
  checked_on: IsoDate;
  created_at: Rfc3339;
  updated_at: Rfc3339;
}

export interface Project {
  id: string;
  objective_id: string;
  title: string;
  description: string | null;
  status: ProjectStatus;
  created_at: Rfc3339;
  updated_at: Rfc3339;
}

export interface Task {
  id: string;
  project_id: string;
  title: string;
  description: string | null;
  status: TaskStatus;
  scheduled_on: IsoDate | null;
  completed_at: Rfc3339 | null;
  created_at: Rfc3339;
  updated_at: Rfc3339;
}

export interface Review {
  id: string;
  cycle_id: string;
  content: string;
  period_start: IsoDate | null;
  period_end: IsoDate | null;
  created_at: Rfc3339;
  updated_at: Rfc3339;
}

export interface TodayResponse {
  date: IsoDate;
  scheduled: Task[];
  overdue: Task[];
  unscheduled_in_progress: Task[];
  completed: Task[];
}

export interface CreateCycleInput {
  name: string;
  start_on: IsoDate;
  end_on: IsoDate;
}

export interface CreateObjectiveInput {
  title: string;
  description?: string | null;
  start_on?: IsoDate | null;
  end_on?: IsoDate | null;
}

export interface CreateKeyResultInput {
  title: string;
  description?: string | null;
  start_value: number;
  target_value?: number | null;
  unit?: string | null;
}

export interface CreateCheckInInput {
  value: number;
  note?: string | null;
  checked_on: IsoDate;
}

export interface CreateProjectInput {
  title: string;
  description?: string | null;
}

export interface CreateTaskInput {
  title: string;
  description?: string | null;
  scheduled_on?: IsoDate | null;
}

export interface CreateReviewInput {
  content: string;
  period_start?: IsoDate | null;
  period_end?: IsoDate | null;
}
