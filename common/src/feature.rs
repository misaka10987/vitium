use serde::{Deserialize, Serialize};

use crate::ObjClass;

#[derive(Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Feature {
    RegID(String),
    Script(Script),
}

#[derive(Clone, Serialize, Deserialize, Hash)]
pub struct Script {
    pub name: String,
    pub script: String,
}

impl PartialEq for Script {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Script {}

#[derive(Clone, Serialize, Deserialize)]
pub struct FeatureAction {
    pub cat: ObjClass,
    pub obj: u64,
    pub feature: String,
    pub arg: Vec<String>,
}

impl FeatureAction {
    pub fn example() -> Self {
        Self {
            cat: ObjClass::Item,
            obj: 114514,
            feature: "example-feature".to_string(),
            arg: vec!["arg1", "arg2"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}
