use std::i64;
use std::str::FromStr;

use sincere::Context;
use sincere::Group;

use mon::coll::options::FindOptions;
use mon::oid::ObjectId;

use chrono::Utc;
use chrono::Local;

use common::{Response, Empty};
use middleware;
use model;
use struct_document::StructDocument;
use error::ErrorCode;

pub struct Collect;

impl Collect {

    hand!(list ,{|context: &mut Context| {
        let page = context.request.get_query("page").unwrap_or("1".to_owned());
        let per_page = context.request.get_query("per_page").unwrap_or("10".to_owned());

        let page = i64::from_str(&page)?;
        let per_page = i64::from_str(&per_page)?;

        let collect_find = doc!{
            "status": 0
        };

        let mut collect_find_option = FindOptions::default();

        collect_find_option.limit = Some(per_page);
        collect_find_option.skip = Some((page - 1) * per_page);

        let collects = model::Collect::find(Some(collect_find), Some(collect_find_option))?;

        let collect_count = model::Article::count(None, None)?;

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
        let collect_id = context.request.get_param("id").unwrap();

        let collect_find = doc!{
            "_id": (ObjectId::with_string(&collect_id)?)
        };

        let collect = model::Collect::find_one(Some(collect_find), None)?;

        match collect {
            None => return Err(ErrorCode(20002).into()),
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

        collect.save(None)?;

        let return_json = json!({
            "collect_id": collect.id.to_hex()
        });

        Ok(Response::success(Some(return_json)))
    }});

    hand!(update, {|context: &mut Context| {
        let collect_id = context.request.get_param("id").unwrap();

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

        let collect = model::Collect::find_one(Some(collect_find), None)?;

        match collect {
            None => return Err(ErrorCode(20002).into()),
            Some(mut doc) => {
                doc.name = update_json.name;
                doc.description = update_json.description;
                doc.image = update_json.image;
                doc.update_at = Utc::now().into();

                doc.save(None)?;

                let return_json = json!({
                    "collect_id": collect_id
                });

                Ok(Response::success(Some(return_json)))
            }
        }
    }});

    // delete

    // push

    // remove

    pub fn handle() -> Group {
        let mut group = Group::new("/article");

        group.get("/", Self::list);
        group.get("/{id:[a-z0-9]{24}}", Self::detail);
        group.post("/", Self::new).before(middleware::auth);
        group.put("/{id:[a-z0-9]{24}}", Self::update).before(middleware::auth);

        group
    }
}
