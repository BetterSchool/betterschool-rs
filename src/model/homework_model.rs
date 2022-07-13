use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "homework")]
pub struct Homework {
    #[sea_orm(primary_key)]
    pub homework_id: i32,
    pub title: String,
    pub content: String,
    pub begin_date: Date,
    pub end_date: Date
}