use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "member")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub active: bool,
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