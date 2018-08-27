use std::i64;
use std::str::FromStr;
use std::cmp;

use sincere::app::context::Context;
use sincere::app::Group;

use mongors::collection::options::FindOptions;
use mongors::object_id::ObjectId;

use chrono::Utc;
use chrono::Local;

use string2::String2;

use common::{Response, Empty};
use middleware;
use model;
use struct_document::StructDocument;
use error::ErrorCode;

pub struct Article;

impl Article {

    hand!(articles, {|context:  &mut Context| {
        let page = context.request.query("page").unwrap_or("1".to_owned());
        let per_page = context.request.query("per_page").unwrap_or("10".to_owned());

        let page = i64::from_str(&page)?;
        let per_page = i64::from_str(&per_page)?;

        let article_find = doc!{
            "status": 0
        };

        let mut article_find_option = FindOptions::default();

        article_find_option.sort = Some(doc!{
            "_id": (-1)
        });

        article_find_option.limit = Some(per_page);
        article_find_option.skip = Some((page - 1) * per_page);

        let articles = model::Article::find(article_find, Some(article_find_option))?;

        let articles_count = model::Article::count(doc!{}, None)?;

        let mut articles_json = Vec::new();

        for article in articles {

            let summary = if !article.summary.is_empty() {
                article.summary
            } else {
                let content = String2::from(article.content);
                let min = cmp::min(350, content.len());
                let summary: String2 = content[0..min].into();

                summary.into()
            };

            articles_json.push(json!({
                "id": article.id.to_hex(),
                "title": article.title,
                "image": article.image,
                "summary": summary,
                "create_at": article.create_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                "update_at": article.update_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string()
            }));
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
            "_id": (ObjectId::with_string(&article_id)?),
            "status": 0
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
                    "create_at": doc.create_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                    "update_at": doc.update_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string()
                });

                let collects = model::Collect::find(doc!{"articles_id": doc.id}, None)?;

                let mut collect_json = Vec::new();

                for collect in  collects {
                    collect_json.push(json!({
                        "id": collect.id.to_hex(),
                        "name": collect.name
                    }))
                }

                return_json["collects"] = json!(collect_json);

                Ok(Response::success(Some(return_json)))
            }
        }
    }});

    hand!(new, {|context: &mut Context| {
        let user_id = context.contexts.get_str("id").unwrap_or("");

        #[derive(Deserialize, Debug)]
        struct New {
            title: String,
            image: Vec<String>,
            content: String,
            #[serde(default)]
            summary: String
        }

        let new_json = context.request.bind_json::<New>()?;

        let article = model::Article {
            id: ObjectId::new()?,
            title: new_json.title,
            image: new_json.image,
            author_id: ObjectId::with_string(&user_id)?,
            collect_ids: Vec::new(),
            content: new_json.content,
            summary: new_json.summary,
            create_at: Utc::now().into(),
            update_at: Utc::now().into(),
            status: 0
        };

        article.save(None)?;

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
            image: Vec<String>,
            content: String,
            #[serde(default)]
            summary: String
        }

        let update_json = context.request.bind_json::<Update>()?;

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
                doc.update_at = Utc::now().into();

                doc.save(None)?;

                let return_json = json!({
                    "article_id": article_id
                });

                Ok(Response::success(Some(return_json)))
            }
        }
    }});

    // delete

    pub fn handle() -> Group {
        let mut group = Group::new("/console/article");

        group.get("/", Self::articles);
        group.get("/{id:[a-z0-9]{24}}", Self::detail);
        group.post("/", Self::new).before(middleware::auth);
        group.put("/{id:[a-z0-9]{24}}", Self::update).before(middleware::auth);

        group
    }
}
