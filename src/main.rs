extern crate sincere;
extern crate sincere_token;
#[macro_use]
extern crate mon;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
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
mod article;
mod collect;
mod middleware;

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref DB: Database = {
    	Client::with_uri("mongodb://dev.mcorce.com:27017").expect("Failed to initialize client.").db("test")
    };
}


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




fn start() -> Result<()> {

	let mut app = App::new();

	app.get("/", |context| {
		//println!("{:?}", DB.version());
		context.response.from_text("hello world!").unwrap();
	});



	app.post("/test", |context| {
		let message = context.request.bind_json::<Messages>();

		println!("{:?}", message);

		let a = Messages {
			user_id: 123,
			date: 456,
			message: None
		};

		context.response.from_json(a).unwrap();
	});


	let mut user_group = Group::new("/user");

    user_group.get("/", user::list).before(middleware::auth);
	user_group.post("/login", user::login);
	user_group.post("/logon", user::logon);

	app.mount(user_group);

    let mut article_group = Group::new("/article");

    article_group.before(middleware::auth);

    article_group.get("/", article::list);
    article_group.get("/{id:[a-z0-9]{24}}", article::detail);
    article_group.post("/", article::new).before(middleware::auth);
    article_group.put("/{id:[a-z0-9]{24}}/release", article::commit).before(middleware::auth);

    app.mount(article_group);

    let mut collect_group = Group::new("/collect");

    collect_group.get("/", collect::list);
    collect_group.get("/{id:[a-z0-9]{24}}", collect::detail);
    collect_group.post("/", collect::new);
    collect_group.put("/{id:[a-z0-9]{24}}", collect::commit);

    app.mount(collect_group);

    middleware::cors(&mut app);

	app.run("0.0.0.0:8000", 4)?;
    //app.run_tls("127.0.0.1:8000", 4,"/home/simple/test.mcorce.com/fullchain.cer", "/home/simple/test.mcorce.com/test.mcorce.com.key").unwrap();

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
