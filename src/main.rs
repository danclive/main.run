#[macro_use]
extern crate sincere;
extern crate sincere_token;
#[macro_use]
extern crate mongors;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
extern crate ring;
extern crate chrono;
extern crate postgres;
extern crate reqwest;
extern crate qiniu;

use sincere::app::App;
use sincere::app::run;
use sincere::log;

use mongors::client::MongoClient;
use mongors::database::Database;

#[macro_use]
mod macros;
mod error;
mod common;
mod util;
mod article;
mod collect;
mod auth;
mod media;
mod middleware;
mod struct_document;
mod model;
mod console;

lazy_static! {
    static ref DB: Database = {
        MongoClient::with_uri("mongodb://danclive.com:30000").expect("Failed to initialize client.").db("main-run")
    };

    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();

    static ref APP: App = start();
}

fn start() -> App {

    // ERROR!(target: "build_helper", "error message");
    // WARN!("warn message");
    // INFO!("info message");
    // DEBUG!("debug message");
    // TRACE!("trace message");

    let mut app = App::new();

    app.get("/", |context| {
        //DEBUG!("{:?}", context.request.headers());
        context.response.from_text("hello world!").unwrap();
    });

    app.post("/", |context| {
        let form_data = &context.request;

        println!("{:?}", form_data.content_type());

        context.response.from_text("hello world!").unwrap();
    });

    app.mount("/user", auth::Auth::handle);

    app.mount_group(article::Article::handle());
    app.mount_group(console::article::Article::handle());

    app.mount_group(collect::Collect::handle());

    app.mount_group(media::Media::handle());

    app.middleware(middleware::cors);

    app.middleware(middleware::log);

    app
}

fn main() {

    #[cfg(debug_assertions)]
    log::init(log::Level::Debug, &log::DefaultLogger);

    run("0.0.0.0:10001", 20, &APP).unwrap();
}
