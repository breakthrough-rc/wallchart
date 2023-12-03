use axum_login::AuthUser;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: String,
    pub email: String,
    pub hashed_password: String,
}

impl User {
    pub fn new(email: String, hashed_password: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            email,
            hashed_password,
        }
    }
}

/**
* Need to implement this for axum-login
*/
impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.hashed_password.as_bytes()
    }
}
