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
extern crate serde_bytes;
extern crate ring;
extern crate chrono;
extern crate postgres;

use sincere::app::App;
use mon::client::Client;
use mon::db::Database;

use error::Result;
#[macro_use]
mod macros;
mod error;
mod common;
mod util;
mod article;
mod collect;
mod auth;
mod middleware;
mod struct_document;
mod model;


lazy_static! {
    static ref DB: Database = {
        Client::with_uri("mongodb://127.0.0.1:27017").expect("Failed to initialize client.").db("main-run")
    };
}

fn start() -> Result<()> {

    let mut app = App::new();

    app.get("/", |context| {
        context.response.from_text("hello world!").unwrap();
    });

    // app.post("/", |context| {
    //     let form_data = context.request.parse_formdata();

    //     if let Some(form_data) = form_data {
    //         println!("{:?}", form_data.fields);

    //         if form_data.has_file() {
    //             for mut file in form_data.files.into_iter() {
    //                 //use std::path::PathBuf;

    //                 //let mut path = PathBuf::from("/home/danc/temp");

    //                 let result = file.1.save_file("/home/danc/temp");

    //                 println!("{:?}", result);
    //             }
    //         }
    //     }

    //     context.response.from_text("hello world!").unwrap();
    // });

    app.mount(auth::Auth::handle);

    app.mount(article::Article::handle);

    app.mount(collect::Collect::handle);

    app.use_middleware(middleware::cors);

    app.use_middleware(middleware::log);

    app.run("127.0.0.1:9001", 20)?;

    Ok(())
}

fn main() {
    println!("Hello, world!");

    start().expect("can't start the server");
}
