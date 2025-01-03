mod controllers;
mod models;
mod app_state;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use app_state::AppState;
use controllers::{
    car_controller::{create_car, get_all_cars, delete_car, edit_car},
    garage_controller::{create_garage, get_all_garages, edit_garage, delete_garage, get_single_garage, get_garage_report},
    maintenance_controller::{create_maintenance, get_all_maintenances, get_maintenance_by_id,  delete_maintenance, edit_maintenance, monthly_requests_report},
};
use sqlx::SqlitePool;
use env_logger::Env;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the .env file
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load DATABASE_URL from the environment or fallback to default
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:data/database.db".to_string());
    
    let pool = SqlitePool::connect(&database_url)
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
                    .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
                    .max_age(3600),
            )
            .route("/garages/dailyAvailabilityReport", web::get().to(get_garage_report))
            .route("/maintenance/monthlyRequestsReport", web::get().to(monthly_requests_report)) 
            .route("/garages", web::get().to(get_all_garages))
            .route("/garages", web::post().to(create_garage))
            .route("/garages/{id}", web::delete().to(delete_garage)) 
            .route("/garages/{id}", web::put().to(edit_garage))
            .route("/garages/{id}", web::get().to(get_single_garage))
            .route("/cars", web::get().to(get_all_cars))
            .route("/cars", web::post().to(create_car))
            .route("/cars/{id}", web::put().to(edit_car))
            .route("/cars/{id}", web::delete().to(delete_car))
            .route("/maintenance", web::get().to(get_all_maintenances))
            .route("/maintenance", web::post().to(create_maintenance)) 
            .route("/maintenance/{id}", web::get().to(get_maintenance_by_id))
            .route("/maintenance/{id}", web::put().to(edit_maintenance))
            .route("/maintenance/{id}", web::delete().to(delete_maintenance)) 
        })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
