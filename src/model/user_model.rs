use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct User {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub user_name: String,
    pub password: String,
    pub intro: Option<String>,
}