use std::time::Instant;

use sincere::App;
use sincere::{Context, Value};
use sincere::http::Method;

use chrono::{Local, DateTime};

use util::token;
use util::console_color::{Print, Color};
use common::{Response, Empty};

pub fn auth(context: &mut Context) {
    if let Some(token) = context.request.get_header("Token") {
        match token::verify_token(token) {
            Ok(id) => {
                context.contexts.insert("id".to_owned(), Value::String(id));
            },
            Err(err) => {
                context.response.from_json(Response::<Empty>::error(err)).unwrap();
                context.stop();
            }
        }
    } else {
        context.response.status(401);
        context.stop();
    }
}

pub fn cors(app: &mut App) {
    
    app.begin(move |context| {
        if context.request.method() == &Method::Options {
            context.response
            .status(204)
            .header(("Access-Control-Allow-Methods", "GET,HEAD,PUT,PATCH,POST,DELETE,OPTIONS"));

            context.stop();
        }
    });

    app.finish(move |context| {
        context.response
        .header(("Access-Control-Allow-Origin", "*"))
        .header(("Access-Control-Allow-Headers", "content-type, token"));
    });
}

pub fn log(app: &mut App) {

    app.begin(move |context| {
        context.contexts.insert("instant".to_owned(), Value::Instant(Instant::now()));
    });

    app.finish(move |context| {
        let start_instant = context.contexts.get("instant").unwrap().as_instant().unwrap();

        let time_now: DateTime<Local> = Local::now();

        let status_code = context.response.status_code.as_ref();

        let now_instant = Instant::now();

        let duration = now_instant - *start_instant;

        let s = duration.as_secs();
        let ms = duration.subsec_nanos() / (1000 * 1000);

        let s = if s != 0 {
            format!("{}s", s)
        } else if ms != 0 {
            format!("{}ms", ms)
        } else {
            format!("{}us", duration.subsec_nanos() / 1000)
        };

        let method = context.request.method();

        let path = context.request.path();

        let status = match status_code / 100 {
            1 => Print::white(status_code.to_string()).background(Color::Blue),
            2 => Print::white(status_code.to_string()).background(Color::Green),
            3 => Print::white(status_code.to_string()).background(Color::Yellow),
            4 => Print::white(status_code.to_string()).background(Color::Purple),
            5 => Print::white(status_code.to_string()).background(Color::Red),
            _ => Print::unset("NONE".to_string())
        };

        println!(
            "{} {} |{}| {:>5} | {} {}",
            Print::green("[MAIN.RUN]"),
            Print::green(time_now.format("%Y/%m/%d - %H:%M:%S %z").to_string()),
            status, Print::green(s),
            Print::green(method),
            Print::green(path)
        );
    });
}
