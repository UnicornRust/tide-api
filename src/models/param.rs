use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct NameParams {
    pub name: String,
}

impl Default for NameParams {
    fn default() -> Self {
        Self {
            name: "World".to_string(),
        }
    }
}
