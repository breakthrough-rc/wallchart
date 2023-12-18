use axum_login::AuthUser;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: String,
    pub email: String,
    pub hashed_password: String,
    pub role: UserRole,
}

impl User {
    pub fn new(email: String, hashed_password: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            email,
            hashed_password,
            role: UserRole::Organizer,
        }
    }
    pub fn update(&self, email: String, role: UserRole) -> Self {
        Self {
            id: self.id.clone(),
            email,
            hashed_password: self.hashed_password.clone(),
            role,
        }
    }
    pub fn has_perm(&self, permission: UserPermission) -> bool {
        self.role.has_perm(permission)
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

#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    Admin,
    Organizer,
    SuperAdmin,
}

impl UserRole {
    pub fn has_perm(&self, permission: UserPermission) -> bool {
        match self {
            Self::Organizer => match permission {
                UserPermission::CreateUser => false,
                UserPermission::ReadUser => false,
                UserPermission::UpdateUser => false,
                UserPermission::DeleteUser => false,
            },
            Self::Admin => match permission {
                UserPermission::CreateUser => false,
                UserPermission::ReadUser => false,
                UserPermission::UpdateUser => false,
                UserPermission::DeleteUser => false,
            },
            Self::SuperAdmin => true,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::Organizer => "Organizer".to_string(),
            Self::Admin => "Admin".to_string(),
            Self::SuperAdmin => "SuperAdmin".to_string(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum UserPermission {
    CreateUser,
    ReadUser,
    UpdateUser,
    DeleteUser,
}

impl From<&str> for UserPermission {
    fn from(permission: &str) -> Self {
        match permission {
            "user.create" => Self::CreateUser,
            "user.read" => Self::ReadUser,
            "user.update" => Self::UpdateUser,
            "user.delete" => Self::DeleteUser,
            _ => panic!("Permission does not exist"),
        }
    }
}
