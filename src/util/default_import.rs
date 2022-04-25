
#[macro_export]
macro_rules! default {
    ( $name:ident ) => {
        mod $name;
        pub use $name::$name;
    };
}

// pub(crate) use default;