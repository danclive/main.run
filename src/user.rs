use sincere::Context;
use ring::digest::{self, SHA256};
//use mon::util::hex::ToHex;
use mon;
use mon::bson;
use mon::bson::spec::BinarySubtype;
use mon::oid::ObjectId;
use mon::coll::options::FindOptions;

use chrono::Utc;
use chrono::Local;

use common::{Response as JsonResponse, Empty};
use DB;
use util::token;
use error::ErrorCode;
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
pub fn detail(context: &mut Context) {

    let id = context.contexts.get("id").unwrap();

    let user_col = DB.collection("user");

    let result = || {

        let user_find = doc!{
            "_id" => (ObjectId::with_string(&id)?)
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

        Ok(JsonResponse::from_data(return_json))
    };

    match result() {
        Ok(result) => {
            context.response.from_json(result).unwrap();
        },
        Err(err) => {
            context.response.from_json(JsonResponse::<Empty>::from_error(err)).unwrap();
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Login {
    username: String,
    password: String,
}

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

        Ok(JsonResponse::from_data(return_json))
    };

    match result() {
        Ok(result) => {
            context.response.from_json(result).unwrap();
        },
        Err(err) => {
            context.response.from_json(JsonResponse::<Empty>::from_error(err)).unwrap();
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Logon {
    username: String,
    password: String,
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

        Ok(JsonResponse::<Empty>::success())
    };

    match result() {
        Ok(result) => {
            context.response.from_json(result).unwrap();
        },
        Err(err) => {
            context.response.from_json(JsonResponse::<Empty>::from_error(err)).unwrap();
        }
    }
}
