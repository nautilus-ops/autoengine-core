use serde::Serialize;

pub const WORKFLOW_EVENT: &str = "workflow";

#[derive(Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStatus {
    Running,
    Paused,
    Finished,
    Cancelled,
}

#[derive(Serialize, Clone)]
pub struct WorkflowEventPayload {
    pub status: WorkflowStatus,
}
