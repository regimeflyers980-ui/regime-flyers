# REGIME FLYERS

Secure, high-performance information systems. Architected in Rust, delivered to you.

## Architecture
- **Frontend:** Next.js (React + Tailwind CSS) - internal Port 3000
- **Backend:** Rust (Axum) - internal Port 3001
- **Reverse Proxy:** Nginx - public Port 80
- **Orchestration:** Docker Compose

## Deployment
To build and start the entire stack:
```bash
docker compose up --build -d
```
Access the site at http://localhost.

## Development (Local)
- Backend: `cd backend && cargo run`
- Frontend: `cd frontend && npm run dev`
