// use std::{fs, marker::PhantomData};

// use serde::Deserialize;
// use serde_json::from_str;

pub trait Check {
    fn check(&self) -> bool;
}

// pub struct Loader<T> {
//     pub path: String,
//     buf:String,
//     fake: PhantomData<T>,
// }

// impl<'a, T> Loader<T>
// where
//     T: Clone + Check + Deserialize<'a>,
// {
//     fn load(&mut self) -> T {
//         self.buf = fs::read_to_string(self.path.as_str())
//             .expect(format!("[FATAL] IO Error reading {}", self.path).as_str());
//         let obj = from_str::<T>(self.buf.as_str())
//             .expect(format!("[FATAL] Parsing Error reading {}", self.path).as_str()).clone();
//         obj
//     }
// }
