extern crate zsft;

use zsft::*;

#[test]
pub fn test_set_with_elements_and_size() {

    let item_a = Item::new();
    let item_b = Item::new();
    let item_c = Item::new();

    let set_a = Set::anonymous();
    WithItems::assert_on(&set_a, vec![
        &item_a, &item_b,
    ]).expect();
    
    assert_eq!(set_a.contains(item_a), LBool::True);
    assert_eq!(set_a.contains(item_b), LBool::True);
    assert_eq!(set_a.contains(item_c), LBool::Unknown);

    HasSize::assert_on(
        NumBound::Eq(Number::Ordinal(2)),
        &set_a
    ).expect();
    assert_eq!(set_a.contains(item_c), LBool::False);
}

#[test]
pub fn test_size_and_with_elements() {

    let item_a = Item::new();
    let item_b = Item::new();
    let item_c = Item::new();

    let set_a = Set::anonymous();
    WithItems::assert_on(&set_a, vec![
        &item_a, &item_b,
    ]).expect();

    HasSize::assert_on(
        NumBound::Eq(Number::Ordinal(2)),
        &set_a
    ).expect();

    assert_eq!(set_a.contains(item_a), LBool::True);
    assert_eq!(set_a.contains(item_b), LBool::True);
    assert_eq!(set_a.contains(item_c), LBool::False);
}