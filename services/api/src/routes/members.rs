// src/routes/members.rs

use rocket::serde::json::Json;
use sea_orm::*;
use rocket::*;
use rocket::http::Status;
use rocket::serde::Deserialize;

use crate::error::*;
use manager::entities::{prelude::*, *};

#[get("/members")]
async fn members(db: &State<DatabaseConnection>) -> Result<Json<Vec<String>>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let member_names = member::Entity::find()
        .all(db)
        .await
        .map(|members| members.into_iter().map(|member| member.name).collect::<Vec<String>>())
        .map_err(|e| ErrorResponder::from(e))?;

    Ok(Json(member_names))
}

#[derive(Debug, Deserialize)]
pub struct MemberRequest {
    pub name: String,
}

#[put("/members", data = "<member_data>")]
async fn create_member(member_data: Json<MemberRequest>, db: &State<DatabaseConnection>) -> Result<Status, ErrorResponder> {
    let db = db as &DatabaseConnection;

    if let Some(_) = Member::find_by_name(&member_data.name, &db)
        .await
        .unwrap() {
        return Err(ErrorResponder::new(Status::Conflict, "Member already registered"));
    }

    let member_data = member_data.into_inner();
    let member = member::ActiveModel {
        name: ActiveValue::Set(member_data.name),
        ..Default::default()
    };

    Member::insert(member)
        .exec(db)
        .await
        .map_err(|e| ErrorResponder::from(e))?;
    
    Ok(Status::Created)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![members, create_member]
}


// fn hash_password(password: &str, config: &State<Configuration>) -> String {
//     let config = config as &Configuration;
//     hash(password, config.rounds).expect("Failed to hash user password.")
// }