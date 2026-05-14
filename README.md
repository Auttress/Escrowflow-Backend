# EscrowFlow Backend

Rust-powered backend infrastructure for EscrowFlow, the freelance escrow platform for milestone payments, dispute resolution, and blockchain settlement.

## Overview

This repository contains the backend API for EscrowFlow. It is designed to support:
- Milestone-based escrow workflows
- Secure dispute management
- Stablecoin-ready payment orchestration
- PostgreSQL persistence and local Docker development
- A modular, testable Rust/Axum architecture

## Architecture

- `Axum` for HTTP routing and JSON APIs
- `SQLx` for PostgreSQL integration
- `Tokio` async runtime
- `Tracing` for observability
- Docker Compose for local database and service orchestration

## Getting Started

### Prerequisites

- Rust toolchain (`rustup`)
- Docker and Docker Compose (for local database setup)

### Local development

1. Copy the example environment file:

```bash
cp .env.example .env
```

2. Start the database and backend service:

```bash
docker compose up --build
```

3. Run the backend directly:

```bash
cargo run
```

4. Visit the health endpoint:

```bash
curl http://127.0.0.1:8080/health
```

## API Endpoints

- `GET /health` — service health check
- `GET /projects` — list active projects
- `POST /projects` — create a project
- `GET /milestones` — list milestone records
- `POST /milestones` — create a milestone
- `POST /payments/escrow` — initiate escrow payment handling
- `POST /disputes` — open a dispute record

## Project Structure

- `src/main.rs` — application entrypoint and router setup
- `src/config.rs` — environment configuration
- `src/db.rs` — PostgreSQL connection management
- `src/handlers.rs` — HTTP request handlers
- `src/models.rs` — shared request and response models
- `migrations/` — database schema migration files

## Next Steps

- Implement persistent project, milestone, and dispute storage
- Add authentication and wallet session management
- Integrate Stellar/Soroban smart contract services
- Add API documentation, tests, and CI pipeline
