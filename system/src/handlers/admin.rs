use crate::auth::{require_admin, AuthUser};
use crate::models::{
    FloorForm, FloorOption, FullPropertyView, House, HouseForm, MeterForm, Tenant, TenantForm,
};
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Redirect},
    Form,
};
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "admin_dashboard.html")]
pub struct DashboardTemplate {
    pub houses: Vec<House>,
    pub floor_options: Vec<FloorOption>,
    pub tenants: Vec<Tenant>,
    pub master_table: Vec<FullPropertyView>,
}

pub async fn dashboard(
    user: AuthUser,
    State(pool): State<PgPool>,
) -> Result<Html<String>, StatusCode> {
    require_admin(&user)?;

    let houses: Vec<House> = sqlx::query_as("SELECT id, house_number, address FROM houses ORDER BY house_number")
        .fetch_all(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let floor_options: Vec<FloorOption> = sqlx::query_as(
        "SELECT f.id, h.house_number, f.floor_no 
         FROM floors f 
         JOIN houses h ON f.house_id = h.id 
         ORDER BY h.house_number, f.floor_no"
    ).fetch_all(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tenants: Vec<Tenant> = sqlx::query_as(
        "SELECT id, floor_id, full_name, phone FROM tenants ORDER BY full_name"
    ).fetch_all(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // THE MASTER QUERY: Joins everything from Houses down to Meters
    let master_table: Vec<FullPropertyView> = sqlx::query_as(
        "SELECT 
            h.house_number, 
            h.address, 
            f.floor_no, 
            f.capacity,
            t.full_name as tenant_name, 
            t.phone, 
            m.meter_number
         FROM houses h
         LEFT JOIN floors f ON h.id = f.house_id
         LEFT JOIN tenants t ON f.id = t.floor_id
         LEFT JOIN water_meters m ON t.id = m.tenant_id
         ORDER BY h.house_number, f.floor_no, t.full_name"
    ).fetch_all(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tpl = DashboardTemplate { houses, floor_options, tenants, master_table };
    Ok(Html(tpl.render().unwrap()))
}

pub async fn add_house(
    user: AuthUser, State(pool): State<PgPool>, Form(form): Form<HouseForm>,
) -> Result<Redirect, StatusCode> {
    require_admin(&user)?;
    sqlx::query("INSERT INTO houses (house_number, address) VALUES ($1, $2)")
        .bind(&form.house_number).bind(&form.address).execute(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/admin"))
}

pub async fn add_floor(
    user: AuthUser, State(pool): State<PgPool>, Form(form): Form<FloorForm>,
) -> Result<Redirect, StatusCode> {
    require_admin(&user)?;
    let result = sqlx::query("INSERT INTO floors (house_id, floor_no, capacity) VALUES ($1, $2, $3)")
        .bind(form.house_id).bind(form.floor_no).bind(form.capacity).execute(&pool).await;
    if let Err(e) = result {
        tracing::error!("Failed to add floor: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(Redirect::to("/admin"))
}

pub async fn add_tenant(
    user: AuthUser, State(pool): State<PgPool>, Form(form): Form<TenantForm>,
) -> Result<Redirect, StatusCode> {
    require_admin(&user)?;
    sqlx::query("INSERT INTO tenants (floor_id, full_name, phone) VALUES ($1, $2, $3)")
        .bind(form.floor_id).bind(&form.full_name).bind(&form.phone).execute(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/admin"))
}

pub async fn add_meter(
    user: AuthUser, State(pool): State<PgPool>, Form(form): Form<MeterForm>,
) -> Result<Redirect, StatusCode> {
    require_admin(&user)?;
    sqlx::query("INSERT INTO water_meters (meter_number, tenant_id) VALUES ($1, $2)")
        .bind(&form.meter_number).bind(form.tenant_id).execute(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/admin"))
}
