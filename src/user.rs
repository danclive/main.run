use sincere::Context;
use sincere::Group;

use mon::oid::ObjectId;
use mon::coll::options::FindOptions;

//use chrono::Utc;
use chrono::Local;

use common::{Response as JsonResponse, Empty};
use DB;
use error::ErrorCode;

use middleware;
/*
#[derive(Serialize, Debug, PartialEq, Eq)]
enum Role {
    Admin,
    Guest
}

#[derive(Serialize, Debug, PartialEq, Eq)]
struct User {
    id: ObjectId,
    username: String,
    avatar: String,
    role: Role,
    create_at: UTCDateTime,
    update_at: UTCDateTime,
}
*/

pub struct User;

impl User {
    pub fn detail(context: &mut Context) {

        let id = context.contexts.get("id").unwrap().as_str().unwrap();

        let user_col = DB.collection("user");

        let result = || {

            let user_find = doc!{
                "_id" => (ObjectId::with_string(id)?)
            };

            let mut user_find_option = FindOptions::default();

            user_find_option.projection = Some(doc!{
                "password" => 0
            });

            let user_doc_find = user_col.find_one(Some(user_find), Some(user_find_option))?;

            if let None = user_doc_find {
                return Err(ErrorCode(10004).into());
            }

            let user_doc = user_doc_find.unwrap();

            let return_json = json!({
                "Id": user_doc.get_object_id("_id")?.to_string(),
                "Username": user_doc.get_str("username")?.to_string(),
                "Avatar": user_doc.get_str("avatar")?.to_string(),
                "Role": user_doc.get_str("role")?.to_string(),
                "CreateAt": user_doc.get_utc_datetime("create_at")?.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string()
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
        let mut group = Group::new("/user");

        group.get("/", User::detail).before(middleware::auth);

        group
    }
}


