use sincere::Request;
use sincere::Response;
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
pub fn list(request: &mut Request, response: &mut Response) {

    if let Some(token) = request.get_header("token") {
        let ok = token::verify_token(token);

        println!("{:?}", ok);
    }

    response.from_text("Hello Sincere").unwrap();
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

pub fn login(request: &mut Request, response: &mut Response) {

    let result = request.bind_json::<Login>()
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
            response.from_json(result).unwrap();
        },
        Err(err) => {
            response.from_json(JsonResponse::<Empty>::from_error(err)).unwrap();
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Logon {
    username: String,
    password: String,
}

pub fn logon(request: &mut Request, response: &mut Response) {
    
    let user_col = DB.collection("user");

    let result = request.bind_json::<Logon>()
        .map_err(|err| err.into() ).
        and_then(|result| {

            let doc = doc!{
                "username" => (result.username.clone())
            };

            if let Some(_) = user_col.find_one(Some(doc), None)? {
                return Err(ErrorCode(20003).into());
            }


            Ok(result)
        }).
        and_then(|result| {

            let actual = digest::digest(&SHA256, result.password.as_bytes());

            let doc = doc!{
                "_id" => (ObjectId::new().unwrap()),
                "username" => (result.username),
                "password" => (BinarySubtype::Generic, actual.as_ref().to_vec()),
                "avatar" => "",
                "role" => "Guest",
                "create_at" => (bson::Bson::from(Utc::now())),
                "update_at" => (bson::Bson::from(Utc::now()))
            };

            Ok(user_col.insert_one(doc, None)?)
        }).and_then(|result| {
            if !result.acknowledged {
                return Err(ErrorCode(20002).into());
            }

            if let Some(exception) = result.write_exception {
                return Err(mon::error::Error::WriteError(exception).into());
            }

            Ok(JsonResponse::<Empty>::success())
        });

    match result {
        Ok(result) => {
            response.from_json(result).unwrap();
        },
        Err(err) => {
            response.from_json(JsonResponse::<Empty>::from_error(err)).unwrap();
        }
    }
}
