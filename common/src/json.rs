// use std::{fs, marker::PhantomData};

// use serde::Deserialize;
// use serde_json::from_str;

use serde::{de, ser};

pub trait Check {
    fn check(&self) -> bool;
}

pub fn json<T>(obj: T) -> String
where
    T: ser::Serialize,
{
    serde_json::to_string(&obj).expect("serialize error")
}

pub fn obj<'a, T>(json: &'a str) -> T
where
    T: de::Deserialize<'a>,
{
    serde_json::from_str(json).unwrap()
}

mod test {
    #[test]
    fn conv() {
        use crate::json::{json, obj};
        assert_eq!(obj::<i32>("114514"), 114514);
        assert_eq!(json(114514), "114514");
    }
}
