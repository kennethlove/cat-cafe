use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Cat {
    pub identifier: String,
    pub name: String,
    pub breed: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewCat {
    pub identifier: Option<String>,
    pub name: String,
    pub breed: String,
}
