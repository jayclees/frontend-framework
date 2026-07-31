#[macro_export]
macro_rules! dd {
    ( $( $x:expr ),* ) => {
        dbg!($($x),*);
        std::process::exit(1);
    };
}

pub use dd;

#[macro_export]
macro_rules! ddif {
    ( $y:expr, $( $x:expr ),+ ) => {
        if $y {
            dbg!($($x),*);
            std::process::exit(1);
        }
    };
}

pub use ddif;
