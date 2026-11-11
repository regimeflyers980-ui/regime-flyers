# TODO - REGIME FLYERS

## Critical (Pre-Production)
- [ ] Add SSL (Let's Encrypt) so the site runs on HTTPS.
- [ ] Lock down CORS in the Rust backend to specific domains.
- [ ] Set up a database (PostgreSQL) to replace hardcoded demo data.

## High Priority
- [ ] Build the actual demo pages (currently the buttons point to 404s).
- [ ] Add a 404 Not Found page to the Next.js frontend.

## Medium Priority
- [ ] Add rate limiting to the Rust API using `tower-governor`.
- [ ] Create a CI/CD pipeline using GitHub Actions.
