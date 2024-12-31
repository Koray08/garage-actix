use crate::app_state::AppState; 
use crate::models::maintenance::{CreateMaintenanceDTO, ResponseMaintenanceDTO, Maintenance};
use actix_web::{web, HttpResponse, Responder}; 
use serde_json::json;
use std::collections::HashMap;
use log::{error, info};

pub async fn get_all_maintenances(
    data: web::Data<AppState>,
) -> impl Responder {
    match sqlx::query_as!(
        ResponseMaintenanceDTO,
        r#"
        SELECT
            maintenance.id,
            maintenance.car_id AS "car_id!",
            maintenance.garage_id AS "garage_id!",
            cars.make || ' ' || cars.model AS car_name,
            garages.name AS garage_name,
            maintenance.service_type,
            maintenance.scheduled_date
        FROM maintenance
        JOIN cars ON maintenance.car_id = cars.id
        JOIN garages ON maintenance.garage_id = garages.id
        "#
    )
    .fetch_all(&data.pool)
    .await
    {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(err) => {
            error!("Failed to fetch maintenances: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch maintenances",
                "details": err.to_string()
            }))
        }
    }
}


pub async fn create_maintenance(
    data: web::Data<AppState>,
    maintenance_req: web::Json<CreateMaintenanceDTO>,
) -> impl Responder {
    match sqlx::query!(
        r#"
        INSERT INTO maintenance (car_id, garage_id, service_type, scheduled_date)
        VALUES (?, ?, ?, ?)
        "#,
        maintenance_req.car_id,
        maintenance_req.garage_id,
        maintenance_req.service_type,
        maintenance_req.scheduled_date,
    )
    .execute(&data.pool)
    .await
    {
        Ok(result) => {
            let id = result.last_insert_rowid();
            HttpResponse::Created().json(ResponseMaintenanceDTO {
                id,
                car_id: maintenance_req.car_id.clone(),
                garage_id: maintenance_req.garage_id.clone(),
                car_name: "Car Name Placeholder".to_string(),
                garage_name: "Garage Name Placeholder".to_string(),
                service_type: maintenance_req.service_type.clone(),
                scheduled_date: maintenance_req.scheduled_date.clone(),
            })
        }
        Err(err) => {
            error!("Failed to create maintenance: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to create maintenance",
                "details": err.to_string()
            }))
        }
    }
}
