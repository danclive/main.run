use std::i64;
use std::str::FromStr;

use sincere::app::context::Context;
use sincere::app::Group;

use mongors::collection::options::FindOptions;
use mongors::object_id::ObjectId;
use mongors::{doc, bson};

use chrono::Local;

use serde_json::json;

use crate::common::{Response, Empty};
use crate::model;
use crate::struct_document::StructDocument;
use crate::error::ErrorCode;

pub struct Collect;

impl Collect {

    hand!(collects, {|context: &mut Context| {
        let page = context.request.query("page").unwrap_or("1".to_owned());
        let per_page = context.request.query("per_page").unwrap_or("10".to_owned());

        let page = i64::from_str(&page)?;
        let per_page = i64::from_str(&per_page)?;

        let collect_find = doc!{
            "status": 0
        };

        let mut collect_find_option = FindOptions::default();

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

    pub fn handle() -> Group {
        let mut group = Group::new("/collect");

        group.get("/", Self::collects);
        group.get("/{id:[a-z0-9]{24}}", Self::detail);

        group
    }
}
