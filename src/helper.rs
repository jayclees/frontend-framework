#[macro_export]
macro_rules! dd {
    ( $( $x:expr ),* ) => {
        dbg!($($x),*);
        std::process::exit(1);
    };
}

use serde_json::Value;
#[allow(unused_imports)]
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

#[allow(unused_imports)]
pub use ddif;

// From https://stackoverflow.com/a/54118457
pub fn merge(a: &mut Value, b: Value) {
    if let Value::Object(a) = a {
        if let Value::Object(b) = b {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(&k);
                } else {
                    merge(a.entry(k).or_insert(Value::Null), v);
                }
            }

            return;
        }
    }

    *a = b;
}