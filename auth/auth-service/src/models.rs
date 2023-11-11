#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub hashed_password: String,
}
