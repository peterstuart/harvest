use super::Id;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Minimal<T> {
    pub id: Id<T>,
    pub name: String,
}

impl<T> std::fmt::Display for Minimal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{} ({})", self.name, self.id).fmt(f)
    }
}
