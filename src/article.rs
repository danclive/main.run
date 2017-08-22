use sincere::Context;

use mon;
use mon::bson;
use mon::oid::ObjectId;
use mon::bson::bson::UTCDateTime;

use chrono::Utc;

use DB;
use common::{Response as JsonResponse, Empty};
//use error::ErrorCode;

#[allow(dead_code)]
struct Collect {
    id: ObjectId,
    create_at: UTCDateTime,
    update_at: UTCDateTime,
}

#[allow(dead_code)]
struct Article {
    id: ObjectId,
    title: String,
    owner_ids: Vec<ObjectId>,
    attend_ids: Vec<ObjectId>,
    collect_ids: Vec<ObjectId>,
    create_at: UTCDateTime,
    update_at: UTCDateTime,
}

#[allow(dead_code)]
struct Release {
    id: ObjectId,
    article_id: ObjectId,
    content: String,
    create_at: UTCDateTime,
}

pub fn list(context: &mut Context) {
    if let Some(_id) = context.contexts.get("id") {

    }
}

pub fn detail(context: &mut Context) {
    if let Some(_id) = context.request.get_param("id") {

    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct New {
    title: String,
    content: String
}

pub fn new(context: &mut Context) {
    let id = context.contexts.get("id").unwrap();

    let request = &context.request;

    let article_col = DB.collection("article");
    let release_col = DB.collection("release");

    let result = || {

        let new_json = request.bind_json::<New>()?;

        let article = doc!{
            "_id" => (ObjectId::new().unwrap()),
            "title" => (new_json.title),
            "owner_ids" => [ObjectId::with_string(id).unwrap()],
            "attend_ids" => [],
            "collect_ids" => [],
            "create_at" => (bson::Bson::from(Utc::now())),
            "update_at" => (bson::Bson::from(Utc::now()))
        };

        let article_id = article.get_object_id("_id").unwrap().clone();

        let release = doc!{
            "_id" => (ObjectId::new().unwrap()),
            "article_id" => (article_id),
            "content" => (new_json.content),
            "create_at" => (bson::Bson::from(Utc::now()))
        };

        let insert_result = release_col.insert_one(release, None).and(article_col.insert_one(article, None))?;

        if let Some(exception) = insert_result.write_exception {
            return Err(mon::error::Error::WriteError(exception).into());
        }

        Ok(JsonResponse::<Empty>::success())
    };

    match result() {
        Ok(result) => {
            context.response.from_json(result).unwrap();
        },
        Err(err) => {
            context.response.from_json(JsonResponse::<Empty>::from_error(err)).unwrap();
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Commit {
    id: i64,
    title: String,
    content: String
}

pub fn commit(context: &mut Context) {
    let _id = context.contexts.get("id").unwrap();
}
