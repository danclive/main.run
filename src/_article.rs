use std::i64;
use std::str::FromStr;

use sincere::Context;
use sincere::Group;

use mon;
use mon::bson::Bson;
use mon::oid::ObjectId;
//use mon::bson::bson::UTCDateTime;
use mon::coll::options::FindOptions;

use chrono::Utc;
use chrono::Local;

use DB;
use common::{Response as JsonResponse, Empty};
use error::ErrorCode;
use middleware;

// #[allow(dead_code)]
// struct Article {
//     id: ObjectId,
//     title: String,
//     owner_ids: Vec<ObjectId>,
//     attend_ids: Vec<ObjectId>,
//     collect_ids: Vec<ObjectId>,
//     create_at: UTCDateTime,
//     update_at: UTCDateTime,
// }

// #[allow(dead_code)]
// struct Release {
//     id: ObjectId,
//     article_id: ObjectId,
//     content: String,
//     create_at: UTCDateTime,
// }

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct New {
    title: String,
    content: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Commit {
    content: String
}

pub struct Article;

impl Article {
    pub fn list(context: &mut Context) {
        let page = context.request.get_query("page").unwrap_or("1".to_owned());
        let per_page = context.request.get_query("per_page").unwrap_or("10".to_owned());

        let article_col = DB.collection("article");
        let release_col = DB.collection("release");

        let result = || {
            
            let page = i64::from_str(&page)?;
            let per_page = i64::from_str(&per_page)?;

            let mut article_find_option = FindOptions::default();

            article_find_option.sort = Some(doc!{
                "_id": (-1)
            });

            article_find_option.limit = Some(per_page);
            article_find_option.skip = Some((page - 1) * per_page);

            let article_doc_find = article_col.find(None, Some(article_find_option))?;

            let mut articles = Vec::new();

            for item in article_doc_find {
                let article = item?;

                let release_find = doc!{
                    "article_id": (article.get_object_id("_id")?.to_owned())
                };

                let mut release_find_option = FindOptions::default();

                release_find_option.sort = Some(doc!{
                    "_id": (-1)
                });

                let release_doc_find = release_col.find_one(Some(release_find), Some(release_find_option))?;

                if let None = release_doc_find {
                    return Err(ErrorCode(10004).into());
                }

                let release_doc = release_doc_find.unwrap();

                articles.push(json!({
                    "Id": article.get_object_id("_id")?.to_hex(),
                    "Title": article.get_str("title").unwrap_or_default(),
                    "OwnerIds": article.get_array("owner_ids")?.iter().map(|i| i.as_object_id().unwrap().to_hex()).collect::<Vec<String>>(),
                    "AttendIds": article.get_array("attend_ids")?.iter().map(|i| i.as_object_id().unwrap().to_hex()).collect::<Vec<String>>(),
                    "CreateAt": article.get_utc_datetime("create_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                    "UpdateAt": article.get_utc_datetime("update_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                    "Release": json!({
                        "Id": release_doc.get_object_id("_id")?.to_hex(),
                        "OwnerId": release_doc.get_object_id("owner_id")?.to_hex(),
                        "Content": release_doc.get_str("content").unwrap_or_default(),
                        "CreateAt": release_doc.get_utc_datetime("create_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                    })
                }));
            }

            let article_doc_count = article_col.count(None, None)?;

            let return_json = json!({
                "Articles": articles,
                "Count": article_doc_count
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
        let page = context.request.get_query("page").unwrap_or("1".to_owned());
        let per_page = context.request.get_query("per_page").unwrap_or("10".to_owned());

        let article_col = DB.collection("article");
        let release_col = DB.collection("release");

        let result = || {
            
            let page = i64::from_str(&page)?;
            let per_page = i64::from_str(&per_page)?;

            let article_find = doc!{
                "_id": (ObjectId::with_string(&id)?)
            };

            let article_doc_find = article_col.find_one(Some(article_find), None)?;

            if let None = article_doc_find {
                return Err(ErrorCode(10004).into());
            }

            let article_doc = article_doc_find.unwrap();

            let release_find = doc!{
                "article_id": (ObjectId::with_string(&id)?)
            };

            let mut release_find_option = FindOptions::default();

            release_find_option.sort = Some(doc!{
                "_id": (-1)
            });

            release_find_option.limit = Some(per_page);
            release_find_option.skip = Some((page - 1) * per_page);

            let release_doc_find = release_col.find(Some(release_find), Some(release_find_option))?;
            
            let mut releases = Vec::new();

            for item in release_doc_find {
                let release = item?;

                releases.push(json!({
                    "Id": release.get_object_id("_id")?.to_hex(),
                    "OwnerId": release.get_object_id("owner_id")?.to_hex(),
                    "Content": release.get_str("content").unwrap_or_default(),
                    "CreateAt": release.get_utc_datetime("create_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                }));
            }

            let return_json = json!({
                "Id": id,
                "Title": article_doc.get_str("title").unwrap_or_default(),
                "OwnerIds": article_doc.get_array("owner_ids")?.iter().map(|i| i.as_object_id().unwrap().to_hex()).collect::<Vec<String>>(),
                "AttendIds": article_doc.get_array("attend_ids")?.iter().map(|i| i.as_object_id().unwrap().to_hex()).collect::<Vec<String>>(),
                "CreateAt": article_doc.get_utc_datetime("create_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                "UpdateAt": article_doc.get_utc_datetime("update_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                "Release": releases
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

    pub fn detail_and_release(context: &mut Context) {
        let id = context.request.get_param("id").unwrap();
        let id2 = context.request.get_param("id2").unwrap();

        let article_col = DB.collection("article");
        let release_col = DB.collection("release");

        let result = || {

            let article_find = doc!{
                "_id": (ObjectId::with_string(&id)?)
            };

            let article_doc_find = article_col.find_one(Some(article_find), None)?;

            if let None = article_doc_find {
                return Err(ErrorCode(10004).into());
            }

            let article_doc = article_doc_find.unwrap();

            let release_find = doc!{
                "_id": (ObjectId::with_string(&id2)?),
                "article_id": (ObjectId::with_string(&id)?)
            };

            let release_doc_find = release_col.find_one(Some(release_find), None)?;

            if let None = release_doc_find {
                return Err(ErrorCode(10004).into());
            }

            let release_doc = release_doc_find.unwrap();
            
            let release = json!({
                "Id": release_doc.get_object_id("_id")?.to_hex(),
                "OwnerId": release_doc.get_object_id("owner_id")?.to_hex(),
                "Content": release_doc.get_str("content").unwrap_or_default(),
                "CreateAt": release_doc.get_utc_datetime("create_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
            });

            let return_json = json!({
                "Id": id,
                "Title": article_doc.get_str("title").unwrap_or_default(),
                "OwnerIds": article_doc.get_array("owner_ids")?.iter().map(|i| i.as_object_id().unwrap().to_hex()).collect::<Vec<String>>(),
                "AttendIds": article_doc.get_array("attend_ids")?.iter().map(|i| i.as_object_id().unwrap().to_hex()).collect::<Vec<String>>(),
                "CreateAt": article_doc.get_utc_datetime("create_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                "UpdateAt": article_doc.get_utc_datetime("update_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                "Release": release
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

        let article_col = DB.collection("article");
        let release_col = DB.collection("release");

        let result = || {

            let new_json = request.bind_json::<New>()?;

            let article_id = ObjectId::new()?;
            let article = doc!{
                "_id": (article_id.clone()),
                "title": (new_json.title),
                "owner_ids": [ObjectId::with_string(user_id)?],
                "attend_ids": [],
                "collect_ids": [],
                "create_at": (Bson::from(Utc::now())),
                "update_at": (Bson::from(Utc::now()))
            };

            let release_id = ObjectId::new()?;
            let release = doc!{
                "_id": (release_id.clone()),
                "article_id": (article_id.clone()),
                "owner_id": (ObjectId::with_string(user_id)?),
                "content": (new_json.content),
                "create_at": (Bson::from(Utc::now()))
            };

            let insert_result = release_col.insert_one(release.clone(), None).and(article_col.insert_one(article, None))?;

            if let Some(exception) = insert_result.write_exception {
                return Err(mon::error::Error::WriteError(exception).into());
            }

            let return_json = json!({
                "ArticleId": article_id.to_hex(),
                "ReleaseId": release_id.to_hex()
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

    pub fn commit(context: &mut Context) {
        let user_id = context.contexts.get("id").unwrap().as_str().unwrap();
        let article_id = context.request.get_param("id").unwrap();

        let request = &mut context.request;

        let article_col = DB.collection("article");
        let release_col = DB.collection("release");

        let result = || {

            let commit_json = request.bind_json::<Commit>()?;

            let article_find = doc!{
                "_id": (ObjectId::with_string(&article_id)?)
            };

            let article_doc_find = article_col.find_one(Some(article_find), None)?;

            if let None = article_doc_find {
                return Err(ErrorCode(10004).into());
            }

            let article_doc = article_doc_find.unwrap();

            let owner_ids = article_doc.get_array("owner_ids").unwrap();
            let attend_ids = article_doc.get_array("attend_ids").unwrap();

            let attend_ids_clone = &mut attend_ids.clone();

            let user_id = ObjectId::with_string(user_id)?;

            if let None = owner_ids.iter().find(|r| **r == Bson::ObjectId(user_id.clone()) ) {
                if let None = attend_ids.iter().find(|r| **r == Bson::ObjectId(user_id.clone()) ) {
                    attend_ids_clone.push(Bson::ObjectId(user_id.clone()));
                }
            }

            let release_id = ObjectId::new()?;
            let release = doc!{
                "_id": (release_id.clone()),
                "article_id": (ObjectId::with_string(&article_id)?),
                "owner_id": user_id,
                "content": (commit_json.content),
                "create_at": (Bson::from(Utc::now()))
            };

            let article_update_filter = doc!{
                "_id": (ObjectId::with_string(&article_id)?)
            };

            let article_update = doc!{
                "$set": {
                    "attend_ids": (attend_ids_clone.to_vec()),
                    "update_at": (Bson::from(Utc::now()))
                }
            };

            let insert_result = release_col.insert_one(release.clone(), None).and(article_col.update_one(article_update_filter, article_update, None))?;

            if let Some(exception) = insert_result.write_exception {
                return Err(mon::error::Error::WriteError(exception).into());
            }

            let return_json = json!({
                "ArticleId": article_id,
                "ReleaseId": release_id.to_hex()
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

    pub fn handle() -> Group {
        let mut group = Group::new("/article");

        group.get("/", Article::list);
        group.get("/{id:[a-z0-9]{24}}", Article::detail);
        group.get("/{id:[a-z0-9]{24}}/release/{id2:[a-z0-9]{24}}", Article::detail_and_release);
        group.post("/", Article::new).before(middleware::auth);
        group.put("/{id:[a-z0-9]{24}}/release", Article::commit).before(middleware::auth);

        group
    }
}
