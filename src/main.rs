extern crate sincere;
extern crate sincere_token;
extern crate mon;
#[macro_use]
extern crate lazy_static;

//use std::sync::Arc;

use sincere::Micro;
use mon::client::Client;

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref DBCLIENT: Client = {
    	Client::with_uri("mongodb://localhost:27017").expect("Failed to initialize client.")
    };
}

fn start() {
	let mut app = Micro::new();

	app.get("/", |_request, _response| {

		let db = DBCLIENT.db("test");
		println!("{:?}", db.version());

	});

	app.run("0.0.0.0:8000").unwrap();
}

fn main() {
    println!("Hello, world!");

    start();
}
