use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub client_id: Uuid,
    pub freelancer_id: Option<Uuid>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProject {
    pub title: String,
    pub description: String,
    pub client_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct Milestone {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub amount_usd: f64,
    pub due_date: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateMilestone {
    pub project_id: Uuid,
    pub title: String,
    pub amount_usd: f64,
    pub due_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct EscrowPaymentRequest {
    pub project_id: Uuid,
    pub milestone_id: Uuid,
    pub amount_usd: f64,
    pub payer_id: Uuid,
    pub payee_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct DisputeRequest {
    pub project_id: Uuid,
    pub milestone_id: Uuid,
    pub opened_by: Uuid,
    pub reason: String,
}

#[derive(Debug, Serialize)]
pub struct StandardResponse<T> {
    pub success: bool,
    pub data: T,
}
