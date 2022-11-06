use std::ops::Deref;

use super::identity::Identity;

/// Utility type. An algebra is a set of two-way identities.
pub struct Algebra(Vec<Identity>);

impl Algebra {

    pub fn new(identities: Vec<Identity>) -> Self {
        Self(identities)
    }
}

impl Deref for Algebra {

    type Target = Vec<Identity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}