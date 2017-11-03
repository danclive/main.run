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

use sincere::App;
use mon::client::Client;
use mon::db::Database;

use error::Result;
#[macro_use]
mod macros;
mod error;
//mod user;
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
        //Client::with_uri("mongodb://dev.mcorce.com:27017").expect("Failed to initialize client.").db("test")
    };
}

fn start() -> Result<()> {

    let mut app = App::new();

    app.get("/", |context| {
        context.response.from_text("hello world!").unwrap();
    });

    app.mount(auth::Auth::handle());

    //app.mount(user::User::handle());

    app.mount(article::Article::handle());

    app.mount(collect::Collect::handle());

    app.use_middleware(middleware::cors);

    app.use_middleware(middleware::log);

    //app.run("0.0.0.0:8000")?;
    app.run_tls("0.0.0.0:443", "/etc/letsencrypt/live/api.main.run/fullchain.pem", "/etc/letsencrypt/live/api.main.run/privkey_rsa.pem").unwrap();

    Ok(())
}

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    println!("Hello, world!");

    //start().expect("can't start the server");

    let pool = util::posrgres_pool::ConnectionPool::new("postgresql://postgres:123456@localhost");

    for _ in 0..1000 {
        let pool = pool.clone();

        ::std::thread::spawn(move || {
            let polled_conn = pool.acquire_conn().unwrap();

            let conn = polled_conn.get_conn();

            // println!("{:?}", rows);

            // conn.execute("CREATE TABLE person (
            //         id              SERIAL PRIMARY KEY,
            //         name            VARCHAR NOT NULL,
            //         data            BYTEA
            //       )", &[]).unwrap();
            // let me = Person {
            //     id: 0,
            //     name: "Steven".to_string(),
            //     data: None,
            // };
            //conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                         //&[&me.name, &me.data]).unwrap();
            for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
                let person = Person {
                    id: row.get(0),
                    name: row.get(1),
                    data: row.get(2),
                };
                //println!("Found person {}", person.name);
            }

            ::std::thread::sleep_ms(500);
        }).join();
    }
}
