use sea_orm::entity::prelude::*;
use sea_orm::DeleteResult;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "outpost")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub system: String,
    pub planets: i32,
    pub arrays: i32,
    pub capsuleer_id: i32,
    pub problem_id: Option<i32>
}

impl Model {

    pub fn reset(&mut self) {
        self.planets = 0;
        self.arrays = 0;
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::capsuleer::Entity",
        from = "Column::CapsuleerId",
        to = "super::capsuleer::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Capsuleer,
    #[sea_orm(
        belongs_to = "super::problem::Entity",
        from = "Column::ProblemId",
        to = "super::problem::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Problem,
}

impl Related<super::capsuleer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Capsuleer.def()
    }
}


impl Related<super::problem::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Problem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub async fn delete_by_name(name: &str, db: &DatabaseConnection) -> Result<DeleteResult, sea_orm::DbErr> {
        Entity::delete_many()
            .filter(Column::Name.eq(name))
            .exec(db)
            .await
            .map_err(|e| e.into())
    }
    
    pub async fn find_by_name(name: &str, db: &DatabaseConnection) -> Result<Option<Model>, sea_orm::DbErr> {
        Entity::find()
            .filter(Column::Name.eq(name))
            .one(db)
            .await
            .map_err(|e| e.into())
    }

    pub async fn find_by_capsuleer(capsuleer_id: i32, db: &DatabaseConnection) -> Result<Vec<Model>, sea_orm::DbErr> {
        Entity::find()
            .filter(Column::CapsuleerId.eq(capsuleer_id))
            .all(db)
            .await
            .map_err(|e| e.into())
    }
}