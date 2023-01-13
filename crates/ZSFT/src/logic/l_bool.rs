use std::ops::{Deref, BitOr, BitAnd, Not};


/// The Logical Boolean. Has `True` and `False` for statements which are *verifiably* true or false, and `Unknown` for
/// everything else.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LBool {
    True,
    False,
    Unknown,
}

use LBool::*;

impl BitOr for LBool {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (False, False) => False,
            (True, _) => True,
            (_, True) => True,
            _ => Unknown,
        }
    }
}

impl BitAnd for LBool {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (True, True) => True,
            (False, _) => False,
            (_, False) => False,
            _ => Unknown,
        }
    }
}

impl Not for LBool {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            True => False,
            False => True,
            Unknown => Unknown,
        }
    }
}

impl Deref for LBool {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::True => &true,
            _ => &false,
        }    
    }
}

impl From<bool> for LBool {
    fn from(b: bool) -> Self {
        if b {
            Self::True
        } else {
            Self::False
        }
    }
}

#[cfg(test)]
mod lbool_logical_ops_tests {

    use super::LBool::*;

    #[test]
    fn test_logical_or() {
        assert_eq!(True | True, True);
        assert_eq!(True | False, True);
        assert_eq!(False | True, True);
        assert_eq!(False | False, False);
        assert_eq!(Unknown | False, Unknown);
        assert_eq!(False | Unknown, Unknown);
        assert_eq!(Unknown | True, True);
        assert_eq!(True | Unknown, True);
        assert_eq!(Unknown | Unknown, Unknown);
    }

    #[test]
    fn test_logical_and() {
        assert_eq!(True & True, True);
        assert_eq!(True & False, False);
        assert_eq!(False & True, False);
        assert_eq!(False & False, False);
        assert_eq!(Unknown & False, False);
        assert_eq!(False & Unknown, False);
        assert_eq!(Unknown & True, Unknown);
        assert_eq!(True & Unknown, Unknown);
        assert_eq!(Unknown & Unknown, Unknown);
    }

    #[test]
    fn test_logical_not() {
        assert_eq!(!True, False);
        assert_eq!(!False, True);
        assert_eq!(!Unknown, Unknown);
    }

}
