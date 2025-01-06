// routes/capsuleers.rs

use rocket::serde::json::Json;
use sea_orm::*;
use rocket::*;
use rocket::http::Status;
use rocket::serde::Deserialize;

use crate::error::*;
use manager::entities::{prelude::*, *};

#[get("/capsuleers")]
async fn capsuleers(db: &State<DatabaseConnection>) -> Result<Json<Vec<String>>, ErrorResponder> {
    let db = db as &DatabaseConnection;
    let capsuleer_names = capsuleer::Entity::find()
        .all(db)
        .await
        .map(|capsuleers| capsuleers.into_iter().map(|capsuleer| capsuleer.name).collect::<Vec<String>>())
        .map_err(|e| ErrorResponder::from(e))?;

    Ok(Json(capsuleer_names))
}

#[derive(Debug, Deserialize)]
pub struct CapsuleerRequest {
    pub name: String,
    pub member: String
}

#[put("/capsuleers", data = "<capsuleer_data>")]
async fn create_capsuleer(capsuleer_data: Json<CapsuleerRequest>, db: &State<DatabaseConnection>) -> Result<Status, ErrorResponder> {
    let db = db as &DatabaseConnection;
    let member = Member::find_by_name(&capsuleer_data.member, &db)
        .await
        .unwrap()
        .unwrap();
    let capsuleer_data = capsuleer_data.into_inner();
    let capsuleer = capsuleer::ActiveModel {
        name: ActiveValue::Set(capsuleer_data.name),
        member_id: ActiveValue::Set(member.id),
        ..Default::default()
    };

    Capsuleer::insert(capsuleer)
        .exec(db)
        .await
        .map_err(|e| ErrorResponder::from(e))?;
    
    Ok(Status::Created)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![capsuleers, create_capsuleer]
}