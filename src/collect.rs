use std::i64;
use std::str::FromStr;

use sincere::app::context::Context;
use sincere::app::Group;

use mongors::collection::options::FindOptions;
use mongors::object_id::ObjectId;
use mongors::bson::Bson;

use chrono::Utc;
use chrono::Local;

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

        let mut collect_find_option = FindOptions::default();

        collect_find_option.limit = Some(per_page);
        collect_find_option.skip = Some((page - 1) * per_page);

        let collects = model::Collect::find(doc!{}, Some(collect_find_option))?;

        let collect_count = model::Collect::count(doc!{}, None)?;

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
            description: String,
            image: Vec<String>
        }

        let new_json = context.request.bind_json::<New>()?;

        let collect = model::Collect {
            id: ObjectId::new()?,
            name: new_json.name,
            description: new_json.description,
            image: new_json.image,
            create_at: Utc::now().into(),
            update_at: Utc::now().into()
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
            description: String,
            image: Vec<String>
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

                doc.save()?;

                let return_json = json!({
                    "collect_id": collect_id
                });

                Ok(Response::success(Some(return_json)))
            }
        }
    }});

    /*
    hand!(push, {|context: &mut Context| {
        let collect_id = context.request.param("id").unwrap();

        #[derive(Deserialize, Debug)]
        struct Push {
            articles: Vec<String>
        }

        let push_json = context.request.bind_json::<Push>()?;

        let collect = model::Collect::find_by_id(ObjectId::with_string(&collect_id)?, None, None)?;

        match collect {
            Some(mut collect) => {
                for article in push_json.articles {
                    let article_id = ObjectId::with_string(&article)?;

                    if model::Article::count(Some(doc!{"_id": (article_id.clone()), "status": 0}), None)? > 0 {
                        if !collect.articles_id.contains(&article_id) {
                            collect.articles_id.push(article_id);
                        }
                    }
                }

                collect.update_at = Utc::now().into();
                collect.save(None)?;
            },
            None => return Err(ErrorCode(10004).into())
        }

        Ok(Response::<Empty>::success(None))
    }});

    hand!(remove, {|context: &mut Context| {
        let collect_id = context.request.param("id").unwrap();

        #[derive(Deserialize, Debug)]
        struct Remove {
            articles: Vec<String>
        }

        let remove_json = context.request.bind_json::<Remove>()?;

        let collect = model::Collect::find_by_id(ObjectId::with_string(&collect_id)?, None, None)?;

        match collect {
            Some(mut collect) => {
                collect.articles_id = collect.articles_id.into_iter().filter(|id|{ !remove_json.articles.contains(&id.to_hex()) }).collect();

                collect.update_at = Utc::now().into();
                collect.save(None)?;
            },
            None => return Err(ErrorCode(10004).into())
        }

        Ok(Response::<Empty>::success(None))
    }});

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

    hand!(push, {|context: &mut Context| {
        let collect_id = context.request.param("id").unwrap();

        let collect_id = ObjectId::with_string(&collect_id)?;

        #[derive(Deserialize, Debug)]
        struct Push {
            articles: Vec<String>
        }

        let push_json = context.request.bind_json::<Push>()?;

        let mut articles_objectid = Vec::new();

        for id in push_json.articles {
            articles_objectid.push(Bson::ObjectId(ObjectId::with_string(&id)?));
        }

        let article_find = doc!{
            "status": 0,
            "_id" => { "$in": articles_objectid}
        };

        let articles = model::Article::find(article_find, None)?;

        for mut article in articles {

            if !article.collect_ids.contains(&collect_id) {
                article.collect_ids.push(collect_id.clone());
            }

            article.save()?;
        }

        Ok(Response::<Empty>::success(None))
    }});

    hand!(remove, {|context: &mut Context| {
        let collect_id = context.request.param("id").unwrap();

        let collect_id = ObjectId::with_string(&collect_id)?;

        #[derive(Deserialize, Debug)]
        struct Remove {
            articles: Vec<String>
        }

        let remove_json = context.request.bind_json::<Remove>()?;

        let mut articles_objectid = Vec::new();

        for id in remove_json.articles {
            articles_objectid.push(Bson::ObjectId(ObjectId::with_string(&id)?));
        }

        let article_find = doc!{
            "status": 0,
            "collect_ids" => { "$in": articles_objectid}
        };

        let articles = model::Article::find(article_find, None)?;

        for mut article in articles {
            article.collect_ids = article.collect_ids.into_iter().filter(|id| id != &collect_id).collect();

            article.save()?;
        }

        Ok(Response::<Empty>::success(None))
    }});

    hand!(articles, {|context: &mut Context| {
        let collect_id = context.request.param("id").unwrap();

        let article_find = doc!{
            "status": 0,
            "collect_ids": ObjectId::with_string(&collect_id)?
        };

        let articles = model::Article::find(article_find, None)?;

        let mut articles_json = Vec::new();

        for article in articles {
            articles_json.push(json!({
                "id": article.id.to_hex(),
                "title": article.title,
                "image": article.image,
                "summary": article.summary,
                "create_at": article.create_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                "update_at": article.update_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string()
            }));
        }

        let return_json = json!({
            "articles": articles_json
        });


        Ok(Response::success(Some(return_json)))
    }});

    pub fn handle() -> Group {
        let mut group = Group::new("/collect");

        group.get("/", Self::collects);
        group.get("/{id:[a-z0-9]{24}}", Self::detail);
        group.post("/", Self::new).before(middleware::auth);
        group.put("/{id:[a-z0-9]{24}}", Self::update).before(middleware::auth);
        group.put("/{id:[a-z0-9]{24}}/push", Self::push).before(middleware::auth);
        group.put("/{id:[a-z0-9]{24}}/remove", Self::remove).before(middleware::auth);
        group.get("/{id:[a-z0-9]{24}}/articles", Self::articles);

        group
    }
}
