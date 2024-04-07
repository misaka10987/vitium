use serde::{Deserialize, Serialize};

use crate::Obj;

#[derive(Clone, Serialize, Deserialize, Hash)]
pub struct Feature {
    pub id: String,
    pub script: Script,
}

impl PartialEq for Feature {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Feature {}

#[derive(Clone, Serialize, Deserialize, Hash)]
pub enum Script {
    Reg(String),
    Inst(String),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FeatureAction {
    pub obj: Obj,
    pub feature: String,
    pub arg: Vec<String>,
}

// impl FeatureAction {
//     pub fn example() -> Self {
//         Self {
//             cat: ObjClass::Item,
//             obj: 114514,
//             feature: "example-feature".to_string(),
//             arg: vec!["arg1", "arg2"]
//                 .into_iter()
//                 .map(|s| s.to_string())
//                 .collect(),
//         }
//     }
// }
