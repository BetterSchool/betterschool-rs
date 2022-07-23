#[derive(Queryable)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub description: String,
    pub user_type: i8
}

pub enum UserType {
    Student,
    Teacher,
    Parents
}