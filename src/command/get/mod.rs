use serde::{Deserialize, Serialize};
use std::string;

pub mod item;

#[derive(Serialize, Deserialize)]
struct OpItem {
    uuid: String,
    vaultUuid: String,
    details: OpDetails,
    overview: OpOverview,
}

#[derive(Serialize, Deserialize)]
struct OpDetails {
    fields: Vec<OpField>,
}

#[derive(Serialize, Deserialize)]
struct OpOverview {
    url: String,
    title: String,
}

#[derive(Serialize, Deserialize)]
struct OpField {
    name: String,
    value: String,
}

impl OpItem {
    pub fn get_field(&self, name: string::String) -> string::String {
        for field in &self.details.fields {
            if field.name == name {
                return field.value.clone();
            }
        }
        "".to_string()
    }
}
