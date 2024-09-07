
use sea_orm::entity::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "skill")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub basic: i32,
    pub advanced: i32,
    pub expert: i32,
    pub capsuleer_id: i32,
}

impl Model {

    pub fn reset(&mut self) {
        self.basic = 0;
        self.advanced = 0;
        self.expert = 0;
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
}

impl Related<super::capsuleer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Capsuleer.def()
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
}