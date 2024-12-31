use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")] 
pub struct CreateMaintenanceDTO {
    pub car_id: String, 
    pub garage_id: String,
    pub service_type: String,
    pub scheduled_date: String,
}

#[derive(Deserialize, Serialize, Debug)] 
#[serde(rename_all = "camelCase")]
pub struct UpdateMaintenanceDTO {
    pub car_id: Option<String>, 
    pub garage_id: String,
    pub service_type: Option<String>,
    pub scheduled_date: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMaintenanceDTO {
    pub id: i64,
    pub car_id: String,
    pub car_name: String,
    pub service_type: String,
    pub scheduled_date: String,
    pub garage_id: String,
    pub garage_name: String,
}

#[derive(Deserialize, Serialize, Debug)] 
#[serde(rename_all = "camelCase")]
pub struct Maintenance {
    pub id: i64,
    pub car_id: String,
    pub garage_id: String,
    pub service_type: String,
    pub scheduled_date: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EditMaintenanceDTO {
    pub id: String,
    pub car_id: String,
    pub garage_id: String,
    pub service_type: String,
    pub scheduled_date: String,
}