// reoutes/mod.rs

pub mod members;
pub mod capsuleers;
pub mod skills;

use rocket::State;
use rocket::get;
use rocket::serde::json::Json;
use manager::database::DatabaseConnection;

use crate::error::ErrorResponder;

#[get("/health-check")]
pub async fn health_check(db: &State<DatabaseConnection>) -> Result<Json<&str>, ErrorResponder> {
    db.ping().await.map_err(|e| ErrorResponder::from(e))?;
    Ok(Json("OK"))
}

