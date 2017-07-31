use sincere::Context;

use util::token;
use common::{Response as JsonResponse, Empty};

pub fn auth(context: &mut Context) {
	if let Some(token) = context.request.get_header("token") {
		match token::verify_token(token) {
			Ok(id) => {
				context.contexts.insert("id".to_owned(), id);
			},
			Err(err) => {
				context.response.from_json(JsonResponse::<Empty>::from_error(err)).unwrap();
				context.stop();
			}
		}
	} else {
		context.response.status(401);
		context.stop();
	}
}
