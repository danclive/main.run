extern crate sincere;
extern crate sincere_token;
extern crate mon;
#[macro_use]
extern crate lazy_static;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::thread;

use sincere::Micro;
use sincere::Group;
use mon::client::Client;

use error::Result;

mod error;
mod user;
mod common;

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref DBCLIENT: Client = {
    	Client::with_uri("mongodb://localhost:27017").expect("Failed to initialize client.")
    };
}

/*
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
//#[serde(deny_unknown_fields)]
struct Messages {
    //#[serde(rename = "userid")]
    #[serde(default)]
    user_id: i64,
    #[serde(default)]
    date: i64,
    #[serde(default)]
    message: Option<(String, String)>,
}
*/



fn start() -> Result<()> {
	let mut app = Micro::new();

	app.get("/", |_request, _response| {

		let db = DBCLIENT.db("test");
		println!("{:?}", db.version());

	});

/*
	app.post("/test", |request, response| {
		let message = request.bind_json::<Messages>();

		println!("{:?}", message);

		let a = Messages {
			user_id: 123,
			date: 456,
			message: None
		};

		let result = response.from_json(a);

		println!("{:?}", result);
	});
*/

	let mut user_group = Group::new("/user");

	user_group.get("/", user::list);
	user_group.post("/login", user::login);
	user_group.post("/logon", user::logon);

	app.mount(user_group);


	app.run("0.0.0.0:8000")?;

    Ok(())
}

fn main() {
    println!("Hello, world!");

    thread::spawn(|| {
    	start().unwrap();
    }).join().unwrap();
}
