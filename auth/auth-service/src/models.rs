use axum_login::{secrecy::SecretVec, AuthUser};

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub hashed_password: String,
}

/**
* Need to implement this for axum-login
*/
impl AuthUser<String> for User {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.hashed_password.clone().into())
    }
}
