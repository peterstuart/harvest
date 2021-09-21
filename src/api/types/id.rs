use serde::Deserialize;
use std::marker::PhantomData;

#[derive(Clone, Copy, Deserialize)]
#[serde(transparent)]
pub struct Id<T> {
    value: u64,
    phantom: PhantomData<T>,
}

impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> std::fmt::Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
