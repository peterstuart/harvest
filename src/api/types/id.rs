use crate::Result;
use serde::{Deserialize, Serialize};
use std::{marker::PhantomData, str::FromStr};

#[derive(Clone, Copy, Deserialize, Eq, Serialize)]
#[serde(transparent)]
pub struct Id<T> {
    value: u64,
    phantom: PhantomData<T>,
}

impl<T> Id<T> {
    fn new(value: u64) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
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

impl<T> FromStr for Id<T> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self::new(s.parse()?))
    }
}
