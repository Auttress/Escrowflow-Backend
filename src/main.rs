mod config;
mod db;
mod handlers;
mod models;

use axum::{routing::{get, post}, Router};
use config::AppConfig;
use db::init_database;
use std::net::SocketAddr;
use tracing::{error, info};

#[derive(Clone)]
struct AppState {
    db: db::Db,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = AppConfig::from_env()?;
    let db_pool = init_database(&config.database_url).await?;
    let state = AppState { db: db_pool };

    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/projects", get(handlers::list_projects).post(handlers::create_project))
        .route("/milestones", get(handlers::list_milestones).post(handlers::create_milestone))
        .route("/payments/escrow", post(handlers::create_escrow_payment))
        .route("/disputes", post(handlers::create_dispute))
        .with_state(state);

    let address = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!(%address, "EscrowFlow backend starting");

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .map_err(|err| {
            error!(%err, "Server failed");
            err
        })?;

    Ok(())
}
