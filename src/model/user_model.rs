use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct User {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    #[sea_orm(unique)]
    pub user_name: String,
    pub password: String,
    pub description: Option<String>,
    pub school_id: i32,
    pub class_id: i32,
    #[sea_orm(default_value = 1)]
    pub user_type: UserType
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum UserType {
    #[sea_orm(num_value = 0)]
    Teacher,
    #[sea_orm(num_value = 1)]
    Student,
    #[sea_orm(num_value = 2)]
    Parents
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Friend {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    #[sea_orm(primary_key)]
    pub friend_id: i32,
}