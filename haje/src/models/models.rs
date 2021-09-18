#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
    pub created_on: std::time::SystemTime,
    pub last_login: Option<std::time::SystemTime>,
    pub create_ip: String,
    pub last_ip: Option<String>,
}