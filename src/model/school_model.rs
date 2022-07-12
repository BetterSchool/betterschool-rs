use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "school")]
pub struct School {
    #[sea_orm(primary_key)]
    pub school_id: i32,
}