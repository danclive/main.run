use serde::Serialize;
use serde::de::DeserializeOwned;

use mongors::bson::{self, Bson, Document};
use mongors::database::Database;
use mongors::bson::encode::EncodeError;
use mongors::collection::options::{FindOptions, UpdateOptions, CountOptions, AggregateOptions, DistinctOptions};
use mongors::collection::results::UpdateResult;
use mongors::error::Error::WriteError;
use mongors::object_id::ObjectId;

use error::Result;

pub trait StructDocument: Serialize + DeserializeOwned {

    const NAME: &'static str;

    fn get_database() -> Database;

    fn to_document(&self) -> Result<Document> {
        let bson = bson::encode::to_bson(self)?;

        if let bson::Bson::Document(doc) = bson {
            Ok(doc)
        } else {
            Err(EncodeError::Unknown("can't encode struct to document".to_string()).into())
        }
    }

    fn from_document(doc: Document) -> Result<Self> {
        Ok(bson::decode::from_bson(bson::Bson::Document(doc))?)
    }

    fn save(&self) -> Result<UpdateResult> {
        let database = Self::get_database();

        let doc = self.to_document()?;

        let id = doc.get_object_id("_id")?.clone();

        let options = UpdateOptions {
            upsert: Some(true),
            ..Default::default()
        };

        let result = database.collection(Self::NAME).replace_one(doc!{"_id": id}, doc, Some(options))?;

        if let Some(exception) = result.write_exception {
            return Err(WriteError(exception).into());
        }

        Ok(result)
    }

    fn find_by_id<O>(id: O, filter: Option<Document>, options: Option<FindOptions>) -> Result<Option<Self>>
        where O: Into<Bson>
    {
        let database = Self::get_database();

        let mut filter_doc = doc!{"_id": id.into()};

        if let Some(filter) = filter {
            filter_doc.extend(filter);
        }

        match database.collection(Self::NAME).find_one(filter_doc, options)? {
            Some(doc) => Ok(Some(Self::from_document(doc)?)),
            None => Ok(None)
        }
    }

    fn find_one(filter: Document, options: Option<FindOptions>) -> Result<Option<Self>> {
        let database = Self::get_database();

        match database.collection(Self::NAME).find_one(filter, options)? {
            Some(doc) => Ok(Some(Self::from_document(doc)?)),
            None => Ok(None)
        }
    }

    fn find(filter: Document, options: Option<FindOptions>) -> Result<Vec<Self>> {
        let database = Self::get_database();

        let doc_find = database.collection(Self::NAME).find(filter, options)?;

        let mut docs = Vec::new();

        for item in doc_find {
            let doc = item?;

            docs.push(Self::from_document(doc)?);
        }

        Ok(docs)
    }

    fn count(filter: Document, options: Option<CountOptions>) -> Result<i64> {
        let database = Self::get_database();

        Ok(database.collection(Self::NAME).count(filter, options)?)
    }

    fn aggregate(pipeline: Vec<Document>, options: Option<AggregateOptions>) -> Result<Vec<Document>> {
        let database = Self::get_database();

        let aggregate_find = database.collection(Self::NAME).aggregate(pipeline, options)?;

        let mut docs = Vec::new();

        for item in aggregate_find {
            let doc = item?;

            docs.push(doc);
        }

        Ok(docs)
    }

    fn distinct(field_name: &str, filter: Document, options: Option<DistinctOptions>) -> Result<Vec<Bson>> {
        let database = Self::get_database();

        Ok(database.collection(Self::NAME).distinct(field_name, filter, options)?)
    }

    fn patch(id: ObjectId, doc: Document) -> Result<Option<Document>> {
        let database = Self::get_database();

        let result = database.collection(Self::NAME).find_one_and_update(doc!{"_id": id}, doc!{"$set": doc}, None)?;

        Ok(result)
    }
}
