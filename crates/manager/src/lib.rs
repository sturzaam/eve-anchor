// src/lib.rs

pub mod database;
pub mod entities;
pub mod environment;
pub mod migrator;


use sea_orm::*;
use entities::*;
use entities::prelude::*;
use migrator::sea_orm::InsertResult;



pub async fn new_alliance(
    db: &DatabaseConnection,
    name: &str
) -> Result<InsertResult<alliance::ActiveModel>, DbErr> {
    let alliance = alliance::ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        ..Default::default()
    };
    Alliance::insert(alliance).exec(db).await
}

pub async fn new_corporation(
    db: &DatabaseConnection,
    name: &str,
    alliance_id: i32
) -> Result<InsertResult<corporation::ActiveModel>, DbErr> {
    let corporation = corporation::ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        alliance_id: ActiveValue::Set(alliance_id),
        ..Default::default()
    };
    Corporation::insert(corporation).exec(db).await
}

pub async fn new_member(
    db: &DatabaseConnection,
    name: &str,
    corporation_id: i32
) -> Result<InsertResult<member::ActiveModel>, DbErr> {
    let member = member::ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        corporation_id: ActiveValue::Set(corporation_id),
        ..Default::default()
    };
    Member::insert(member).exec(db).await
}

pub async fn new_capsuleer(
    db: &DatabaseConnection,
    name: &str,
    member_id: i32,
    corporation_id: i32
) -> Result<InsertResult<capsuleer::ActiveModel>, DbErr> {
    let capsuleer = capsuleer::ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        member_id: ActiveValue::Set(member_id),
        corporation_id: ActiveValue::Set(corporation_id),
        ..Default::default()
    };
    Capsuleer::insert(capsuleer).exec(db).await
}

pub async fn new_skill(
    db: &DatabaseConnection,
    name: &str,
    basic: i32,
    advanced: i32,
    expert: i32,
    capsuleer_id: i32
) -> Result<InsertResult<skill::ActiveModel>, DbErr> {
    let skill = skill::ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        basic: ActiveValue::Set(basic),
        advanced: ActiveValue::Set(advanced),
        expert: ActiveValue::Set(expert),
        capsuleer_id: ActiveValue::Set(capsuleer_id),
        ..Default::default()
    };
    Skill::insert(skill).exec(db).await
}