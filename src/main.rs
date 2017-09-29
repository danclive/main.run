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
use mon::client::Client;
use mon::db::Database;

use error::Result;

mod error;
mod user;
mod common;
mod util;
mod article;
mod collect;
mod auth;
mod middleware;
mod odm;

lazy_static! {
    static ref DB: Database = {
        Client::with_uri("mongodb://127.0.0.1:27017").expect("Failed to initialize client.").db("main-run")
        //Client::with_uri("mongodb://dev.mcorce.com:27017").expect("Failed to initialize client.").db("test")
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

impl odm::StructDocument for Messages {
    // add code here
    const NAME: &'static str = "A";

    fn get_database() -> Database {
        DB.clone()
    }
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

    app.mount(auth::Auth::handle());

    app.mount(user::User::handle());

    app.mount(article::Article::handle());

    app.mount(collect::Collect::handle());

    app.use_middleware(middleware::cors);

    app.use_middleware(middleware::log);

    //app.run("0.0.0.0:8000", 4)?;
    app.run_tls("0.0.0.0:443", 4, "/etc/letsencrypt/live/api.main.run/fullchain.pem", "/etc/letsencrypt/live/api.main.run/privkey_rsa.pem").unwrap();
    //app.run_tls("127.0.0.1:1443", 4,"/home/simple/coding/rust/main.run/api.main.run/fullchain.pem", "/home/simple/coding/rust/main.run/api.main.run/privkey_rsa.pem").unwrap();

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
