use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Car {
    pub id: Option<i64>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub production_year: Option<i64>,
    pub license_plate: Option<String>,
    pub garage_ids: Option<Value>,
    pub garages: Option<Value>, 
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateCarRequest {
    pub make: String,
    pub model: String,
    pub production_year: i64,
    pub license_plate: String,
    pub garage_ids: Option<Vec<String>>, 
}

