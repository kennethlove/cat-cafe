use serde::{Deserialize, Serialize};
// use surrealdb::RecordId;
// use surrealdb::sql::Uuid;
// use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cat {
    // pub id: String,
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
