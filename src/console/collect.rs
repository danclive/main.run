use std::i64;
use std::str::FromStr;
use std::collections::HashMap;

use sincere::app::context::Context;
use sincere::app::Group;

use mongors::collection::options::FindOptions;
use mongors::object_id::ObjectId;

use chrono::Utc;
use chrono::Local;

use serde_json::Value;

use common::{Response, Empty};
use middleware;
use model;
use struct_document::StructDocument;
use error::ErrorCode;

pub struct Collect;

impl Collect {

    hand!(collects, {|context: &mut Context| {
        let page = context.request.query("page").unwrap_or("1".to_owned());
        let per_page = context.request.query("per_page").unwrap_or("10".to_owned());

        let page = i64::from_str(&page)?;
        let per_page = i64::from_str(&per_page)?;

        let collect_find = doc!{
            "status": { "$lte": 2 }
        };

        let mut collect_find_option = FindOptions::default();

        collect_find_option.sort = Some(doc!{
            "_id": -1
        });

        collect_find_option.limit = Some(per_page);
        collect_find_option.skip = Some((page - 1) * per_page);

        let collects = model::Collect::find(collect_find.clone(), Some(collect_find_option))?;

        let collect_count = model::Collect::count(collect_find, None)?;

        let mut collects_json = Vec::new();

        for collect in collects {
            collects_json.push(json!({
                "id": collect.id.to_hex(),
                "name": collect.name,
                "description": collect.description,
                "image": collect.image,
                "create_at": collect.create_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                "update_at": collect.update_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string()
            }));
        }

        let return_json = json!({
            "collects": collects_json,
            "count": collect_count
        });

        Ok(Response::success(Some(return_json)))
    }});

    hand!(detail, {|context: &mut Context| {
        let collect_id = context.request.param("id").unwrap();

        let collect = model::Collect::find_by_id(ObjectId::with_string(&collect_id)?, None, None)?;

        match collect {
            None => return Err(ErrorCode(10004).into()),
            Some(doc) => {
                let return_json = json!({
                    "id": doc.id.to_hex(),
                    "name": doc.name,
                    "description": doc.description,
                    "image": doc.image,
                    "create_at": doc.create_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                    "update_at": doc.update_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string()
                });

                Ok(Response::success(Some(return_json)))
            }
        }
    }});

    hand!(new, {|context: &mut Context| {
        #[derive(Deserialize, Debug)]
        struct New {
            name: String,
            #[serde(default)]
            description: String,
            #[serde(default)]
            image: Vec<String>,
            #[serde(default)]
            status: i32
        }

        let new_json = context.request.bind_json::<New>()?;

        if new_json.status == 3 {
            return Err(ErrorCode(10005).into())
        }

        let collect = model::Collect {
            id: ObjectId::new()?,
            name: new_json.name,
            description: new_json.description,
            image: new_json.image,
            create_at: Utc::now().into(),
            update_at: Utc::now().into(),
            status: new_json.status
        };

        collect.save()?;

        let return_json = json!({
            "collect_id": collect.id.to_hex()
        });

        Ok(Response::success(Some(return_json)))
    }});

    hand!(update, {|context: &mut Context| {
        let collect_id = context.request.param("id").unwrap();

        #[derive(Deserialize, Debug)]
        struct Update {
            name: String,
            #[serde(default)]
            description: String,
            #[serde(default)]
            image: Vec<String>,
            #[serde(default)]
            status: i32
        }

        let update_json = context.request.bind_json::<Update>()?;

        let collect_find = doc!{
            "_id": (ObjectId::with_string(&collect_id)?)
        };

        let collect = model::Collect::find_one(collect_find, None)?;

        match collect {
            None => return Err(ErrorCode(10004).into()),
            Some(mut doc) => {
                doc.name = update_json.name;
                doc.description = update_json.description;
                doc.image = update_json.image;
                doc.update_at = Utc::now().into();
                doc.status = update_json.status;

                doc.save()?;

                let return_json = json!({
                    "collect_id": collect_id
                });

                Ok(Response::success(Some(return_json)))
            }
        }
    }});

    hand!(patch, {|context: &mut Context| {
        let collect_id = context.request.param("id").unwrap();

        #[derive(Deserialize, Debug)]
        struct Patch {
            #[serde(default)]
            name: String,
            #[serde(default)]
            description: String,
            #[serde(default)]
            image: Vec<String>,
            #[serde(flatten)]
            extra: HashMap<String, Value>
        }

        let patch_json = context.request.bind_json::<Patch>()?;

        let mut patch_doc = doc!{};

        if !patch_json.name.is_empty() {
            patch_doc.insert("name", patch_json.name);
        }

        if !patch_json.description.is_empty() {
            patch_doc.insert("description", patch_json.description);
        }

        if !patch_json.image.is_empty() {
            patch_doc.insert("image", patch_json.image);
        }

        if let Some(Value::Number(status)) = patch_json.extra.get("status") {
            if let Some(status) = status.as_i64() {
                patch_doc.insert("status", status as i32);
            }
        }

        if !patch_doc.is_empty() {
            patch_doc.insert("update_at", Utc::now());
        }

        if None == model::Collect::patch(ObjectId::with_string(&collect_id)?, patch_doc)? {
            return Err(ErrorCode(10004).into())
        }

        let return_json = json!({
            "collect_id": collect_id
        });

        Ok(Response::success(Some(return_json)))
    }});

    /*
    hand!(articles, {|context: &mut Context| {

        let collect_id = context.request.param("id").unwrap();

        // db.collect.aggregate([
        //     { $match: { "_id": ObjectId("5a94e0e9643838356ac5b2ce")} },
        //     { $unwind: "$articles_id" },
        //     { $sort: { "articles_id": -1 } },
        //     { $lookup: { from: "article", localField: "articles_id", foreignField: "_id", as: "articles" } },
        //     { $match: { "article.status": 3 } }
        // ])
        let pipeline = vec![
            doc!{ "$match": { "_id": ObjectId::with_string(&collect_id)? } },
            doc!{ "$unwind": "$articles_id" },
            doc!{ "$sort": { "articles_id": -1 } },
            doc!{ "$lookup": { "from": "article", "localField": "articles_id", "foreignField": "_id", "as": "article" } },
            doc!{ "$match": { "article.status": 0 } }
        ];

        let articles = model::Collect::aggregate(pipeline, None)?;

        let mut articles_json = Vec::new();

        for article_item in articles {
            match article_item.get_array("article") {
                Ok(article_array) => {

                    let article = if article_array.len() > 0 {
                        match article_array[0].as_document() {
                            Some(doc) => doc,
                            None => continue
                        }
                    } else {
                        continue
                    };

                    articles_json.push(json!({
                        "id": match article.get_object_id("_id") {
                            Ok(id) => id.to_hex(),
                            Err(_) => continue
                        },
                        "title": article.get_str("title").unwrap_or(""),
                        "image": article.get_array("image").map(|a| 
                            {a.iter().map(|b| b.as_str().unwrap_or_default().to_owned()).collect::<Vec<String>>()}
                        ).unwrap_or(Vec::new()),
                        "author_id": article.get_object_id("author_id").map(|id| id.to_hex()).unwrap_or_default(),
                        //"content": article.get_str("content").unwrap_or_default(),
                        "create_at": article.get_utc_datetime("create_at").map(|date| date.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_default(),
                        "update_at": article.get_utc_datetime("update_at").map(|date| date.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_default()
                    }));
                }
                Err(_) => continue
            }
        }

        let return_json = json!({
            "articles": articles_json
        });

        Ok(Response::success(Some(return_json)))
    }});
    */

    pub fn handle() -> Group {
        let mut group = Group::new("/console/collect");

        group.get("/", Self::collects);
        group.get("/{id:[a-z0-9]{24}}", Self::detail);
        group.post("/", Self::new).before(middleware::auth);
        group.put("/{id:[a-z0-9]{24}}", Self::update).before(middleware::auth);
        group.patch("/{id:[a-z0-9]{24}}", Self::patch).before(middleware::auth);

        group
    }
}
