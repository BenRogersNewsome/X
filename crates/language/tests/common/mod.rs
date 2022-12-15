
macro_rules! expect_scoped_item_to_be {
    ($scope:ident, $name:literal, $type:ident) => {
        match $scope.get($name).unwrap() {
            language::ScopedItem::$type(x) => x,
            _ => panic!("Expected \"{:?}\" to be a {:?}", $name, ""),
        }
    };
}

pub(super) use expect_scoped_item_to_be;