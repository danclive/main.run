use std::i64;
use std::str::FromStr;

use sincere::app::context::Context;
use sincere::app::Group;

use mongors::collection::options::FindOptions;
use mongors::object_id::ObjectId;

use chrono::Local;

use common::{Response, Empty};
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
                let collects = model::Collect::find(doc!{"_id": {"$in": article.collect_ids}}, None)?;

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

                if !doc.collect_ids.is_empty() {
                    let collects = model::Collect::find(doc!{"_id": {"$in": doc.collect_ids}}, None)?;

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

    pub fn handle() -> Group {
        let mut group = Group::new("/article");

        group.get("/", Self::articles);
        group.get("/{id:[a-z0-9]{24}}", Self::detail);

        group
    }
}
