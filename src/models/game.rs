use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::Utc;


#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct GameModel {
     pub id: Uuid,
     pub field_name: String,
     pub address: String,
     pub day: chrono::DateTime<Utc>,
     pub created_at: Option<chrono::DateTime<Utc>>,
     pub updated_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGameSchema {
     pub field_name: String,
     pub address: String,
     pub day: chrono::DateTime<Utc>,   
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGameSchema {
     pub field_name: String,
     pub address: String,
     pub day: chrono::DateTime<Utc>,   
}