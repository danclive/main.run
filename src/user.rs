use sincere::Request;
use sincere::Response;
use ring::digest::{self, SHA256};
//use mon::util::hex::ToHex;
use mon::bson::spec::BinarySubtype;

use common::{Response as JsonResponse, Null};
use DB;
use util::token;
use error::ErrorCode;

pub fn list(request: &mut Request, response: &mut Response) {
    println!("{:?}", request.path());
    println!("{:?}", request.querys());
    println!("{:?}", request.get_query("aaaa"));
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
    user_id: String,
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
                    user_id: id,
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
            response.from_json(JsonResponse::<Null>::from_error(err)).unwrap();
        }
    }
}

pub fn logon(request: &mut Request, response: &mut Response) {
    println!("{:?}", request.path());
    response.from_text("Hello Sincere").unwrap();
}
