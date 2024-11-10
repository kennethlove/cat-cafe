use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use surrealdb::sql::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cat {
    // pub id: RecordId,
    pub identifier: Uuid,
    pub name: String,
    pub breed: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewCat {
    pub identifier: Option<Uuid>,
    pub name: String,
    pub breed: String,
}
