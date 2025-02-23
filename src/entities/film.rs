//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.5

use super::sea_orm_active_enums::Rating;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "film")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub film_id: i32,
    pub title: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub release_year: Option<u16>,
    pub language_id: i32,
    pub original_language_id: Option<i32>,
    pub rental_duration: i32,
    #[sea_orm(column_type = "Decimal(Some((4, 2)))")]
    pub rental_rate: Decimal,
    pub length: Option<i32>,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub replacement_cost: Decimal,
    pub rating: Option<Rating>,
    #[sea_orm(
        // column_type = "custom(\"SET ('Trailers', 'Commentaries', 'Deleted Scenes', 'Behind the Scenes')\")",
        nullable
    )]
    pub special_features: Option<String>,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::film_actor::Entity")]
    FilmActor,
    #[sea_orm(has_many = "super::film_category::Entity")]
    FilmCategory,
    #[sea_orm(has_many = "super::inventory::Entity")]
    Inventory,
    #[sea_orm(
        belongs_to = "super::language::Entity",
        from = "Column::LanguageId",
        to = "super::language::Column::LanguageId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Language2,
    #[sea_orm(
        belongs_to = "super::language::Entity",
        from = "Column::OriginalLanguageId",
        to = "super::language::Column::LanguageId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Language1,
}

impl Related<super::film_actor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FilmActor.def()
    }
}

impl Related<super::film_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FilmCategory.def()
    }
}

impl Related<super::inventory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inventory.def()
    }
}

impl Related<super::actor::Entity> for Entity {
    fn to() -> RelationDef {
        super::film_actor::Relation::Actor.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::film_actor::Relation::Film.def().rev())
    }
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        super::film_category::Relation::Category.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::film_category::Relation::Film.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::film_actor::Entity")]
    FilmActor,
    #[sea_orm(entity = "super::film_category::Entity")]
    FilmCategory,
    #[sea_orm(entity = "super::inventory::Entity")]
    Inventory,
    #[sea_orm(entity = "super::language::Entity", def = "Relation::Language2.def()")]
    Language2,
    #[sea_orm(entity = "super::language::Entity", def = "Relation::Language1.def()")]
    Language1,
    #[sea_orm(entity = "super::actor::Entity")]
    Actor,
    #[sea_orm(entity = "super::category::Entity")]
    Category,
}
