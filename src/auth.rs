use sincere::Context;
use sincere::Group;
use ring::digest::{self, SHA256};

use mon;
use mon::bson;
use mon::bson::spec::BinarySubtype;
use mon::oid::ObjectId;

use chrono::Utc;

use common::{Response as JsonResponse, Empty};
use DB;
use util::token;
use error::ErrorCode;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Login {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Logon {
    username: String,
    password: String,
}

pub struct Auth;

impl Auth {
    pub fn login(context: &mut Context) {

        let request = &context.request;

        let user_col = DB.collection("user");

        let result = || {
            
            let login_json = request.bind_json::<Login>()?;

            let actual = digest::digest(&SHA256, login_json.password.as_bytes());

            let doc = doc!{
                "username" => (login_json.username),
                "password" => (BinarySubtype::Generic, actual.as_ref().to_vec())
            };

            let user_doc_find = user_col.find_one(Some(doc), None)?;

            if let None = user_doc_find {
                return Err(ErrorCode(20002).into())
            }

            let user_doc = user_doc_find.unwrap();

            let id = user_doc.get_object_id("_id")?.to_string();
            let token = token::generate_token(id)?;

            let return_json = json!({
                "Token": token
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

    #[allow(dead_code)]
    pub fn logon(context: &mut Context) {
    
        let request = &context.request;

        let user_col = DB.collection("user");

        let result = || {
            
            let logon_json = request.bind_json::<Logon>()?;

            let doc = doc!{
                "username" => (logon_json.username.clone())
            };

            if let Some(_) = user_col.find_one(Some(doc), None)? {
                return Err(ErrorCode(20003).into());
            }

            let actual = digest::digest(&SHA256, logon_json.password.as_bytes());

            let doc = doc!{
                "_id" => (ObjectId::new().unwrap()),
                "username" => (logon_json.username),
                "password" => (BinarySubtype::Generic, actual.as_ref().to_vec()),
                "avatar" => "",
                "role" => "Guest",
                "create_at" => (bson::Bson::from(Utc::now())),
                "update_at" => (bson::Bson::from(Utc::now()))
            };

            let insert_result = user_col.insert_one(doc, None)?;

            if let Some(exception) = insert_result.write_exception {
                return Err(mon::error::Error::WriteError(exception).into());
            }

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
        let mut group = Group::new("/auth");

        group.post("/login", Auth::login);
        //group.post("/logon", Auth::logon);

        group
    }
}
