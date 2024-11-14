use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Cat {
    pub identifier: String,
    pub name: String,
    pub breed: String,
    pub microchip: Option<String>,
    pub image: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewCat {
    pub identifier: Option<String>,
    pub name: String,
    pub breed: String,
    pub microchip: Option<String>,
    pub image: Option<String>,
}

#[derive(Clone, Debug)]
pub struct UploadedFile {
    pub name: String,
    pub contents: Vec<u8>
}