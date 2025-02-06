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

pub async fn new_problem(
    db: &DatabaseConnection,
    name: &str,
    constraint: Vec<u8>,
    member_id: i32,
    corporation_id: i32,
    alliance_id: Option<i32>
) -> Result<InsertResult<problem::ActiveModel>, DbErr> {
    let problem = problem::ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        constraint: ActiveValue::Set(constraint),
        member_id: ActiveValue::Set(member_id),
        corporation_id: ActiveValue::Set(corporation_id),
        alliance_id: ActiveValue::Set(alliance_id),
        ..Default::default()
    };
    Problem::insert(problem).exec(db).await
}

pub async fn new_outpost(
    db: &DatabaseConnection,
    name: &str,
    system: &str,
    planets: i32,
    arrays: i32,
    capsuleer_id: i32,
    problem_id: Option<i32>
) -> Result<InsertResult<outpost::ActiveModel>, DbErr> {
    let outpost = outpost::ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        system: ActiveValue::Set(system.to_owned()),
        planets: ActiveValue::Set(planets),
        arrays: ActiveValue::Set(arrays),
        capsuleer_id: ActiveValue::Set(capsuleer_id),
        problem_id: ActiveValue::Set(problem_id),
        ..Default::default()
    };
    Outpost::insert(outpost).exec(db).await
}