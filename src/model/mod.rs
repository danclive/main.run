use mon::oid::ObjectId;
use mon::bson::bson::UTCDateTime;
use mon::db::Database;

use serde_bytes;

use struct_document::StructDocument;
use DB;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Role {
    Admin,
    Guest
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub avatar: String,
    pub role: Role,
    #[serde(with = "serde_bytes")]
    pub password: Vec<u8>,
    pub create_at: UTCDateTime,
    pub update_at: UTCDateTime
}

impl StructDocument for User {
    const NAME: &'static str = "user";

    fn get_database() -> Database {
        DB.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub image: Vec<String>,
    pub author_id: ObjectId,
    pub content: String,
    pub create_at: UTCDateTime,
    pub update_at: UTCDateTime,
    pub status: i32
}

impl StructDocument for Article {
    const NAME: &'static str = "article";

    fn get_database() -> Database {
        DB.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collect {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub image: Vec<String>,
    #[serde(default)]
    pub articles_id: Vec<ObjectId>,
    pub create_at: UTCDateTime,
    pub update_at: UTCDateTime,
}

impl StructDocument for Collect {
    const NAME: &'static str = "collect";

    fn get_database() -> Database {
        DB.clone()
    }
}
