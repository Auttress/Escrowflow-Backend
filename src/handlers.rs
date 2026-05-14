use crate::models::{CreateMilestone, CreateProject, DisputeRequest, EscrowPaymentRequest, HealthResponse, Milestone, Project, StandardResponse};
use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use uuid::Uuid;

use crate::AppState;

pub async fn health() -> Json<StandardResponse<HealthResponse>> {
    let response = HealthResponse {
        status: "ok".to_string(),
        timestamp: Utc::now(),
    };

    Json(StandardResponse {
        success: true,
        data: response,
    })
}

pub async fn list_projects(State(_state): State<AppState>) -> Json<StandardResponse<Vec<Project>>> {
    let projects = vec![Project {
        id: Uuid::new_v4(),
        title: "Smart contract escrow integration".to_string(),
        description: "Create milestone-backed escrow for freelance agreements.".to_string(),
        client_id: Uuid::new_v4(),
        freelancer_id: None,
        status: "draft".to_string(),
    }];

    Json(StandardResponse {
        success: true,
        data: projects,
    })
}

pub async fn create_project(
    State(_state): State<AppState>,
    Json(payload): Json<CreateProject>,
) -> (StatusCode, Json<StandardResponse<Project>>) {
    let project = Project {
        id: Uuid::new_v4(),
        title: payload.title,
        description: payload.description,
        client_id: payload.client_id,
        freelancer_id: None,
        status: "pending".to_string(),
    };

    (
        StatusCode::CREATED,
        Json(StandardResponse {
            success: true,
            data: project,
        }),
    )
}

pub async fn list_milestones(State(_state): State<AppState>) -> Json<StandardResponse<Vec<Milestone>>> {
    let milestones = vec![Milestone {
        id: Uuid::new_v4(),
        project_id: Uuid::new_v4(),
        title: "Initial deposit".to_string(),
        amount_usd: 750.0,
        due_date: Utc::now(),
        status: "locked".to_string(),
    }];

    Json(StandardResponse {
        success: true,
        data: milestones,
    })
}

pub async fn create_milestone(
    State(_state): State<AppState>,
    Json(payload): Json<CreateMilestone>,
) -> (StatusCode, Json<StandardResponse<Milestone>>) {
    let milestone = Milestone {
        id: Uuid::new_v4(),
        project_id: payload.project_id,
        title: payload.title,
        amount_usd: payload.amount_usd,
        due_date: payload.due_date,
        status: "scheduled".to_string(),
    };

    (
        StatusCode::CREATED,
        Json(StandardResponse {
            success: true,
            data: milestone,
        }),
    )
}

pub async fn create_escrow_payment(
    State(_state): State<AppState>,
    Json(payload): Json<EscrowPaymentRequest>,
) -> (StatusCode, Json<StandardResponse<String>>) {
    let message = format!(
        "Escrow payment initiated for project {} and milestone {}",
        payload.project_id, payload.milestone_id
    );

    (
        StatusCode::ACCEPTED,
        Json(StandardResponse {
            success: true,
            data: message,
        }),
    )
}

pub async fn create_dispute(
    State(_state): State<AppState>,
    Json(payload): Json<DisputeRequest>,
) -> (StatusCode, Json<StandardResponse<String>>) {
    let message = format!(
        "Dispute opened for project {} by {}",
        payload.project_id, payload.opened_by
    );

    (
        StatusCode::CREATED,
        Json(StandardResponse {
            success: true,
            data: message,
        }),
    )
}
