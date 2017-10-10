use std::i64;
use std::str::FromStr;

use sincere::Context;
use sincere::Group;

use mon::coll::options::FindOptions;

use chrono::Utc;
use chrono::Local;

use common::{Response, Empty};
use middleware;
use model;
use struct_document::StructDocument;

#[derive(Deserialize, Debug)]
struct New {
    title: String,
    content: String
}

pub struct Article;

impl Article {
    pub fn list(context: &mut Context) {
        let page = context.request.get_query("page").unwrap_or("1".to_owned());
        let per_page = context.request.get_query("per_page").unwrap_or("10".to_owned());

        let result = || {
            let page = i64::from_str(&page)?;
            let per_page = i64::from_str(&per_page)?;

            let mut article_find_option = FindOptions::default();

            article_find_option.sort = Some(doc!{
                "_id": (-1)
            });

            article_find_option.limit = Some(per_page);
            article_find_option.skip = Some((page - 1) * per_page);

            let articles = model::Article::find(None, Some(article_find_option))?;

            let articles_count = model::Article::count(None, None)?;

            let mut articles_json = Vec::new();

            for article in articles {
                articles_json.push(json!({
                    "id": article.id.to_hex(),
                    "title": article.title,
                    "image": article.image,
                    "create_at": article.create_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                    "update_at": article.update_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                }));
            }

            let return_json = json!({
                "Articles": articles_json,
                "Count": articles_count
            });

            Ok(Response::success(Some(return_json)))

        };

        match result() {
            Ok(result) => {
                context.response.from_json(result).unwrap();
            },
            Err(err) => {
                context.response.from_json(Response::<Empty>::error(err)).unwrap();
            }
        }

    }

    pub fn detail(_context: &mut Context) {

    }

    pub fn new(_context: &mut Context) {

    }

    pub fn update(_context: &mut Context) {

    }

    pub fn handle() -> Group {
        let mut group = Group::new("/article");

        group.get("/", Self::list);
        group.get("/{id:[a-z0-9]{24}}", Self::detail);
        group.post("/", Self::new).before(middleware::auth);
        group.put("/{id:[a-z0-9]{24}}", Self::update).before(middleware::auth);

        group
    }
}
