use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub enum CatStatus {
    #[default]
    New,
    Waiting,
    InCafe,
    Fostered,
    Adopted,
}

impl Display for CatStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CatStatus::New => write!(f, "new"),
            CatStatus::Waiting => write!(f, "waiting"),
            CatStatus::InCafe => write!(f, "in a cafe"),
            CatStatus::Fostered => write!(f, "fostered"),
            CatStatus::Adopted => write!(f, "adopted"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Cat {
    pub identifier: String,
    pub name: String,
    pub breed: String,
    pub microchip: Option<String>,
    pub image: Option<String>,
    pub status: CatStatus
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cafe {
    pub identifier: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewCafe {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CatCafe {
    pub cat_identifier: String,
    pub cafe_identifier: String,
}
