use mongors::object_id::ObjectId;
use mongors::bson::bson::UTCDateTime;
use mongors::database::Database;

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

model!(User, "user");

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub image: Vec<String>,
    pub author_id: ObjectId,
    #[serde(default)]
    pub collect_ids: Vec<ObjectId>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub summary: String,
    pub create_at: UTCDateTime,
    pub update_at: UTCDateTime,
    pub status: i32
}

model!(Article, "article");

#[derive(Serialize, Deserialize, Debug)]
pub struct Collect {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub image: Vec<String>,
    pub create_at: UTCDateTime,
    pub update_at: UTCDateTime,
}

model!(Collect, "collect");

#[derive(Serialize, Deserialize, Debug)]
pub struct Media {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub filename: String,
    pub filesize: i32,
    pub mime_type: String,
    pub extension: String,
    #[serde(default)]
    pub width: i32,
    #[serde(default)]
    pub height: i32,
    pub hash: String
}

model!(Media, "media");
