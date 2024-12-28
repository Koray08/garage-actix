use actix_web::{http, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use actix_cors::Cors;

#[derive(Serialize, Deserialize, Clone)]
struct Garage {
    id: String,
    name: String,
    location: String,
    city: String,
    capacity: i64,
}

#[derive(Deserialize)]
struct CreateGarageRequest {
    name: String,
    location: String,
    city: String,
    capacity: i64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Car {
    id: String,
    make: String,
    model: String,
    production_year: i64,
    license_plate: String,
    garage_ids: Vec<String>,
}

#[derive(Deserialize)]
struct CreateCarRequest {
    make: String,
    model: String,
    production_year: i64,
    license_plate: String,
}

struct AppState {
    pool: SqlitePool,
}

async fn get_all_garages(data: web::Data<AppState>) -> impl Responder {
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
                    id: row.id.unwrap(),
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

async fn create_garage(
    data: web::Data<AppState>,
    garage_req: web::Json<CreateGarageRequest>,
) -> impl Responder {
    let new_id = Uuid::new_v4().to_string();
    let result = sqlx::query!(
        "INSERT INTO garages (id, name, location, city, capacity) VALUES (?, ?, ?, ?, ?)",
        new_id,
        garage_req.name,
        garage_req.location,
        garage_req.city,
        garage_req.capacity
    )
    .execute(&data.pool)
    .await;

    match result {
        Ok(_) => {
            let garage = Garage {
                id: new_id,
                name: garage_req.name.clone(),
                location: garage_req.location.clone(),
                city: garage_req.city.clone(),
                capacity: garage_req.capacity,
            };
            HttpResponse::Ok().json(garage)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to create garage"),
    }
}

async fn get_all_cars(data: web::Data<AppState>) -> impl Responder {
    let cars = sqlx::query!(
        "SELECT id, make, model, production_year, license_plate FROM cars"
    )
    .fetch_all(&data.pool)
    .await;

    match cars {
        Ok(rows) => {
            let cars: Vec<Car> = rows
                .into_iter()
                .map(|row| Car {
                    id: row.id.unwrap(),
                    make: row.make,
                    model: row.model,
                    production_year: row.production_year,
                    license_plate: row.license_plate,
                    garage_ids: vec![],
                })
                .collect();

            HttpResponse::Ok().json(cars)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch cars"),
    }
}

async fn create_car(
    data: web::Data<AppState>,
    car_req: web::Json<CreateCarRequest>,
) -> impl Responder {
    let new_id = Uuid::new_v4().to_string();
    let result = sqlx::query!(
        "INSERT INTO cars (id, make, model, production_year, license_plate) VALUES (?, ?, ?, ?, ?)",
        new_id,
        car_req.make,
        car_req.model,
        car_req.production_year,
        car_req.license_plate
    )
    .execute(&data.pool)
    .await;

    match result {
        Ok(_) => {
            let car = Car {
                id: new_id,
                make: car_req.make.clone(),
                model: car_req.model.clone(),
                production_year: car_req.production_year,
                license_plate: car_req.license_plate.clone(),
                garage_ids: vec![],
            };
            HttpResponse::Ok().json(car)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to create car"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = "sqlite:data/car_management.db";
    
    let pool = SqlitePool::connect(database_url)
        .await
        .expect("Failed to create pool.");

    sqlx::migrate!().run(&pool).await.expect("Failed to run migrations");

    let app_data = web::Data::new(AppState { pool });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) 
                    .allowed_headers(vec![http::header::CONTENT_TYPE]) 
                    .max_age(3600),
            )
            .route("/garages", web::get().to(get_all_garages))
            .route("/garages", web::post().to(create_garage))
            .route("/cars", web::get().to(get_all_cars))
            .route("/cars", web::post().to(create_car))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}