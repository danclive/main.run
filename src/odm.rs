use serde::Serialize;
use serde::de::DeserializeOwned;

use mon::bson::{self, Bson, Document};
//use mon::oid::ObjectId;
use mon::db::Database;
use mon::bson::encode::EncodeError;
use mon::coll::options::FindOptions;


use error::Result;
use error::ErrorCode;

pub trait StructDocument: Serialize + DeserializeOwned {

    const NAME: &'static str;

    fn get_database() -> Database;

    fn to_document(&self) -> Result<Document> {
        let bson = bson::encode::to_bson(self)?;

        if let bson::Bson::Document(doc) = bson {
            Ok(doc)
        } else {
            Err(EncodeError::Unknown("can't encode object to document".to_string()).into())
        }
    }

    fn from_document(doc: Document) -> Result<Self> {
        Ok(bson::decode::from_bson(bson::Bson::Document(doc))?)
    }
}

pub fn find_by_id<T, O>(id: O, options: Option<FindOptions>) -> Result<Option<T>>
    where T: StructDocument, O: Into<Bson>
{
    let database = T::get_database();

    let doc_find = database.collection(T::NAME).find_one(Some(doc!{"_id": (id.into())}), options)?;

    if let None = doc_find {
        return Ok(None)
    }

    Ok(Some(T::from_document(doc_find.unwrap())?))
}

pub fn find_one<T>(filter: Option<Document>, options: Option<FindOptions>) -> Result<Option<T>>
    where T: StructDocument
{
    let database = T::get_database();

    let doc_find = database.collection(T::NAME).find_one(filter, options)?;

    if let None = doc_find {
        return Ok(None)
    }

    Ok(Some(T::from_document(doc_find.unwrap())?))
}
