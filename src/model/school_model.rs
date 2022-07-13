use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "school")]
pub struct School {
    #[sea_orm(primary_key)]
    pub school_id: i32,
    pub school_name: String,
    pub description: Option<String>
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "school_class")]
pub struct Class {
    #[sea_orm(primary_key)]
    pub class_id: i32,
    pub school_id: i32,
    pub description: Option<String>
}