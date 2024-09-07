use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "capsuleer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub member_id: i32,
}

impl Model {
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::skill::Entity")]
    Skill,
    #[sea_orm(
        belongs_to = "super::member::Entity",
        from = "Column::MemberId",
        to = "super::member::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Member,
}

impl Related<super::skill::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Skill.def()
    }
}

impl Related<super::member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Member.def()
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