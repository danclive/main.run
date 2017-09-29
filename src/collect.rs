use std::i64;
use std::str::FromStr;

use sincere::Group;
use sincere::Context;

use mon;
use mon::oid::ObjectId;
use mon::bson::Bson;
use mon::coll::options::FindOptions;

use chrono::Utc;
use chrono::Local;

use DB;
use common::{Response as JsonResponse, Empty};
use error::ErrorCode;
use middleware;

// #[allow(dead_code)]
// struct Collect {
//     id: ObjectId,
//     title: String,
//     description: String,
//     owner_ids: Vec<ObjectId>,
//     create_at: UTCDateTime,
//     update_at: UTCDateTime,
// }

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct New {
    title: String,
    description: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Include {
    articles: Vec<String>
}

pub struct Collect;

impl Collect {
    pub fn list(context: &mut Context) {
        let page = context.request.get_query("page").unwrap_or("1".to_owned());
        let per_page = context.request.get_query("per_page").unwrap_or("10".to_owned());

        let collect_col = DB.collection("collect");

        let result = || {

            let page = i64::from_str(&page)?;
            let per_page = i64::from_str(&per_page)?;

            let mut collect_find_option = FindOptions::default();

            collect_find_option.sort = Some(doc!{
                "_id": (-1)
            });

            collect_find_option.limit = Some(per_page);
            collect_find_option.skip = Some((page - 1) * per_page);

            let collect_doc_find = collect_col.find(None, Some(collect_find_option))?;

            let mut collects = Vec::new();

            for item in collect_doc_find {
                let collect = item?;

                collects.push(json!({
                    "Id": collect.get_object_id("_id")?.to_hex(),
                    "Title": collect.get_str("title").unwrap_or_default(),
                    "Description": collect.get_str("description").unwrap_or_default(),
                    "OwnerIds": collect.get_array("owner_ids")?.iter().map(|i| i.as_object_id().unwrap().to_hex()).collect::<Vec<String>>(),
                    "CreateAt": collect.get_utc_datetime("create_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                    "UpdateAt": collect.get_utc_datetime("update_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string()
                }));
            }

            let collect_doc_count = collect_col.count(None, None)?;

            let return_json = json!({
                "Collects": collects,
                "Count": collect_doc_count
            });

            Ok(JsonResponse::success(Some(return_json)))
        };

        match result() {
            Ok(result) => {
                context.response.from_json(result).unwrap();
            },
            Err(err) => {
                context.response.from_json(JsonResponse::<Empty>::error(err)).unwrap();
            }
        }
    }

    pub fn detail(context: &mut Context) {
        let id = context.request.get_param("id").unwrap();

        let collect_col = DB.collection("collect");
        let article_col = DB.collection("article");

        let result = || {

            let collect_find = doc!{
                "_id": (ObjectId::with_string(&id)?)
            };

            let collect_doc_find = collect_col.find_one(Some(collect_find), None)?;

            if let None = collect_doc_find {
                return Err(ErrorCode(10004).into());
            }

            let collect_doc = collect_doc_find.unwrap();

            let article_find = doc!{
                "collect_ids": (ObjectId::with_string(&id)?)
            };

            let mut article_find_option = FindOptions::default();

            article_find_option.sort = Some(doc!{
                "_id": (-1)
            });

            let article_doc_find = article_col.find(Some(article_find), Some(article_find_option))?;

            let mut articles = Vec::new();

            for item in article_doc_find {
                let article = item?;

                articles.push(json!({
                    "Id": article.get_object_id("_id")?.to_hex(),
                    "Title": article.get_str("title").unwrap_or_default(),
                    "OwnerIds": article.get_array("owner_ids")?.iter().map(|i| i.as_object_id().unwrap().to_hex()).collect::<Vec<String>>(),
                    "AttendIds": article.get_array("attend_ids")?.iter().map(|i| i.as_object_id().unwrap().to_hex()).collect::<Vec<String>>(),
                    "CreateAt": article.get_utc_datetime("create_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                    "UpdateAt": article.get_utc_datetime("update_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string()
                }));
            }

            let return_json = json!({
                "Id": collect_doc.get_object_id("_id")?.to_hex(),
                "Title": collect_doc.get_str("title").unwrap_or_default(),
                "Description": collect_doc.get_str("description").unwrap_or_default(),
                "OwnerIds": collect_doc.get_array("owner_ids")?.iter().map(|i| i.as_object_id().unwrap().to_hex()).collect::<Vec<String>>(),
                "CreateAt": collect_doc.get_utc_datetime("create_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                "UpdateAt": collect_doc.get_utc_datetime("update_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                "Articles": articles 
            });

            Ok(JsonResponse::success(Some(return_json)))
        };

        match result() {
            Ok(result) => {
                context.response.from_json(result).unwrap();
            },
            Err(err) => {
                context.response.from_json(JsonResponse::<Empty>::error(err)).unwrap();
            }
        }
    }

    pub fn new(context: &mut Context) {
        let user_id = context.contexts.get("id").unwrap().as_str().unwrap();

        let request = &context.request;

        let collect_col = DB.collection("collect");
        
        let result = || {

            let new_json = request.bind_json::<New>()?;

            let collect_id = ObjectId::new()?;
            let collect = doc!{
                "_id": (collect_id.clone()),
                "title": (new_json.title),
                "description": (new_json.description),
                "owner_ids": [ObjectId::with_string(user_id)?],
                "create_at": (Bson::from(Utc::now())),
                "update_at": (Bson::from(Utc::now()))
            };

            let insert_result = collect_col.insert_one(collect, None)?;

            if let Some(exception) = insert_result.write_exception {
                return Err(mon::error::Error::WriteError(exception).into());
            }

            let return_json = json!({
                "CollectId": collect_id.to_hex()
            });

            Ok(JsonResponse::success(Some(return_json)))
        };

        match result() {
            Ok(result) => {
                context.response.from_json(result).unwrap();
            },
            Err(err) => {
                context.response.from_json(JsonResponse::<Empty>::error(err)).unwrap();
            }
        }
    }

    pub fn include(context: &mut Context) {
        let user_id = context.contexts.get("id").unwrap().as_str().unwrap();
        let collect_id = context.request.get_param("id").unwrap();

        let request = &mut context.request;

        let collect_col = DB.collection("collect");
        let article_col = DB.collection("article");

        let result = || {

            let include_json = request.bind_json::<Include>()?;

            let collect_find = doc!{
                "_id": (ObjectId::with_string(&collect_id)?)
            };

            let collect_doc_find = collect_col.find_one(Some(collect_find), None)?;

            if let None = collect_doc_find {
                return Err(ErrorCode(10004).into());
            }

            let collect_doc = collect_doc_find.unwrap();

            if let None = collect_doc.get_array("owner_ids")?.iter().find( |o| **o == Bson::ObjectId(ObjectId::with_string(&user_id).unwrap()) ) {
                return Err(ErrorCode(10005).into());
            }

            let collect_update_filter = doc!{
                "_id": (ObjectId::with_string(&collect_id)?)
            };

            let collect_update = doc!{
                "$set": {
                    "update_at": (Bson::from(Utc::now()))
                }
            };

            let update_result = collect_col.update_one(collect_update_filter, collect_update, None)?;

            if let Some(exception) = update_result.write_exception {
                return Err(mon::error::Error::WriteError(exception).into());
            }

            let article_doc_find = doc!{
                "_id": {
                    "$in": (include_json.articles.iter().map(|s| Bson::ObjectId(ObjectId::with_string(s).unwrap()) ).collect::<Vec<Bson>>())
                }
            };

            let article_doc_find = article_col.find(Some(article_doc_find), None)?;

            for item in article_doc_find {
                let article = item?;

                let mut collects = article.get_array("collect_ids")?.clone();

                let collects_bson_objectid = Bson::ObjectId(collect_doc.get_object_id("_id")?.clone());

                if let None = collects.iter().find(|a| **a == collects_bson_objectid ) {
                    collects.push(collects_bson_objectid.clone())
                }

                let article_update_filter = doc!{
                    "_id": (article.get_object_id("_id")?.clone())
                };

                let article_update = doc!{
                    "$set": {
                        "collect_ids": collects
                    }
                };

                let update_result = article_col.update_one(article_update_filter, article_update, None)?;

                if let Some(exception) = update_result.write_exception {
                    return Err(mon::error::Error::WriteError(exception).into());
                }
            }

            Ok(JsonResponse::<Empty>::success(None))
        };

        match result() {
            Ok(result) => {
                context.response.from_json(result).unwrap();
            },
            Err(err) => {
                context.response.from_json(JsonResponse::<Empty>::error(err)).unwrap();
            }
        }
    }

    pub fn handle() -> Group {
        let mut group = Group::new("/collect");

        group.get("/", Collect::list);
        group.get("/{id:[a-z0-9]{24}}", Collect::detail);
        group.post("/", Collect::new).before(middleware::auth);
        group.put("/{id:[a-z0-9]{24}}", Collect::include).before(middleware::auth);

        group
    }
}
