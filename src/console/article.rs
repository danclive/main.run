use std::i64;
use std::str::FromStr;
use std::collections::HashMap;

use sincere::app::context::Context;
use sincere::app::Group;

use mongors::collection::options::FindOptions;
use mongors::object_id::ObjectId;
use mongors::{doc, bson};

use chrono::Utc;
use chrono::Local;

use serde_json::Value;
use serde_json::json;
use serde_derive::Deserialize;

use crate::common::{Response, Empty};
use crate::middleware;
use crate::model;
use crate::struct_document::StructDocument;
use crate::error::ErrorCode;

pub struct Article;

impl Article {

    hand!(articles, {|context:  &mut Context| {
        let page = context.request.query("page").unwrap_or("1".to_owned());
        let per_page = context.request.query("per_page").unwrap_or("10".to_owned());
        let status = context.request.query("status");

        let page = i64::from_str(&page)?;
        let per_page = i64::from_str(&per_page)?;

        let mut article_find = doc!{
            "status": { "$lte": 2 }
        };

        if let Some(status) = status {
            article_find.insert("status", i32::from_str(&status)?);
        }

        let mut article_find_option = FindOptions::default();

        article_find_option.sort = Some(doc!{
            "_id": -1
        });

        article_find_option.limit = Some(per_page);
        article_find_option.skip = Some((page - 1) * per_page);

        let articles = model::Article::find(article_find.clone(), Some(article_find_option))?;

        let articles_count = model::Article::count(article_find, None)?;

        let mut articles_json = Vec::new();

        for article in articles {
            let mut article_json = json!({
                "id": article.id.to_hex(),
                "title": article.title,
                "image": article.image,
                "summary": article.summary,
                "status": article.status,
                "create_at": article.create_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                "update_at": article.update_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string()
            });

            if !article.collect_ids.is_empty() {
                let collects = model::Collect::find(doc!{"_id": {"$in": article.collect_ids}, "status": 0}, None)?;

                let mut collect_json = Vec::new();

                for collect in  collects {
                    collect_json.push(json!({
                        "id": collect.id.to_hex(),
                        "name": collect.name
                    }))
                }

                article_json["collects"] = json!(collect_json);
            }

            articles_json.push(article_json);
        }

        let return_json = json!({
            "articles": articles_json,
            "count": articles_count
        });

        Ok(Response::success(Some(return_json)))
    }});

    hand!(detail, {|context: &mut Context| {
        let article_id = context.request.param("id").unwrap();

        let article_find = doc!{
            "_id": ObjectId::with_string(&article_id)?
        };

        let article = model::Article::find_one(article_find, None)?;

        match article {
            None => return Err(ErrorCode(10004).into()),
            Some(doc) => {
                let mut return_json = json!({
                    "id": doc.id.to_hex(),
                    "title": doc.title,
                    "image": doc.image,
                    "content": doc.content,
                    "summary": doc.summary,
                    "status": doc.status,
                    "create_at": doc.create_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                    "update_at": doc.update_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string()
                });

                if !doc.collect_ids.is_empty() {
                    let collects = model::Collect::find(doc!{"_id": {"$in": doc.collect_ids}, "status": 0}, None)?;

                    let mut collect_json = Vec::new();

                    for collect in  collects {
                        collect_json.push(json!({
                            "id": collect.id.to_hex(),
                            "name": collect.name
                        }))
                    }

                    return_json["collects"] = json!(collect_json);
                }

                Ok(Response::success(Some(return_json)))
            }
        }
    }});

    hand!(new, {|context: &mut Context| {
        let user_id = context.contexts.get_str("id").unwrap_or("");

        #[derive(Deserialize, Debug)]
        struct New {
            title: String,
            #[serde(default)]
            image: Vec<String>,
            content: String,
            #[serde(default)]
            summary: String,
            #[serde(default)]
            collect_ids: Vec<String>,
            #[serde(default)]
            status: i32
        }

        let new_json = context.request.bind_json::<New>()?;

        if new_json.status == 3 {
            return Err(ErrorCode(10005).into())
        }

        let collect_ids = {
            let mut temp = Vec::new();

            for o in new_json.collect_ids {
                temp.push(ObjectId::with_string(&o)?);
            }

            temp
        };

        let article = model::Article {
            id: ObjectId::new()?,
            title: new_json.title,
            image: new_json.image,
            author_id: ObjectId::with_string(&user_id)?,
            collect_ids: collect_ids,
            content: new_json.content,
            summary: new_json.summary,
            create_at: Utc::now().into(),
            update_at: Utc::now().into(),
            status: new_json.status
        };

        article.save()?;

        let return_json = json!({
            "article_id": article.id.to_hex()
        });

        Ok(Response::success(Some(return_json)))
    }});

    hand!(update, {|context: &mut Context| {
        let article_id = context.request.param("id").unwrap();

        #[derive(Deserialize, Debug)]
        struct Update {
            title: String,
            #[serde(default)]
            image: Vec<String>,
            content: String,
            #[serde(default)]
            summary: String,
            #[serde(default)]
            collect_ids: Vec<String>,
            #[serde(default)]
            status: i32
        }

        let update_json = context.request.bind_json::<Update>()?;

        let collect_ids = {
            let mut temp = Vec::new();

            for o in update_json.collect_ids {
                temp.push(ObjectId::with_string(&o)?);
            }

            temp
        };

        let article_find = doc!{
            "_id": (ObjectId::with_string(&article_id)?)
        };

        let article = model::Article::find_one(article_find, None)?;

        match article {
            None => return Err(ErrorCode(10004).into()),
            Some(mut doc) => {
                doc.title = update_json.title;
                doc.image = update_json.image;
                doc.content = update_json.content;
                doc.summary = update_json.summary;
                doc.collect_ids = collect_ids;
                doc.update_at = Utc::now().into();
                doc.status = update_json.status;

                doc.save()?;

                let return_json = json!({
                    "article_id": article_id
                });

                Ok(Response::success(Some(return_json)))
            }
        }
    }});

    // patch
    hand!(patch, {|context: &mut Context| {
        let article_id = context.request.param("id").unwrap();

        #[derive(Deserialize, Debug)]
        struct Patch {
            #[serde(default)]
            title: String,
            #[serde(default)]
            image: Vec<String>,
            #[serde(default)]
            content: String,
            #[serde(default)]
            summary: String,
            #[serde(default)]
            collect_ids: Vec<String>,
            #[serde(flatten)]
            extra: HashMap<String, Value>
        }

        let patch_json = context.request.bind_json::<Patch>()?;

        let mut patch_doc = doc!{};

        if !patch_json.title.is_empty() {
            patch_doc.insert("title", patch_json.title);
        }

        if !patch_json.image.is_empty() {
            patch_doc.insert("image", patch_json.image);
        }

        if !patch_json.content.is_empty() {
            patch_doc.insert("content", patch_json.content);
        }

        if !patch_json.summary.is_empty() {
            patch_doc.insert("summary", patch_json.summary);
        }

        if !patch_json.collect_ids.is_empty() {
            patch_doc.insert("collect_ids", patch_json.collect_ids);
        }

        if let Some(Value::Number(status)) = patch_json.extra.get("status") {
            if let Some(status) = status.as_i64() {
                patch_doc.insert("status", status as i32);
            }
        }

        if !patch_doc.is_empty() {
            patch_doc.insert("update_at", Utc::now());
        }

        if None == model::Article::patch(ObjectId::with_string(&article_id)?, patch_doc)? {
            return Err(ErrorCode(10004).into())
        }

        let return_json = json!({
            "article_id": article_id
        });

        Ok(Response::success(Some(return_json)))
    }});

    // delete

    pub fn handle() -> Group {
        let mut group = Group::new("/console/article");

        group.get("/", Self::articles);
        group.get("/{id:[a-z0-9]{24}}", Self::detail);
        group.post("/", Self::new).before(middleware::auth);
        group.put("/{id:[a-z0-9]{24}}", Self::update).before(middleware::auth);
        group.patch("/{id:[a-z0-9]{24}}", Self::patch).before(middleware::auth);

        group
    }
}
