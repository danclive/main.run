extern crate sincere;
extern crate sincere_token;
#[macro_use]
extern crate mon;
#[macro_use]
extern crate lazy_static;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate ring;
extern crate chrono;

use std::thread;

use sincere::App;
use sincere::Group;
use mon::client::Client;
use mon::db::Database;

use error::Result;

mod error;
mod user;
mod common;
mod util;

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref DB: Database = {
    	Client::with_uri("mongodb://dev.mcorce.com:27017").expect("Failed to initialize client.").db("test")
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
	let mut app = App::new();

	app.get("/", |context| {

		//let db = DBCLIENT.db("test");
		//println!("{:?}", db.version());
		context.response.from_text("hello world!").unwrap();
	}).before(|_context| {
        println!("{:?}", "before");
    
    }).after(|_context| {
        println!("{:?}", "after");
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

	//user_group.get("/{id:[a-z0-9]{24}}", user::list);
    user_group.get("/list", user::list);
	user_group.post("/login", user::login);
	user_group.post("/logon", user::logon);

	app.mount(user_group);



	app.run("0.0.0.0:8000", 8)?;

    Ok(())
}

fn main() {
    println!("Hello, world!");

    thread::spawn(|| {
    	loop {
    		match start() {
    			Ok(_) => (),
    			Err(_) => continue
    		}
    	}
    }).join().unwrap();

}

/*
use chrono::offset::TimeZone;

fn main() {
    let a = chrono::Utc::now();

    println!("{:?}", a);

    let b = a.with_timezone(&chrono::Local);

    println!("{:?}", b);

    let c = chrono::Local::now();

    println!("{:?}", c);
}
*/
