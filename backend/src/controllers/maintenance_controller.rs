use crate::app_state::AppState; 
use crate::models::maintenance::{CreateMaintenanceDTO, ResponseMaintenanceDTO, Maintenance};
use actix_web::{web, HttpResponse, Responder}; 
use crate::models::maintenance::{UpdateMaintenanceDTO};
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


pub async fn edit_maintenance(
    id: web::Path<String>,
    maintenance_req: web::Json<UpdateMaintenanceDTO>, 
    data: web::Data<AppState>,
) -> impl Responder {
    info!(
        "Received request to update maintenance with ID {}: {:?}",
        id, maintenance_req
    );

    let maintenance_id = id.as_str();

    let mut transaction = match data.pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            error!("Failed to start transaction: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "error": "Failed to start transaction",
                "details": err.to_string()
            }));
        }
    };

    let car_id = maintenance_req.car_id.as_deref();
    let garage_id = maintenance_req.garage_id.as_str();
    let service_type = maintenance_req.service_type.as_deref();
    let scheduled_date = maintenance_req.scheduled_date.as_deref();

    if let Err(err) = sqlx::query!(
        r#"
        UPDATE maintenance
        SET 
            car_id = COALESCE(?, car_id), 
            garage_id = COALESCE(?, garage_id), 
            service_type = COALESCE(?, service_type), 
            scheduled_date = COALESCE(?, scheduled_date)
        WHERE id = ?
        "#,
        car_id,
        garage_id,
        service_type,
        scheduled_date,
        maintenance_id
    )
    .execute(&mut *transaction)
    .await
    {
        error!("Failed to update maintenance: {:?}", err);
        let _ = transaction.rollback().await;
        return HttpResponse::InternalServerError().json(json!({
            "error": "Failed to update maintenance",
            "details": err.to_string()
        }));
    }

    if let Err(err) = transaction.commit().await {
        error!("Failed to commit transaction: {:?}", err);
        return HttpResponse::InternalServerError().json(json!({
            "error": "Failed to finalize update",
            "details": err.to_string()
        }));
    }

    HttpResponse::Ok().json(json!({
        "id": maintenance_id,
        "updated": true,
    }))
}

pub async fn delete_maintenance(
    id: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Received request to delete maintenance with ID {}", id);

    let maintenance_id = id.as_str();

    match sqlx::query!(
        r#"
        DELETE FROM maintenance
        WHERE id = ?
        "#,
        maintenance_id
    )
    .execute(&data.pool)
    .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                error!("Maintenance with ID {} not found", maintenance_id);
                HttpResponse::NotFound().json(json!({
                    "error": "Maintenance not found",
                }))
            } else {
                HttpResponse::Ok().json(json!({
                    "id": maintenance_id,
                    "deleted": true,
                }))
            }
        }
        Err(err) => {
            error!("Failed to delete maintenance: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to delete maintenance",
                "details": err.to_string(),
            }))
        }
    }
}
