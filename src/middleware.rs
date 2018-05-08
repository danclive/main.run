use sincere::app::App;
use sincere::app::context::Context;
use sincere::http::Method;
use sincere::log::color::{Print, Color};

use chrono::{Local, Utc, DateTime};

use util::token;
use common::{Response, Empty};

pub fn auth(context: &mut Context) {
    if let Some(token) = context.request.header("Token") {
        match token::verify_token(token) {
            Ok(id) => {
                context.contexts.insert("id".to_owned(), id);
            },
            Err(err) => {
                context.response.from_json(Response::<Empty>::error(err)).unwrap();
                context.stop();
            }
        }
    } else {
        context.response.status_code(401);
        context.stop();
    }
}

pub fn cors(app: &mut App) {
    
    app.begin(move |context| {
        if context.request.method() == &Method::Options {
            context.response
            .status_code(204)
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
        let data_time: DateTime<Utc> = Utc::now();

        context.contexts.insert("time".to_owned(), data_time);
    });

    app.finish(move |context| {
        let start_time = context.contexts.get_utc_datetime("time").unwrap();

        let now_time: DateTime<Utc> = Utc::now();

        let duration = now_time.signed_duration_since(*start_time);

        let s = duration.num_seconds();
        let ms = duration.num_nanoseconds().unwrap_or(0) / (1000 * 1000);

        let s = if s != 0 {
            format!("{}s", s)
        } else if ms != 0 {
            format!("{}ms", ms)
        } else {
            format!("{}us", duration.num_nanoseconds().unwrap_or(0) / 1000)
        };

        let status_code = context.response.get_status_code();

        let method = context.request.method();

        let path = context.request.uri();

        let status = match status_code / 100 {
            1 => Print::white(status_code.to_string()).background(Color::Blue),
            2 => Print::white(status_code.to_string()).background(Color::Green),
            3 => Print::white(status_code.to_string()).background(Color::Yellow),
            4 => Print::white(status_code.to_string()).background(Color::Purple),
            5 => Print::white(status_code.to_string()).background(Color::Red),
            _ => Print::unset("NONE".to_string())
        };

        println!(
            "{} {} {}{}{} {:>5} {} {} {}",
            Print::green("[MAIN.RUN]"),
            Print::green(now_time.with_timezone(&Local).format("%Y/%m/%d - %H:%M:%S %z").to_string()),
            Print::green("|"),
            status,
            Print::green("|"),
            Print::green(s),
            Print::green("|"),
            Print::green(method),
            Print::green(path)
        );
    });
}
