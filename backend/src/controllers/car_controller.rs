use crate::app_state::AppState;
use crate::models::car::{Car, CreateCarRequest};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use log::{error, info};

pub async fn create_car(
    data: web::Data<AppState>,
    car_req: web::Json<CreateCarRequest>,
) -> impl Responder {
    info!("Received request to create car: {:?}", car_req);

    let car_id = uuid::Uuid::new_v4().to_string(); // Assign the UUID to a variable

    match sqlx::query!(
        r#"
        INSERT INTO cars (id, make, model, production_year, license_plate)
        VALUES (?, ?, ?, ?, ?)
        "#,
        car_id,
        car_req.make,
        car_req.model,
        car_req.production_year,
        car_req.license_plate
    )
    .execute(&data.pool)
    .await
    {
        Ok(_) => {
            // Handle garage_ids insertion
            if let Some(garage_ids) = &car_req.garage_ids {
                for garage_id in garage_ids {
                    if let Err(err) = sqlx::query!(
                        r#"
                        INSERT INTO car_garages (car_id, garage_id)
                        VALUES (?, ?)
                        "#,
                        car_id,
                        garage_id
                    )
                    .execute(&data.pool)
                    .await
                    {
                        error!("Failed to associate car with garage: {:?}", err);
                    }
                }
            }

            HttpResponse::Created().json(Car {
                id: None,
                make: Some(car_req.make.clone()),
                model: Some(car_req.model.clone()),
                production_year: Some(car_req.production_year),
                license_plate: Some(car_req.license_plate.clone()),
                garage_ids: car_req
                    .garage_ids
                    .as_ref()
                    .map(|ids| serde_json::to_value(ids).unwrap_or_default()),
            })
        }
        Err(err) => {
            error!("Database error creating car: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to create car",
                "details": err.to_string()
            }))
        }
    }
}

pub async fn get_all_cars(data: web::Data<AppState>) -> impl Responder {
    info!("Received request to get all cars");

    let cars_with_garages = sqlx::query!(
        r#"
        SELECT
            cars.id AS car_id,
            cars.make AS make,
            cars.model AS model,
            cars.production_year AS production_year,
            cars.license_plate AS license_plate,
            COALESCE(json_group_array(car_garages.garage_id), '[]') AS garage_ids
        FROM cars
        LEFT JOIN car_garages ON cars.id = car_garages.car_id
        GROUP BY cars.id
        "#
    )
    .fetch_all(&data.pool)
    .await;

    match cars_with_garages {
        Ok(rows) => {
            let cars: Vec<Car> = rows
                .into_iter()
                .map(|row| Car {
                    id: Some(row.car_id), // Use car_id directly; no parsing needed
                    make: Some(row.make),
                    model: Some(row.model),
                    production_year: Some(row.production_year),
                    license_plate: Some(row.license_plate),
                    garage_ids: Some(serde_json::from_str(&row.garage_ids).unwrap_or_default()),
                })
                .collect();
            HttpResponse::Ok().json(cars)
        }
        Err(err) => {
            error!("Error fetching cars: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch cars",
                "details": err.to_string()
            }))
        }
    }
}
