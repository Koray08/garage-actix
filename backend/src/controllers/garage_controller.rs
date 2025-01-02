use crate::{app_state::AppState, models::garage::{CreateGarageRequest, Garage, GarageReportQueryParams, GarageDailyAvailabilityReportDTO }};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{query, query_as};

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




pub async fn get_garage_report(
    data: web::Data<AppState>,
    query_params: web::Query<GarageReportQueryParams>,
) -> impl Responder {
    log::debug!("Received request parameters: {:?}", query_params);
    
    let garage_id = query_params.garage_id;
    log::debug!("Processing request for garage_id: {}", garage_id);

    log::debug!("Querying garage with id orspu: {}", garage_id);

    let garage = query!(
        r#"
        SELECT id, capacity 
        FROM garages 
        WHERE id = ?1
        "#,
        garage_id
    )
    .fetch_optional(&data.pool)
    .await;

    let garage = match garage {
        Ok(Some(garage)) => {
            log::debug!("Found garage with id {}: capacity {}", garage.id, garage.capacity);
            garage
        }
        Ok(None) => {
            log::error!("No garage found with id {}", garage_id);
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Garage not found",
                "details": format!("No garage found with id {}", garage_id)
            }));
        }
        Err(err) => {
            log::error!("Database error while fetching garage {}: {:?}", garage_id, err);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error",
                "details": err.to_string()
            }));
        }
    };

    let start_date = &query_params.start_date;
    let end_date = &query_params.end_date;
    
    log::debug!("Fetching availability report for garage {} between {} and {}", 
        garage_id, start_date, end_date);

    let result = query_as!(
        GarageDailyAvailabilityReportDTO,
        r#"
        WITH RECURSIVE dates(date) AS (
            SELECT date(?1) as date
            UNION ALL
            SELECT date(date, '+1 day')
            FROM dates
            WHERE date < date(?2)
        ),
        daily_counts AS (
            SELECT 
                date(scheduled_date) as scheduled_date,
                COUNT(*) as request_count
            FROM maintenance
            WHERE garage_id = ?3
            AND date(scheduled_date) BETWEEN date(?1) AND date(?2)
            GROUP BY date(scheduled_date)
        )
        SELECT 
            dates.date as "date!: String",
            CAST(COALESCE(daily_counts.request_count, 0) as INTEGER) as "requests!: i32",
            CAST(
                ?4 - COALESCE(daily_counts.request_count, 0)
                as INTEGER
            ) as "available_capacity!: i32"
        FROM dates
        LEFT JOIN daily_counts ON dates.date = daily_counts.scheduled_date
        ORDER BY dates.date
        "#,
        start_date,
        end_date,
        garage_id,
        garage.capacity
    )
    .fetch_all(&data.pool)
    .await;

    match result {
        Ok(records) => {
            log::debug!("Successfully generated report with {} records", records.len());
            HttpResponse::Ok().json(records)
        }
        Err(err) => {
            log::error!("Failed to generate availability report: {:?}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch daily availability report",
                "details": err.to_string()
            }))
        }
    }
}

