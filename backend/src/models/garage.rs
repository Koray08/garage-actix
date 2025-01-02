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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GarageReportQueryParams {
    #[serde(alias = "garageId")]
    pub garage_id: i64,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GarageDailyAvailabilityReportDTO {
    pub date: String,
    pub requests: i32,
    pub available_capacity: i32,
}