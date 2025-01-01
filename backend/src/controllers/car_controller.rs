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

    match sqlx::query!(
        r#"
        INSERT INTO cars (make, model, production_year, license_plate)
        VALUES (?, ?, ?, ?)
        "#,
        car_req.make,
        car_req.model,
        car_req.production_year,
        car_req.license_plate
    )
    .execute(&data.pool)
    .await
    {
        Ok(result) => {
            let car_id = result.last_insert_rowid(); 

            let mut garage_details: Vec<serde_json::Value> = Vec::new();

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
                    } else {
                        if let Ok(garage) = sqlx::query!(
                            r#"
                            SELECT id, name, location, city, capacity
                            FROM garages
                            WHERE id = ?
                            "#,
                            garage_id
                        )
                        .fetch_one(&data.pool)
                        .await
                        {
                            garage_details.push(json!({
                                "id": garage.id,
                                "name": garage.name,
                                "location": garage.location,
                                "city": garage.city,
                                "capacity": garage.capacity,
                            }));
                        }
                    }
                }
            }

            HttpResponse::Created().json(Car {
                id: Some(car_id),
                make: Some(car_req.make.clone()),
                model: Some(car_req.model.clone()),
                production_year: Some(car_req.production_year),
                license_plate: Some(car_req.license_plate.clone()),
                garage_ids: car_req
                    .garage_ids
                    .as_ref()
                    .map(|ids| serde_json::to_value(ids).unwrap_or_default()),
                garages: Some(serde_json::Value::Array(garage_details)),
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
    info!("Starting get_all_cars request");

    let cars_with_garages = sqlx::query!(
        r#"
        SELECT
            cars.id,
            cars.make,
            cars.model,
            cars.production_year,
            cars.license_plate,
            COALESCE(json_group_array(car_garages.garage_id), '[]') as garage_ids
        FROM cars
        LEFT JOIN car_garages ON cars.id = car_garages.car_id
        GROUP BY cars.id
        "#
    )
    .fetch_all(&data.pool)
    .await;

    match cars_with_garages {
        Ok(rows) => {
            let mut cars: Vec<Car> = Vec::new();

            for row in rows {
                let garages = sqlx::query!(
                    r#"
                    SELECT
                        garages.id,
                        garages.name,
                        garages.location,
                        garages.city,
                        garages.capacity
                    FROM garages
                    JOIN car_garages ON garages.id = car_garages.garage_id
                    WHERE car_garages.car_id = ?
                    "#,
                    row.id
                )
                .fetch_all(&data.pool)
                .await
                .unwrap_or_else(|_| Vec::new());

                let garage_details: Vec<serde_json::Value> = garages
                    .into_iter()
                    .map(|garage| {
                        json!({
                            "id": garage.id,
                            "name": garage.name,
                            "location": garage.location,
                            "city": garage.city,
                            "capacity": garage.capacity,
                        })
                    })
                    .collect();

                cars.push(Car {
                    id: Some(row.id),
                    make: Some(row.make),
                    model: Some(row.model),
                    production_year: Some(row.production_year),
                    license_plate: Some(row.license_plate),
                    garage_ids: Some(serde_json::Value::Array(
                        serde_json::from_str(&row.garage_ids).unwrap_or_default(),
                    )),
                    garages: Some(serde_json::Value::Array(garage_details)),
                });
            }

            HttpResponse::Ok().json(cars)
        }
        Err(err) => {
            error!("Database error: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch cars",
                "details": err.to_string()
            }))
        }
    }
}

pub async fn delete_car(
    id: web::Path<i64>, 
    data: web::Data<AppState>,
) -> impl Responder {
    match sqlx::query!(
        r#"
        DELETE FROM cars
        WHERE id = ?
        "#,
        *id
    )
    .execute(&data.pool)
    .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().finish()
            } else {
                HttpResponse::Ok().json(true)
            }
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to delete car",
                "details": err.to_string()
            }))
        }
    }
}

pub async fn edit_car(
    id: web::Path<String>,
    car_req: web::Json<CreateCarRequest>, 
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Received request to update car with ID {}: {:?}", id, car_req);

    let car_id = id.as_str(); 

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

    if let Err(err) = sqlx::query!(
        r#"
        UPDATE cars
        SET make = ?, model = ?, production_year = ?, license_plate = ?
        WHERE id = ?
        "#,
        car_req.make,
        car_req.model,
        car_req.production_year,
        car_req.license_plate,
        car_id 
    )
    .execute(&mut *transaction)
    .await
    {
        error!("Failed to update car: {:?}", err);
        let _ = transaction.rollback().await;
        return HttpResponse::InternalServerError().json(json!({
            "error": "Failed to update car",
            "details": err.to_string()
        }));
    }

    if let Err(err) = sqlx::query!(
        r#"
        DELETE FROM car_garages
        WHERE car_id = ?
        "#,
        car_id 
    )
    .execute(&mut *transaction)
    .await
    {
        error!("Failed to clear garage associations: {:?}", err);
        let _ = transaction.rollback().await;
        return HttpResponse::InternalServerError().json(json!({
            "error": "Failed to update car",
            "details": err.to_string()
        }));
    }

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
            .execute(&mut *transaction)
            .await
            {
                error!("Failed to associate car with garage: {:?}", err);
                let _ = transaction.rollback().await;
                return HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to update car",
                    "details": err.to_string()
                }));
            }
        }
    }

    if let Err(err) = transaction.commit().await {
        error!("Failed to commit transaction: {:?}", err);
        return HttpResponse::InternalServerError().json(json!({
            "error": "Failed to finalize update",
            "details": err.to_string()
        }));
    }

    HttpResponse::Ok().json(json!({
        "id": car_id,
        "make": car_req.make,
        "model": car_req.model,
        "productionYear": car_req.production_year,
        "licensePlate": car_req.license_plate,
        "garageIds": car_req.garage_ids
    }))
}
