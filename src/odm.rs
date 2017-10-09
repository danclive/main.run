use serde::Serialize;
use serde::de::DeserializeOwned;

use mon::bson::{self, Bson, Document};
//use mon::oid::ObjectId;
use mon::db::Database;
use mon::bson::encode::EncodeError;
use mon::coll::options::{FindOptions, UpdateOptions};
use mon::common::WriteConcern;
use mon::coll::results::UpdateResult;


use error::Result;
//use error::ErrorCode;

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

    fn save(&self, write_concern: Option<WriteConcern>) -> Result<UpdateResult> {
        let database = Self::get_database();

        let doc = self.to_document()?;

        let id = doc.get_object_id("_id")?.clone();

        let options = UpdateOptions {
            upsert: Some(true),
            write_concern: write_concern
        };

        Ok(database.collection(Self::NAME).update_one(doc!{"_id": id}, doc, Some(options))?)
    }

    fn find_by_id<O>(id: O, options: Option<FindOptions>) -> Result<Option<Self>>
        where O: Into<Bson>
    {
        let database = Self::get_database();

        match database.collection(Self::NAME).find_one(Some(doc!{"_id": (id.into())}), options)? {
            Some(doc) => Ok(Some(Self::from_document(doc)?)),
            None => Ok(None)
        }
    }

    fn find_one(filter: Option<Document>, options: Option<FindOptions>) -> Result<Option<Self>>
    {
        let database = Self::get_database();

        match database.collection(Self::NAME).find_one(filter, options)? {
            Some(doc) => Ok(Some(Self::from_document(doc)?)),
            None => Ok(None)
        }
    }

    fn find(filter: Option<Document>, options: Option<FindOptions>) -> Result<Vec<Self>>
    {
        let database = Self::get_database();

        let doc_find = database.collection(Self::NAME).find(filter, options)?;

        let mut docs = Vec::new();

        for item in doc_find {
            let doc = item?;

            docs.push(Self::from_document(doc)?);
        }

        Ok(docs)
    }
}

/*
pub fn find_by_id<T, O>(id: O, options: Option<FindOptions>) -> Result<Option<T>>
    where T: StructDocument, O: Into<Bson>
{
    let database = T::get_database();

    match database.collection(T::NAME).find_one(Some(doc!{"_id": (id.into())}), options)? {
        Some(doc) => Ok(Some(T::from_document(doc)?)),
        None => Ok(None)
    }
}

pub fn find_one<T>(filter: Option<Document>, options: Option<FindOptions>) -> Result<Option<T>>
    where T: StructDocument
{
    let database = T::get_database();

    match database.collection(T::NAME).find_one(filter, options)? {
        Some(doc) => Ok(Some(T::from_document(doc)?)),
        None => Ok(None)
    }
}

pub fn find<T>(filter: Option<Document>, options: Option<FindOptions>) -> Result<Vec<T>>
    where T: StructDocument
{
    let database = T::get_database();

    let doc_find = database.collection(T::NAME).find(filter, options)?;

    let mut docs = Vec::new();

    for item in doc_find {
        let doc = item?;

        docs.push(T::from_document(doc)?);
    }

    Ok(docs)
}
*/
