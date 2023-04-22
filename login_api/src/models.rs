

use axum::http::Error;
use bcrypt::{DEFAULT_COST, hash};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub id: i32,
    pub username: String,
    pub password: String,
}


impl NewUser {

    pub fn new(id: i32, username: String, password: String) -> Self {
        NewUser {
            id,
            username,
            password,
        }
    }

    pub(crate) async fn save_user(&self) -> Result<String ,Error> {
        todo!()
    }

    pub fn is_password_match(&self, password: &str) -> bool {
        bcrypt::verify(password, self.password.as_ref()).unwrap_or(false)
    }

    pub(crate) async fn find_by_username(username: &str) -> NewUser  {
        todo!()
    }
}
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hashed_password = hash(password, DEFAULT_COST)?;
    Ok(hashed_password)
}
