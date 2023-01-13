use super::l_bool::LBool::{self, *};


pub trait LEq {
    fn eq(&self, other: &Self) -> LBool;
}

impl LEq for usize {
    fn eq(&self, other: &Self) -> LBool {
        if self == other {
            True
        } else if self != other {
            False
        } else {
            Unknown
        }
    }
}

impl LEq for u8 {
    fn eq(&self, other: &Self) -> LBool {
        if self == other {
            True
        } else if self != other {
            False
        } else {
            Unknown
        }
    }
}

impl LEq for u16 {
    fn eq(&self, other: &Self) -> LBool {
        if self == other {
            True
        } else if self != other {
            False
        } else {
            Unknown
        }
    }
}

impl LEq for u32 {
    fn eq(&self, other: &Self) -> LBool {
        if self == other {
            True
        } else if self != other {
            False
        } else {
            Unknown
        }
    }
}

impl LEq for u64 {
    fn eq(&self, other: &Self) -> LBool {
        if self == other {
            True
        } else if self != other {
            False
        } else {
            Unknown
        }
    }
}

impl LEq for u128 {
    fn eq(&self, other: &Self) -> LBool {
        if self == other {
            True
        } else if self != other {
            False
        } else {
            Unknown
        }
    }
}

impl LEq for isize {
    fn eq(&self, other: &Self) -> LBool {
        if self == other {
            True
        } else if self != other {
            False
        } else {
            Unknown
        }
    }
}

impl LEq for i8 {
    fn eq(&self, other: &Self) -> LBool {
        if self == other {
            True
        } else if self != other {
            False
        } else {
            Unknown
        }
    }
}

impl LEq for i16 {
    fn eq(&self, other: &Self) -> LBool {
        if self == other {
            True
        } else if self != other {
            False
        } else {
            Unknown
        }
    }
}

impl LEq for i32 {
    fn eq(&self, other: &Self) -> LBool {
        if self == other {
            True
        } else if self != other {
            False
        } else {
            Unknown
        }
    }
}

impl LEq for i64 {
    fn eq(&self, other: &Self) -> LBool {
        if self == other {
            True
        } else if self != other {
            False
        } else {
            Unknown
        }
    }
}

impl LEq for i128 {
    fn eq(&self, other: &Self) -> LBool {
        if self == other {
            True
        } else if self != other {
            False
        } else {
            Unknown
        }
    }
}

pub trait LOrd {
    fn ge(&self, other: &Self) -> LBool;
    fn gt(&self, other: &Self) -> LBool;
    fn le(&self, other: &Self) -> LBool;
    fn lt(&self, other: &Self) -> LBool;
}

impl<T> LOrd for T where T: PartialOrd {
    fn ge(&self, other: &Self) -> LBool {
        LBool::from(self.ge(other))
    }

    fn gt(&self, other: &Self) -> LBool {
        LBool::from(self.gt(other))
    }

    fn le(&self, other: &Self) -> LBool {
        LBool::from(self.le(other))
    }

    fn lt(&self, other: &Self) -> LBool {
        LBool::from(self.lt(other))
    }
}