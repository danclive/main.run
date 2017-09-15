use sincere::App;
use sincere::{Context, Value};
use sincere::http::Method;

use util::token;
use common::{Response as JsonResponse, Empty};

pub fn auth(context: &mut Context) {
	if let Some(token) = context.request.get_header("Token") {
		match token::verify_token(token) {
			Ok(id) => {
				context.contexts.insert("id".to_owned(), Value::String(id));
			},
			Err(err) => {
				context.response.from_json(JsonResponse::<Empty>::error(err)).unwrap();
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

}
