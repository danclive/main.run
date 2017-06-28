use sincere::Request;
use sincere::Response;
//use serde_json;

use common;

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
    
	let login = request.bind_json::<Login>();

	match login {
		Ok(result) => {
			response.from_json(common::Response::from_data(result)).unwrap();
		}
		Err(err) => {
			response.from_json(common::Response::<common::Null>::from_error(err.into())).unwrap();
		}
	}

    //println!("{:?}", request.path());
    //response.from_text("Hello Sincere").unwrap();
}

pub fn logon(request: &mut Request, response: &mut Response) {
    println!("{:?}", request.path());
    response.from_text("Hello Sincere").unwrap();
}
