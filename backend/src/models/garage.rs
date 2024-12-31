use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Garage {
    pub id: i64,
    pub name: String,
    pub location: String,
    pub city: String,
    pub capacity: i64,
}

#[derive(Deserialize)]
pub struct CreateGarageRequest {
    pub name: String,
    pub location: String,
    pub city: String,
    pub capacity: i64,
}
