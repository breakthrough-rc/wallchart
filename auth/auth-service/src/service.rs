use std::sync::Arc;

use crate::{
    //##PLOP INSERT COMMAND IMPORTS HOOK##
};

#[derive(Clone)]
pub struct AuthService {
    //##PLOP INSERT COMMAND HOOK##
    // Add service infra dependencies here
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            //##PLOP INSERT COMMAND INSTANTIATION HOOK##
        }
    }
    //##PLOP INSERT DELEGATE HOOK##
}
