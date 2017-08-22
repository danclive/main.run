use sincere::Context;
use ring::digest::{self, SHA256};
//use mon::util::hex::ToHex;
use mon;
use mon::bson;
use mon::bson::spec::BinarySubtype;
use mon::oid::ObjectId;
//use mon::bson::bson::UTCDateTime;

use chrono::Utc;

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
pub fn list(context: &mut Context) {

    let id = context.contexts.get("id");

    println!("{:?}", id);

    context.response.from_text("Hello Sincere").unwrap();
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Login {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct LoginReturn {
    token: String,
}

pub fn login(context: &mut Context) {

    let result = context.request.bind_json::<Login>()
        .map_err(|err| err.into() )
        .and_then(|result| {

            let user_col = DB.collection("user");

            let actual = digest::digest(&SHA256, result.password.as_bytes());

            let doc = doc!{
                "username" => (result.username),
                "password" => (BinarySubtype::Generic, actual.as_ref().to_vec())
            };

            Ok(user_col.find_one(Some(doc), None)?)

        }).and_then(|result| {
            match result {
                Some(doc) => {

                    let id = doc.get_object_id("_id")?.to_hex();
                    let token = token::generate_token(id.clone())?;

                    let login_return = LoginReturn {
                        token: token,
                    };

                    Ok(JsonResponse::from_data(login_return))
                }
                None => Err(ErrorCode(20002).into())
            }
        });

    match result {
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
