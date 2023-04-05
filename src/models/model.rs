use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 使用结构体接收参数使用 serde::Deserialize
// 使用结构体作为返回结果 serde::Serialize
#[derive(Deserialize, Serialize)]
pub struct Wizard {
    pub name: String,
    pub level: u8,
}

pub struct Repository {
    pub wizards: HashMap<String, Wizard>,
}

impl Repository {
    pub fn new() -> Self {
        Self {
            wizards: HashMap::new(),
        }
    }
}
