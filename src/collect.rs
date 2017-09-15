use sincere::Group;
use sincere::Context;

use mon;
use mon::oid::ObjectId;

use DB;
use common::{Response as JsonResponse, Empty};
//use error::ErrorCode;
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
    articles: Vec<ObjectId>
}

pub struct Collect;

impl Collect {
    pub fn list(_context: &mut Context) {
        
    }

    pub fn detail(_context: &mut Context) {
        
    }

    pub fn new(context: &mut Context) {
        let id = context.contexts.get("id").unwrap().as_str().unwrap();

        let request = &context.request;

        let collect_col = DB.collection("collect");
        
        let result = || {

            let new_json = request.bind_json::<New>()?;

            let collect_id = ObjectId::new()?;
            let collect = doc!{
                "_id" => (collect_id.clone()),
                "title" => (new_json.title),
                "description" => (new_json.description),
                "owner_ids" => [ObjectId::with_string(id)?]
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

        let result = || {


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
        let mut group = Group::new("/user");

        group.get("/", Collect::list);
        group.get("/{id:[a-z0-9]{24}}", Collect::detail);
        group.post("/", Collect::new).before(middleware::auth);
        group.put("/{id:[a-z0-9]{24}}", Collect::include).before(middleware::auth);

        group
    }
}
