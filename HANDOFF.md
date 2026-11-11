# Project Handoff - REGIME FLYERS

## Current Status
MVP built, containerized, and running via Nginx.

## Architecture
- **Frontend:** Next.js (Port 3000, internal)
- **Backend:** Rust/Axum (Port 3001, internal)
- **Proxy:** Nginx (Port 80, public)
- **Orchestration:** Docker Compose

## How to Run
```bash
cd ~/regime-flyers
docker compose up -d
```
Access the site at http://localhost (Port 80).

## Development Mode
To run locally without Docker:
Terminal 1: `cd backend && cargo run`
Terminal 2: `cd frontend && npm run dev`
