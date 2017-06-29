use sincere::Request;
use sincere::Response;
use ring::digest::{self, SHA256};
//use mon::util::hex::ToHex;
use mon::bson::spec::BinarySubtype;

use common::{Response as JsonResponse, Null};
use DB;
pub use util::token;
use error::ErrorCode;

pub fn list(request: &mut Request, response: &mut Response) {
    println!("{:?}", request.path());
    response.from_text("Hello Sincere").unwrap();
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Login {
    //#[serde(default)]
    username: String,
    //#[serde(default)]
    password: String,
}

pub fn login(request: &mut Request, response: &mut Response) {
    
    let result = request.bind_json::<Login>().map_err(|err| err.into() ).and_then(|result| {

        let user_col = DB.collection("user");

        let actual = digest::digest(&SHA256, result.password.as_bytes());

        let doc = doc!{
            "username" => (result.username),
            "password" => (BinarySubtype::Generic, actual.as_ref().to_vec())
        };

        //let r = user_col.insert_one(doc, None);
        user_col.find_one(Some(doc), None).map_err(|err| err.into() )

        //Ok(JsonResponse::from_data(result))
    }).and_then(|result| {
        match result {
            Some(doc) => {

                #[derive(Serialize)]
                #[serde(rename_all = "PascalCase", deny_unknown_fields)]
                struct LoginReturn {
                    user_id: String,
                    token: String,
                }

                let id = doc.get_object_id("_id")?;

                //let token = token::generate_token();

                Ok(())
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


    /*
    if let Err(err) = result {
        response.from_json(JsonResponse::<Null>::from_error(err.into())).unwrap();
    }
    */

    /*
    match login {
        Ok(result) => {
            response.from_json(common::Response::from_data(result)).unwrap();
        }
        Err(err) => {
            response.from_json(common::Response::<common::Null>::from_error(err.into())).unwrap();
        }
    }
    */
    /*
    login.and_then(|result|{
        response.from_json(common::Response::from_data(result))
    });
    */



    //println!("{:?}", request.path());
    //response.from_text("Hello Sincere").unwrap();
}

pub fn logon(request: &mut Request, response: &mut Response) {
    println!("{:?}", request.path());
    response.from_text("Hello Sincere").unwrap();
}
