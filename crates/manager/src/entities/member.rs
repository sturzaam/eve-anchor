use sea_orm::entity::prelude::*;

use crate::capsuleer;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "member")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub corporation_id: i32,
}

impl Model {
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::capsuleer::Entity")]
    Capsuleer,
    #[sea_orm(
        belongs_to = "super::corporation::Entity",
        from = "Column::CorporationId",
        to = "super::corporation::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Corporation,
}

impl Related<super::capsuleer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Capsuleer.def()
    }
}

impl Related<super::corporation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Corporation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub async fn find_by_name(name: &str, db: &DatabaseConnection) -> Result<Option<Model>, sea_orm::DbErr> {
        Entity::find()
            .filter(Column::Name.eq(name))
            .one(db)
            .await
            .map_err(|e| e.into())
    }
    pub async fn find_capsuleer_by_name(name: &str, db: &DatabaseConnection) -> Result<Vec<(Model, Option<capsuleer::Model>)>, sea_orm::DbErr> {
        Entity::find()
            .filter(Column::Name.eq(name))
            .find_also_related(capsuleer::Entity)
            .all(db)
            .await
            .map_err(|e| e.into())
    }
}