
macro_rules! expect_scoped_item_to_be {
    ($scope:ident, $name:literal, $type:ident) => {
        match $scope.get($literal).unwrap() {
            ScopedItem::$ident(_) => {},
            _ => panic!("Expected \"F\" to be a Set"),
        };
    };
}

pub(self) use expect_scoped_item_to_be;