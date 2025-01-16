use sea_orm::entity::prelude::*;

use crate::outpost;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "problem")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub constraint: Vec<u8>,
    pub active: bool,
    pub member_id: i32,
    pub corporation_id: i32,
    pub alliance_id: Option<i32>
}

impl Model {
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::outpost::Entity")]
    Outpost,
    #[sea_orm(
        belongs_to = "super::member::Entity",
        from = "Column::MemberId",
        to = "super::member::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Member,
    #[sea_orm(
        belongs_to = "super::corporation::Entity",
        from = "Column::CorporationId",
        to = "super::corporation::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Corporation,
    #[sea_orm(
        belongs_to = "super::alliance::Entity",
        from = "Column::AllianceId",
        to = "super::alliance::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Alliance,
}

impl Related<super::outpost::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Outpost.def()
    }
}

impl Related<super::member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Member.def()
    }
}

impl Related<super::corporation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Corporation.def()
    }
}

impl Related<super::alliance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Alliance.def()
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

    pub async fn find_outposts_by_name(name: &str, db: &DatabaseConnection) -> Result<Vec<(Model, Option<outpost::Model>)>, sea_orm::DbErr> {
        Entity::find()
            .filter(Column::Name.eq(name))
            .find_also_related(outpost::Entity)
            .all(db)
            .await
            .map_err(|e| e.into())
    }
}