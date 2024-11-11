use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserDataGuard<'r> {
    pub name: Option<&'r str>,
    pub username: &'r str,
    pub password: &'r str,
}
