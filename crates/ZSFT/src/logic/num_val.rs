use std::ops::{BitAnd, Add, BitOr};
use std::vec::Vec;
use super::{traits::{LEq, LOrd}, l_bool::LBool};

////////////////////////////////////////////////////////////////////////////////
// Number
////////////////////////////////////////////////////////////////////////////////

/// A generalized number, containing both the cardinal and the ordinal numbers.
/// 
/// Both numbers are indexed by a usize. For example:
/// ```
/// let one = Number::Ordinal(1)
/// let two = Number::Ordinal(2)
/// // etc ...
/// 
/// let aleph_null = Number::Cardinal(0)
/// let aleph_one = Number::Cardinal(1)
/// // etc ...
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Number {
    Ordinal(usize),
    Cardinal(usize),
}

impl Add for Number {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        use Number::*;
        match (self, rhs) {
            (Ordinal(a), Ordinal(b)) => Ordinal(a + b),

            // TODO: Is this right?
            (Cardinal(a), Cardinal(b)) => Cardinal(std::cmp::max(a, b)),
            (Cardinal(a), Ordinal(_)) |
            (Ordinal(_), Cardinal(a)) => Cardinal(a),
        }
    }
}

use Number::*;

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match (self, other) {
            (Ordinal(_), Cardinal(_)) => Ordering::Less,
            (Cardinal(_), Ordinal(_)) => Ordering::Greater,
            (Cardinal(a), Cardinal(b)) => a.cmp(b),
            (Ordinal(a), Ordinal(b)) => a.cmp(b),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Number Ranges and Boundaries
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumRangeBoundary<T: Clone> {
    Open(T),
    Closed(T),
    Unbounded,
}

impl <T: Copy> Copy for NumRangeBoundary<T> {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NumRange<T: Clone>(pub NumRangeBoundary<T>, pub NumRangeBoundary<T>);

impl<T: Clone + PartialOrd + Ord> NumRange<T> {

    /// Create a single `NumRange` (where possible) which is valid for the span
    /// of the two given `NumRange`s. I.e.
    ///     ◐------◐ 
    ///        ◐-------◐
    /// =>  ◐----------◐ 
    pub fn union(&self, other: &Self) -> Option<NumBound<T>> {

        let NumRange(l1, u1) = self;
        let NumRange(l2, u2) = other;
        
        // Take lowest lower bound
        let result_lower: NumRangeBoundary<T> = match (l1, l2) {
            (Unbounded, _) | (_, Unbounded) => Unbounded,
            (Open(a), Closed(b)) | 
            (Closed(b), Open(a)) =>
                if a < b {
                    Open(a.clone())
                } else {
                    Closed(b.clone())
                },
            (Closed(a), Closed(b)) =>
                Closed(std::cmp::min(a, b).clone()),
            (Open(a), Open(b)) =>
                Open(std::cmp::min(a, b).clone()),
        };

        // Take highest upper bound
        let result_upper = match (u1, u2) {
            (Unbounded, _) | (_, Unbounded) => Unbounded,
            (Open(a), Closed(b)) |
            (Closed(b), Open(a)) =>
                if a > b {
                    Open(a.clone())
                } else {
                    Closed(b.clone())
                },
            (Closed(a), Closed(b)) =>
                Closed(std::cmp::max(a, b).clone()),
            (Open(a), Open(b)) =>
                Open(std::cmp::max(a, b).clone()),
        };

        NumBound::from_range(result_lower, result_upper)

    }
}

impl<T: Clone + Add<Output=T>> NumRange<T> {
    
    /// For two `NumRanges` representing the possible values of the numbers `x`
    /// and `y`, create a third `NumRange` representing the possible values of
    /// the value `x+y`
    pub fn sum(self, other: Self) -> NumRange<T> {
        
        let new_lower = match (self.0, other.0) {
            (Open(a), Closed(b) | Open(b)) |
            (Closed(b) | Open(b), Open(a)) =>
                Open(a + b),
            (Closed(a), Closed(b)) =>
                Closed(a+b),
            (Unbounded, x) |
            (x, Unbounded) => x,
        };

        let new_upper = match (self.1, other.1) {
            (Open(a), Closed(b) | Open(b)) |
            (Closed(b) | Open(b), Open(a)) =>
                Open(a + b),
            (Closed(a), Closed(b)) =>
                Closed(a+b),
            (Unbounded, x) |
            (x, Unbounded) => x,
        };

        NumRange(new_lower, new_upper)
    }

    pub fn shifted_up(&self, n: T) -> Self {
        use NumRangeBoundary::*;
        let shifted_lower = match &self.0 {
            Open(a) => Open(a.clone()+n.clone()),
            Closed(a) => Open(a.clone()+n.clone()),
            Unbounded => Closed(n.clone()),
        };

        let shifted_upper = match &self.0 {
            Open(a) => Open(a.clone()+n),
            Closed(a) => Open(a.clone()+n),
            Unbounded => Unbounded,
        };

        Self(shifted_lower, shifted_upper)
    }

}

impl <T: Copy> Copy for NumRange<T> {}

impl<T: PartialEq + Ord + Clone> NumRange<T> {
    

    /// Combine two ranges into a single range, returning non if there is no
    /// overlap between the two ranges.
    /// 
    /// Possible Situations:
    ///     - Partial overlap
    ///         ◐------◐
    ///             ◐-------◐
    ///     - Full Overlap
    ///         ◐------------◐
    ///             ◐-----◐
    ///     - No Overlap
    ///         ◐---◐   ◐---◐
    pub fn combine(&self, other: &NumRange<T>) -> Option<NumBound<T>> {

        // The highest lower bound should be taken
        let result_lower: NumRangeBoundary<T> = match (&self.0, &other.0) {
            (x, Unbounded) |
            (Unbounded, x) => x.clone(),

            // ●    o
            (Open(a), Closed(b)) |
            (Closed(b), Open(a)) => 
                if a > b {
                    Open(a.clone())
                } else {
                    Closed(b.clone())
                },

            (Closed(a), Closed(b)) => Closed(std::cmp::max(a, b).clone()),
            (Open(a), Open(b)) => Open(std::cmp::max(a, b).clone()),
        };

        // The lowest upper bound should be taken
        let result_upper: NumRangeBoundary<T> = match (&self.1, &other.1) {
            (x, Unbounded) |
            (Unbounded, x) => x.clone(),

            (Open(a), Closed(b)) |
            (Closed(b), Open(a)) => 
                if a < b {
                    Open(a.clone())
                } else {
                    Closed(b.clone())
                }

            (Closed(a), Closed(b)) => Closed(std::cmp::min(a, b).clone()),
            (Open(a), Open(b)) => Open(std::cmp::min(a, b).clone()),
        };

        NumBound::from_range(result_lower, result_upper)

    }

}

impl<T: Clone + Ord + PartialEq> BitAnd for NumRange<T> {
    type Output = Option<NumBound<T>>;
    fn bitand(self, rhs: Self) -> Self::Output {
        self.combine(&rhs)
    }
}

impl<T: Clone + Add<Output=T>> Add for NumRange<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.sum(rhs)
    }
}

use NumRangeBoundary::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NumBound<T: Clone> {
    Eq(T),
    Range(NumRange<T>),
}

impl<T: Clone + PartialOrd> NumBound<T> {

    /// Attempt to construct a `NumBound` from a pair of range boundaries. Will
    /// return `None` if the resulting range would be invalid (i.e. the lower
    /// bound would be greater than the upper bound).
    pub fn from_range(lower: NumRangeBoundary<T>, upper: NumRangeBoundary<T>) -> Option<Self> {
        match (lower, upper) {
            (Open(l) | Closed(l), Open(u) | Closed(u)) if u < l =>
                None,
            (Open(l), Open(u)) if u <= l =>
                None,
            (Open(l) | Closed(l), Open(u) | Closed(u)) if u == l =>
                Some(Eq(u)),
            (result_lower, result_upper) =>
                Some(Range(NumRange(result_lower, result_upper)))
        }
    }
}

impl<T: Clone + Add<Output=T>> NumBound<T> {

    /// For two `NumBound`s representing the possible values of the numbers `x`
    /// and `y`, create a third `NumBound` representing the possible values of
    /// the value `x+y`
    pub fn sum(self, other: Self) -> Self {
        match (self, other) {
            (Eq(a), Eq(b)) => Eq(a + b),
            (Eq(a), Range(r)) |
            (Range(r), Eq(a)) => Range(r.shifted_up(a)),
            (Range(r1), Range(r2)) => Range(r1 + r2),
        }
    }
}

impl <T: Copy> Copy for NumBound<T> {}

impl<T: PartialOrd + PartialEq + Clone> NumBound<T> {
    pub fn contains(&self, num: &T) -> bool {
        match self {
            Eq(a) => num == a,
            Range(NumRange(lower, upper)) => 
                (match lower {
                    Unbounded => true,
                    Open(n_bound) => n_bound < num,
                    Closed(n_bound) => n_bound <= num,
                }) &&
                (match upper {
                    Unbounded => true,
                    Open(n_bound) => n_bound > num,
                    Closed(n_bound) => n_bound >= num,
                })
        }
    }
}

use NumBound::*;

impl<T: LEq + Clone> LEq for NumBound<T> {
    fn eq(&self, other: &Self) -> LBool {
        // self = other ?
        match (self, other) {
            (Eq(a), Eq(b)) => a.eq(b),
            _ => LBool::Unknown,
        }
    }
}

impl<T: LOrd + Clone> LOrd for NumBound<T> {
    fn ge(&self, other: &Self) -> LBool {
        // self >= other ?
        match (self, other) {
            (Eq(a), Eq(b)) => a.ge(b),
            
            // -- other --- self ----
            // -- o---o --- o---o ---
            // ------ b --- a -------
            // or
            // --- other --- self ----
            // --- o------o-------o---
            // --------- a=b ---------
            (Range(NumRange(Open(a), _)), Range(NumRange(_, Open(b)))) |
            (Range(NumRange(Closed(a), _)), Range(NumRange(_, Open(b)))) |
            (Range(NumRange(Open(a), _)), Range(NumRange(_, Closed(b)))) |
            (Range(NumRange(Closed(a), _)), Range(NumRange(_, Closed(b))))
            if *a.ge(b) =>
                LBool::True,

            // -- self --- other ----
            // -- o---o --- o---o ---
            // ------ a --- b -------
            // or
            // --- self --- other ----
            // ----o------o-------o---
            // --------- a=b ---------
            (Range(NumRange(_, Open(a))), Range(NumRange(Open(b), _))) |
            (Range(NumRange(_, Closed(a))), Range(NumRange(Open(b), _))) |
            (Range(NumRange(_, Open(a))), Range(NumRange(Closed(b), _))) |
            (Range(NumRange(_, Closed(a))), Range(NumRange(Closed(b), _)))
            if *a.le(b) =>
                LBool::False,

            _ => LBool::Unknown,
        }
    }

    fn gt(&self, other: &Self) -> LBool {
        // self > other ?
        match (self, other) {
            (Eq(a), Eq(b)) => a.gt(b),
            
            // self          ◐----
            // other ----◐   a
            //           b
            // OR
            // self      ◐----
            // other ----◐
            //          a=b
            (Range(NumRange(Open(a), _)), Range(NumRange(_, Open(b)))) |
            (Range(NumRange(Closed(a), _)), Range(NumRange(_, Open(b)))) |
            (Range(NumRange(Open(a), _)), Range(NumRange(_, Closed(b))))
            if *a.ge(b) =>
                LBool::True,

            // self          ●----
            // other ----●   a
            //           b
            (Range(NumRange(Closed(a), _)), Range(NumRange(_, Closed(b))))
            if *a.gt(b) =>
                LBool::True,

            // -- self --- other ----
            // -- o---o --- o---o ---
            // ------ a --- b -------
            // or
            // --- self --- other ----
            // ----o------o-------o---
            // --------- a=b ---------
            (Range(NumRange(_, Open(a))), Range(NumRange(Open(b), _))) |
            (Range(NumRange(_, Closed(a))), Range(NumRange(Open(b), _))) |
            (Range(NumRange(_, Open(a))), Range(NumRange(Closed(b), _)))
            if *a.le(b) =>
                LBool::False,

            // -- self --- other ----
            // -- o---● --- ●---o ---
            // ------ a --- b -------
            (Range(NumRange(_, Closed(a))), Range(NumRange(Closed(b), _)))
            if *a.lt(b) =>
                LBool::False,

            _ => LBool::Unknown,
        }
    }
    
    fn le(&self, other: &Self) -> LBool {
        // self <= other ?
        match (self, other) {
            (Eq(a), Eq(b)) => a.le(b),
            
            // -- other --- self ----
            // -- o---o --- o---o ---
            // ------ a --- b -------
            // or
            // --- other --- self ----
            // --- o------o-------o---
            // --------- a=b ---------
            (Range(NumRange(Open(a), _)), Range(NumRange(_, Open(b)))) |
            (Range(NumRange(Closed(a), _)), Range(NumRange(_, Open(b)))) |
            (Range(NumRange(Open(a), _)), Range(NumRange(_, Closed(b)))) |
            (Range(NumRange(Closed(a), _)), Range(NumRange(_, Closed(b))))
            if *a.ge(b) =>
                LBool::False,

            // -- self --- other ----
            // -- o---o --- o---o ---
            // ------ a --- b -------
            // or
            // --- self --- other ----
            // ----o------o-------o---
            // --------- a=b ---------
            (Range(NumRange(Open(a), _)), Range(NumRange(_, Open(b)))) |
            (Range(NumRange(Closed(a), _)), Range(NumRange(_, Open(b)))) |
            (Range(NumRange(Open(a), _)), Range(NumRange(_, Closed(b)))) |
            (Range(NumRange(Closed(a), _)), Range(NumRange(_, Closed(b))))
            if *a.le(b) =>
                LBool::True,

            _ => LBool::Unknown,
        }
    }

    fn lt(&self, other: &Self) -> LBool {
        // self < other ?
        match (self, other) {
            (Eq(a), Eq(b)) => a.gt(b),
            
            // -- other --- self ----
            // -- o---● --- o---o ---
            // -- o---o --- ●---o ---
            // ------ a --- b -------
            // or
            // --- other --- self ----
            // --- o------●o-------o---
            // --- o------o●-------o---
            // --------- a=b ---------
            (Range(NumRange(Open(a), _)), Range(NumRange(_, Open(b)))) |
            (Range(NumRange(Closed(a), _)), Range(NumRange(_, Open(b)))) |
            (Range(NumRange(Open(a), _)), Range(NumRange(_, Closed(b))))
            if *a.ge(b) =>
                LBool::False,

            // -- other --- self ----
            // -- o---● --- ●---o ---
            // ------ a --- b -------
            (Range(NumRange(Closed(a), _)), Range(NumRange(_, Closed(b))))
            if *a.gt(b) =>
                LBool::False,

            // -- self --- other ----
            // -- o---o --- o---o ---
            // ------ a --- b -------
            // or
            // --- self --- other ----
            // ----o------o-------o---
            // --------- a=b ---------
            (Range(NumRange(Open(a), _)), Range(NumRange(_, Open(b)))) |
            (Range(NumRange(Closed(a), _)), Range(NumRange(_, Open(b)))) |
            (Range(NumRange(Open(a), _)), Range(NumRange(_, Closed(b))))
            if *a.le(b) =>
                LBool::True,

            // -- self --- other ----
            // -- o---● --- ●---o ---
            // ------ a --- b -------
            (Range(NumRange(Closed(a), _)), Range(NumRange(_, Closed(b))))
            if *a.lt(b) =>
                LBool::True,

            _ => LBool::Unknown,
        }
    }
}

impl<T: PartialEq + Ord + Clone> BitAnd for NumBound<T> {
    type Output = Option<NumBound<T>>;
    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Eq(a), x) | (x, Eq(a)) =>
                if x.contains(&a) {
                    Some(Eq(a))
                } else {
                    None
                },
            (Range(a), Range(b)) =>
                a.combine(&b),
        }
    }
}

impl<T: Clone + Add<Output=T>> Add for NumBound<T> {
    type Output = NumBound<T>;
    fn add(self, rhs: Self) -> Self::Output {
        self.sum(rhs)
    }
}

/// The union of two NumBounds together calculates the `NumBound` which describes a
/// number which resides in either or both of the individual `NumBound`s. In some
/// cases, the given `NumBound`s cannot be combined in this way (i.e. they are
/// disjoint) in which case `None` is returned.
impl<T: PartialEq + Ord + Clone> BitOr for NumBound<T> {
    type Output = Option<NumBound<T>>;
    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Eq(a), Eq(b)) =>
                if a == b {
                    Some(Eq(a))
                } else {
                    None
                },
            (Eq(a), Range(NumRange(l, u))) |
            (Range(NumRange(l, u)), Eq(a)) => {
                let new_lower = match (&l, &a) {
                    (Open(x), a) => 
                        if a > x {
                            Open(x.clone())
                        } else if a == x {
                            Closed(x.clone())
                        } else {
                            return None;
                        }
                    (Closed(x), a) => {
                        if a >= x {
                            Closed(x.clone())
                        } else {
                            return None;
                        }
                    },
                    (Unbounded, _) => Unbounded,
                };
                let new_upper = match (&u, &a) {
                    (Open(x), a) => 
                        if a < x {
                            Open(x.clone())
                        } else if a == x {
                            Closed(x.clone())
                        } else {
                            return None;
                        }
                    (Closed(x), a) => {
                        if a <= x {
                            Closed(x.clone())
                        } else {
                            return None;
                        }
                    },
                    (Unbounded, _) => Unbounded,
                };
                NumBound::from_range(new_lower, new_upper)
            },
            (Range(a), Range(b)) => 
                a.union(&b),
        }
    }
}

#[cfg(test)]
mod test_num_bound_lord {

    use crate::logic::{LBool::*, traits::LOrd};
    use super::NumBound::*;
    use super::NumRangeBoundary::*;
    use super::NumRange;

    #[test]
    fn test_gt_with_equalities(){
        assert_eq!(Eq(4).gt(&Eq(5)), False);
        assert_eq!(Eq(10).gt(&Eq(3)), True);
        assert_eq!(Eq(3).gt(&Eq(3)), False);
    }

    #[test]
    fn test_ge_with_equalities(){
        assert_eq!(Eq(4).ge(&Eq(5)), False);
        assert_eq!(Eq(10).ge(&Eq(3)), True);
        assert_eq!(Eq(3).ge(&Eq(3)), True);
    }

    #[test]
    fn test_lt_with_equalities(){
        assert_eq!(Eq(4).lt(&Eq(5)), True);
        assert_eq!(Eq(10).lt(&Eq(3)), False);
        assert_eq!(Eq(3).lt(&Eq(3)), False);
    }

    #[test]
    fn test_le_with_equalities(){
        assert_eq!(Eq(4).le(&Eq(5)), True);
        assert_eq!(Eq(10).le(&Eq(3)), False);
        assert_eq!(Eq(3).ge(&Eq(3)), True);
    }

    #[test]
    fn test_gt_with_ranges() {
        // a > b ?

        // a        ◐-----
        // b ----◐  
        assert_eq!(Range(NumRange(Open(4), Unbounded)).gt(&Range(NumRange(Unbounded, Open(3)))), True);
        assert_eq!(Range(NumRange(Closed(4), Unbounded)).gt(&Range(NumRange(Unbounded, Open(3)))), True);
        assert_eq!(Range(NumRange(Open(4), Unbounded)).gt(&Range(NumRange(Unbounded, Closed(3)))), True);
        assert_eq!(Range(NumRange(Closed(4), Unbounded)).gt(&Range(NumRange(Unbounded, Closed(3)))), True);

        // a     ◐----
        // b ----◐
        assert_eq!(Range(NumRange(Open(4), Unbounded)).gt(&Range(NumRange(Unbounded, Open(4)))), True);
        assert_eq!(Range(NumRange(Closed(4), Unbounded)).gt(&Range(NumRange(Unbounded, Open(4)))), True);
        assert_eq!(Range(NumRange(Open(4), Unbounded)).gt(&Range(NumRange(Unbounded, Closed(4)))), True);
        
        // a ----◐
        // b        ◐-----
        assert_eq!(Range(NumRange(Unbounded, Open(1))).gt(&Range(NumRange(Open(2), Unbounded))), False);
        assert_eq!(Range(NumRange(Unbounded, Open(1))).gt(&Range(NumRange(Closed(2), Unbounded))), False);
        assert_eq!(Range(NumRange(Unbounded, Closed(1))).gt(&Range(NumRange(Open(2), Unbounded))), False);

        // a ----◐
        // b     ◐----
        assert_eq!(Range(NumRange(Unbounded, Open(4))).gt(&Range(NumRange(Open(4), Unbounded))), False);
        assert_eq!(Range(NumRange(Unbounded, Open(4))).gt(&Range(NumRange(Closed(4), Unbounded))), False);
        assert_eq!(Range(NumRange(Unbounded, Closed(4))).gt(&Range(NumRange(Open(4), Unbounded))), False);

        // a ----●
        // b     ●----
        assert_eq!(Range(NumRange(Unbounded, Closed(4))).gt(&Range(NumRange(Closed(4), Unbounded))), Unknown);

        // a     ●----
        // b ----●
        assert_eq!(Range(NumRange(Closed(4), Unbounded)).gt(&Range(NumRange(Unbounded, Closed(4)))), Unknown);

        // a    ◐----◐
        // b ----◐
        assert_eq!(Range(NumRange(Open(3), Open(5))).gt(&Range(NumRange(Unbounded, Open(4)))), Unknown);

        // a    ◐-----
        // b ◐----◐
        assert_eq!(Range(NumRange(Closed(3), Unbounded)).gt(&Range(NumRange(Closed(2), Open(4)))), Unknown);

        // a ◐----◐
        // b    ◐----◐
        assert_eq!(Range(NumRange(Open(1), Closed(3))).gt(&Range(NumRange(Open(2), Unbounded))), Unknown);
    }

}

#[cfg(test)]
mod test_combine_num_bounds {

    use super::NumRange;
    use super::NumRangeBoundary::*;
    use super::NumBound::*;

    // TODO: These tests all need padding out

    #[test]
    fn test_and_to_new_ranges() {
        assert_eq!(
            Range(NumRange(Unbounded, Open(4))) & Range(NumRange(Closed(1), Unbounded)),
            Some(Range(NumRange(Closed(1), Open(4))))
        );
    }

    #[test]
    fn test_and_to_eqs() {
        assert_eq!(
            Range(NumRange(Closed(1), Open(4))) & Range(NumRange(Closed(4), Unbounded)),
            Some(Eq(4))
        );
    }

    #[test]
    fn test_and_to_invalid() {
        assert_eq!(
            Range(NumRange(Closed(1), Open(4))) & Range(NumRange(Open(4), Unbounded)),
            None
        );
    }

}