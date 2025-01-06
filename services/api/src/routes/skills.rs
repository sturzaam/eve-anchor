// routes/skills.rs

use rocket::serde::json::Json;
use sea_orm::*;
use rocket::*;
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;

use crate::error::*;
use manager::entities::{prelude::*, *};

#[derive(Debug, Deserialize, Serialize)]
pub struct SkillResponse {
    pub id: i32,
    pub name: String,
    pub capsuleer_id: i32,
    pub basic: i32,
    pub advanced: i32,
    pub expert: i32,
}

#[get("/skills")]
async fn skills(db: &State<DatabaseConnection>) -> Result<Json<Vec<SkillResponse>>, ErrorResponder> {
    let db = db as &DatabaseConnection;
    let skills = skill::Entity::find()
        .all(db)
        .await
        .map_err(|e| ErrorResponder::from(e))?;

    let skill_responses: Vec<SkillResponse> = skills.into_iter().map(|skill| SkillResponse {
        id: skill.id,
        name: skill.name,
        capsuleer_id: skill.capsuleer_id,
        basic: skill.basic,
        advanced: skill.advanced,
        expert: skill.expert,
    }).collect();

    Ok(Json(skill_responses))
}

#[derive(Debug, Deserialize)]
pub struct SkillRequest {
    pub name: String,
    pub capsuleer: String,
    pub basic: i32,
    pub advanced: i32,
    pub expert: i32,
}

#[put("/skills", data = "<skill_data>")]
async fn create_skill(skill_data: Json<SkillRequest>, db: &State<DatabaseConnection>) -> Result<Status, ErrorResponder> {
    let db = db as &DatabaseConnection;
    let capsuleer = Capsuleer::find_by_name(&skill_data.capsuleer, &db)
        .await
        .unwrap()
        .unwrap();
    let skill_data = skill_data.into_inner();
    let skill = skill::ActiveModel {
        name: ActiveValue::Set(skill_data.name),
        basic: ActiveValue::Set(skill_data.basic),
        advanced: ActiveValue::Set(skill_data.advanced),
        expert: ActiveValue::Set(skill_data.expert),
        capsuleer_id: ActiveValue::Set(capsuleer.id),
        ..Default::default()
    };

    Skill::insert(skill)
        .exec(db)
        .await
        .map_err(|e| ErrorResponder::from(e))?;
    
    Ok(Status::Created)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![skills, create_skill]
}