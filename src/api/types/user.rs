use super::Id;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub id: Id<User>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
