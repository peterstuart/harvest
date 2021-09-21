use super::Id;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Minimal<T> {
    pub id: Id<T>,
    pub name: String,
}
