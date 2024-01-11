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
    serde_json::from_str(json).expect("deserialize error")
}

pub trait FromJSON<T>
where
    T: de::DeserializeOwned,
{
    fn obj(&self) -> T;
}

impl<T> FromJSON<T> for String
where
    T: de::DeserializeOwned,
{
    fn obj(&self) -> T {
        serde_json::from_str(self).unwrap()
    }
}

pub trait JSON
where
    Self: ser::Serialize + de::DeserializeOwned,
{
    fn json(&self) -> String {
        serde_json::to_string(self).expect("serialize error")
    }
}

macro_rules! jsonimpl {
    ( $( $type:ty ),* ) => {
        $(
            impl JSON for $type {}
        )*
    };
}
jsonimpl!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
jsonimpl!(String);

mod tests {
    #[test]
    fn conv() {
        use crate::json::{FromJSON, JSON};
        let a: i32 = "114514".to_string().obj();
        assert_eq!(a, 114514);
        assert_eq!(114514.json(), "114514");
    }
}
