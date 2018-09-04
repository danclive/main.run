use mongors::bson::Document;
use mongors::object_id::ObjectId;
use chrono::Utc;

use struct_document::StructDocument;
use error::Result;
use super::Article;

impl Article {
    pub fn patch(id: ObjectId, doc: Document) -> Result<Option<Document>> {
        let database = Self::get_database();

        let result = database.collection(Self::NAME).find_one_and_update(doc!{"_id": id}, doc!{"$set": doc}, None)?;

        Ok(result)
    }
}
