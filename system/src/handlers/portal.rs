use crate::auth::{AppError, AuthUser};
use crate::models::PortalReading;
use askama::Template;
use axum::{extract::State, http::StatusCode, response::{Html, Redirect}};
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "portal_dashboard.html")]
pub struct PortalTemplate {
    pub tenant_name: String,
    pub readings: Vec<PortalReading>,
}

pub async fn dashboard(
    user: AuthUser,
    State(pool): State<PgPool>,
) -> Result<Html<String>, AppError> {
    // If an admin lands here, kick them back to admin dashboard
    if user.role == "ADMIN" {
        return Err(Redirect::to("/admin").into());
    }

    let tenant_name: String = sqlx::query_scalar("SELECT full_name FROM tenants WHERE user_id = $1")
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let readings: Vec<PortalReading> = sqlx::query_as(
        "SELECT m.meter_number, r.reading_date, r.previous_units::float8, r.current_units::float8, r.units_used::float8
         FROM meter_readings r
         JOIN water_meters m ON r.meter_id = m.id
         JOIN tenants t ON m.tenant_id = t.id
         WHERE t.user_id = $1
         ORDER BY r.reading_date DESC"
    )
    .bind(user.id)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tpl = PortalTemplate { tenant_name, readings };
    Ok(Html(tpl.render().unwrap()))
}
