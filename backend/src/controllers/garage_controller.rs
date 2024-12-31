use crate::{app_state::AppState, models::garage::{CreateGarageRequest, Garage}};
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use serde::Deserialize;

pub async fn get_all_garages(data: web::Data<AppState>) -> impl Responder {
    let garages = sqlx::query!(
        "SELECT id, name, location, city, capacity FROM garages"
    )
    .fetch_all(&data.pool)
    .await;

    match garages {
        Ok(rows) => {
            let garages: Vec<Garage> = rows
                .into_iter()
                .map(|row| Garage {
                    id: row.id,
                    name: row.name,
                    location: row.location,
                    city: row.city,
                    capacity: row.capacity,
                })
                .collect();

            HttpResponse::Ok().json(garages)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch garages"),
    }
}

pub async fn create_garage(
    data: web::Data<AppState>,
    garage_req: web::Json<CreateGarageRequest>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO garages (name, location, city, capacity) VALUES (?, ?, ?, ?)",
        garage_req.name,
        garage_req.location,
        garage_req.city,
        garage_req.capacity
    )
    .execute(&data.pool)
    .await;

    match result {
        Ok(query_result) => {
            let new_id = query_result.last_insert_rowid();

            let garage = Garage {
                id: new_id,
                name: garage_req.name.clone(),
                location: garage_req.location.clone(),
                city: garage_req.city.clone(),
                capacity: garage_req.capacity,
            };

            HttpResponse::Ok().json(garage)
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Failed to create garage: {}", err))
        }
    }
}

pub async fn delete_garage(
    data: web::Data<AppState>,
    garage_id: web::Path<String>,
) -> impl Responder {
    let id = garage_id.into_inner(); 
    let result = sqlx::query!(
        "DELETE FROM garages WHERE id = ?",
        id
    )
    .execute(&data.pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Garage deleted successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete garage"),
    }
}

#[derive(Deserialize)]
pub struct EditGarageRequest {
    name: Option<String>,
    location: Option<String>,
    city: Option<String>,
    capacity: Option<i64>,
}

pub async fn edit_garage(
    data: web::Data<AppState>,
    garage_id: web::Path<String>,
    garage_req: web::Json<EditGarageRequest>,
) -> impl Responder {
    let id = garage_id.into_inner(); 
    let result = sqlx::query!(
        "UPDATE garages 
        SET 
            name = COALESCE(?, name),
            location = COALESCE(?, location),
            city = COALESCE(?, city),
            capacity = COALESCE(?, capacity)
        WHERE id = ?",
        garage_req.name,
        garage_req.location,
        garage_req.city,
        garage_req.capacity,
        id
    )
    .execute(&data.pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Garage updated successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update garage"),
    }
}

pub async fn get_single_garage(
    data: web::Data<AppState>,
    garage_id: web::Path<String>,
) -> impl Responder {
    let id = garage_id.into_inner();

    let result = sqlx::query!(
        "SELECT id, name, location, city, capacity FROM garages WHERE id = ?",
        id
    )
    .fetch_one(&data.pool)
    .await;

    match result {
        Ok(row) => {
            let garage = Garage {
                id: row.id,
                name: row.name,
                location: row.location,
                city: row.city,
                capacity: row.capacity,
            };
            HttpResponse::Ok().json(garage)
        }
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("Garage not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch garage"),
    }
}
