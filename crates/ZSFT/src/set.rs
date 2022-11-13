use super::set_element::{SetElement, SetElementDefinition};

#[derive(Clone, Debug)]
pub struct Set<'a>(SetDefinition<'a>);

impl PartialEq for Set<'_>{
    
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (SetDefinition::Anonymous, SetDefinition::Anonymous) => self._literally_equal(other),
            _ => self.0 == other.0,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for Set<'_> { }

impl<'a> Set<'a> {
    pub fn new() -> Self {
        Self(SetDefinition::Anonymous)
    }

    /// A utility function for creating the union of 2 sets.
    pub fn union<'b: 'a>(set_1: &'b Set, set_2: &'b Set) -> Self {
        Self(SetDefinition::Union(set_1, set_2))
    }

    /// A utility function for creating the intersection of 2 sets.
    pub fn intersection<'b: 'a>(set_1: &'b Set, set_2: &'b Set) -> Self {
        Self(SetDefinition::Intersection(set_1, set_2))
    }

    /// A utility function for creating the difference of 2 sets.
    pub fn difference<'b: 'a>(set_1: &'b Set, set_2: &'b Set) -> Self {
        Self(SetDefinition::Difference(set_1, set_2))
    }

    /// A utility function for creating a set from a list of set elements.
    pub fn from_elements<'b: 'a>(set_elements: Vec<&'b SetElement<'b>>) -> Self {
        Self(SetDefinition::FromElements(set_elements))
    }

    pub fn contains(&self, set_element: &SetElement) -> bool {
        match &self.0 {
            SetDefinition::Anonymous => {
                match **set_element {
                    // Needed to prevent infinite recursion when asking if an anonymous element is a member of an anonymous set.
                    SetElementDefinition::Anonymous => false,
                    _ => set_element.in_set(self),
                }
            },
            SetDefinition::Union(set_1, set_2) => {
                set_1.contains(set_element) || set_2.contains(set_element)
            },
            SetDefinition::Intersection(set_1, set_2) => {
                set_1.contains(set_element) && set_2.contains(set_element)
            },
            SetDefinition::Difference(set_1, set_2) => {
                set_1.contains(set_element) && !set_2.contains(set_element)
            },
            SetDefinition::FromElements(elements) => elements.contains(&set_element), 
        }
    }

    fn _literally_equal(&self, other: &Set) -> bool {
        let addr_self: *const Set = self;
        let addr_other: *const Set = other;

        addr_self == addr_other
    }
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum SetDefinition<'a> {
    Anonymous,
    Union(&'a Set<'a>, &'a Set<'a>),
    Intersection(&'a Set<'a>, &'a Set<'a>),
    Difference(&'a Set<'a>, &'a Set<'a>),
    FromElements(Vec<&'a SetElement<'a>>),
}

#[cfg(test)]
mod test_set_equality {
    use super::*;

    #[test]
    fn test_reflexive_equality_of_sets() {
        let set_a = Set::new();
        let set_b = Set::new();
        let set_c = Set::union(&set_a, &set_b);
        let set_d = Set::intersection(&set_a, &set_b);
        let set_e = Set::difference(&set_a, &set_b);

        assert_eq!(set_a, set_a);
        assert_eq!(set_b, set_b);
        assert_eq!(set_c, set_c);
        assert_eq!(set_d, set_d);
        assert_eq!(set_e, set_e);
    }

    #[test]
    fn test_non_equality_of_anonymous_sets() {
        let set_a = Set::new();
        let set_b = Set::new();

        assert_ne!(set_a, set_b);
    }

    #[test]
    fn test_equality_of_union_sets() {
        let set_a = Set::new();
        let set_b = Set::new();
        let set_c = Set::union(&set_a, &set_b);
        let set_d = Set::union(&set_a, &set_b);
        let set_e = Set::union(&set_c, &set_b);

        assert_ne!(set_a, set_c);
        assert_ne!(set_a, set_d);
        assert_ne!(set_b, set_c);
        assert_ne!(set_b, set_d);
        assert_eq!(set_c, set_d);
        assert_ne!(set_c, set_e);
        assert_ne!(set_d, set_e);
    }

    #[test]
    fn test_equality_of_intersection_sets() {
        let set_a = Set::new();
        let set_b = Set::new();
        let set_c = Set::intersection(&set_a, &set_b);
        let set_d = Set::intersection(&set_a, &set_b);
        let set_e = Set::intersection(&set_c, &set_b);

        assert_ne!(set_a, set_c);
        assert_ne!(set_a, set_d);
        assert_ne!(set_b, set_c);
        assert_ne!(set_b, set_d);
        assert_eq!(set_c, set_d);
        assert_ne!(set_c, set_e);
        assert_ne!(set_d, set_e);
    }

    #[test]
    fn test_equality_of_difference_sets() {
        let set_a = Set::new();
        let set_b = Set::new();
        let set_c = Set::difference(&set_a, &set_b);
        let set_d = Set::difference(&set_a, &set_b);
        let set_e = Set::difference(&set_c, &set_b);

        assert_ne!(set_a, set_c);
        assert_ne!(set_a, set_d);
        assert_ne!(set_b, set_c);
        assert_ne!(set_b, set_d);
        assert_eq!(set_c, set_d);
        assert_ne!(set_c, set_e);
        assert_ne!(set_d, set_e);
    }

    #[test]
    fn test_non_equality_of_different_set_definitions() {
        let set_a = Set::new();
        let set_b = Set::new();
        let set_c = Set::union(&set_a, &set_b);
        let set_d = Set::intersection(&set_a, &set_b);
        let set_e = Set::difference(&set_a, &set_b);

        assert_ne!(set_a, set_b);
        assert_ne!(set_c, set_d);
        assert_ne!(set_c, set_e);
        assert_ne!(set_d, set_e);
    }
}

#[cfg(test)]
mod test_set_membership {

    use super::*;

    #[test]
    fn test_direct_membership() {

        let set_a = Set::new();
        let element_a = SetElement::element_of(&set_a);
        let set_b = Set::new();

        assert!(set_a.contains(&element_a));
        assert!(!set_b.contains(&element_a));
    }

    #[test]
    fn test_union_membership() {

        let set_a = Set::new();
        let element_a = SetElement::element_of(&set_a);
        let set_b = Set::new();

        let set_c = Set::union(&set_a, &set_b);
        
        assert!(set_a.contains(&element_a));
        assert!(!set_b.contains(&element_a));
        assert!(set_c.contains(&element_a));
    }

    #[test]
    fn test_union_membership_recursive() {

        let set_a = Set::new();
        let element_a = SetElement::element_of(&set_a);
        let set_b = Set::new();
        
        let set_c = Set::union(&set_a, &set_b);
        let set_d = Set::union(&set_c, &set_b);

        assert!(set_a.contains(&element_a));
        assert!(!set_b.contains(&element_a));
        assert!(set_c.contains(&element_a));
        assert!(set_d.contains(&element_a));
    }

    #[test]
    fn test_intersection_membership() {

        let set_a = Set::new();
        let element_a = SetElement::element_of(&set_a);
        let set_b = Set::new();

        let set_c = Set::union(&set_a, &set_b); // a is in C
        let set_d = Set::intersection(&set_a, &set_c); // a is in D

        assert!(set_a.contains(&element_a));
        assert!(!set_b.contains(&element_a));
        assert!(set_c.contains(&element_a));
        assert!(set_d.contains(&element_a));
    }

    #[test]
    fn test_difference_membership() {

        let set_a = Set::new();
        let element_a = SetElement::element_of(&set_a);
        let set_b = Set::new();

        let set_c = Set::difference(&set_a, &set_b); // a is in C
        let set_d = Set::difference(&set_a, &set_c); // a not in D

        assert!(set_a.contains(&element_a));
        assert!(!set_b.contains(&element_a));
        assert!(set_c.contains(&element_a));
        assert!(!set_d.contains(&element_a));
    }

    #[test]
    fn test_from_elements_membership() {
        let element_a = SetElement::new();
        let element_b = SetElement::new();
        let element_c = SetElement::new();

        let set_a = Set::from_elements(vec![
            &element_a,
            &element_b,
            &element_c,
        ]);
        let set_b = Set::from_elements(vec![]);

        assert!(set_a.contains(&element_a));
        assert!(set_a.contains(&element_b));
        assert!(set_a.contains(&element_c));
        assert!(!set_b.contains(&element_a));
        assert!(!set_b.contains(&element_b));
        assert!(!set_b.contains(&element_c));
    }

    #[test]
    fn test_not_infinite_recursion_in_anonymous_membership() {
        use super::super::set_element::SetElement;
        let set_a = Set::new();
        let element_a = SetElement::new();

        assert!(!set_a.contains(&element_a));
    }
}