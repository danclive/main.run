use std::time::Instant;

use sincere::App;
use sincere::{Context, Value};
use sincere::http::Method;

use chrono::Duration;
use chrono::{Local, DateTime, TimeZone};

use util::token;
use util::console_color::Print;
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

#[allow(dead_code, unused_variables)]
pub fn log(app: &mut App) {

    app.begin(move |context| {
        context.contexts.insert("instant".to_owned(), Value::Instant(Instant::now()));
    });

    app.finish(move |context| {
        let start_instant = context.contexts.get("instant").unwrap().as_instant().unwrap();
        let now_instant = Instant::now();

        let duration = Duration::from_std(now_instant - *start_instant).unwrap();

        //println!("{:?}", duration.num_milliseconds());
        let time_now: DateTime<Local> = Local::now();

        let status_code = context.response.status_code.as_ref();

        let status = match status_code / 100 {
            _ => Print::unset("NONE")
        };

        println!("{} {} |{}|", Print::green("[SINCERE]"), Print::green(time_now.format("%Y/%m/%d - %H:%M:%S %z").to_string()), status);
    });
}
