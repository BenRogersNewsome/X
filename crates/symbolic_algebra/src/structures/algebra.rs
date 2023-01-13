use std::ops::Deref;

use super::identity::Identity;

/// Utility type. An algebra is a set of one-way identities.
pub struct Algebra<'a>(Vec<Identity<'a>>);

impl<'a> Algebra<'a> {

    pub fn new(identities: Vec<Identity<'a>>) -> Self {
        Self(identities)
    }
}

impl<'a> Deref for Algebra<'a> {

    type Target = Vec<Identity<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}