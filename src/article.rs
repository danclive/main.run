use sincere::Context;

use mon::oid::ObjectId;
use mon::bson::bson::UTCDateTime;

use common::{Response as JsonResponse, Empty};

#[allow(dead_code)]
struct Collect {
	id: ObjectId,
	create_at: UTCDateTime,
	update_at: UTCDateTime,
}

#[allow(dead_code)]
struct Article {
	id: ObjectId,
	title: String,
	owner_ids: Vec<ObjectId>,
	attend_ids: Vec<ObjectId>,
	collect_ids: Vec<ObjectId>,
	create_at: UTCDateTime,
	update_at: UTCDateTime,
}

#[allow(dead_code)]
struct Release {
	id: ObjectId,
	article_id: ObjectId,
	sections: Vec<ObjectId>,
	create_at: UTCDateTime,
}

#[allow(dead_code)]
struct Section {
	id: ObjectId,
	content: String,
	create_at: UTCDateTime,
}

pub fn list(context: &mut Context) {
	if let Some(_id) = context.contexts.get("id") {

	}
}

pub fn detail(context: &mut Context) {
	if let Some(_id) = context.request.get_param("id") {

	}
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct New {
	title: String,
	content: String
}

pub fn new(context: &mut Context) {
	let _id = context.contexts.get("id").unwrap();

	let result = context.request.bind_json::<New>()
		.map_err(|err| err.into() )
		.and_then(|result| {
				
			println!("{:?}", result.title);
			println!("{:?}", result.content);

			Ok(())
		});

	match result {
	    Ok(result) => {
	        context.response.from_json(result).unwrap();
	    },
	    Err(err) => {
	        context.response.from_json(JsonResponse::<Empty>::from_error(err)).unwrap();
	    }
	}
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Commit {
	id: i64,
	title: String,
	content: String
}

pub fn commit(context: &mut Context) {
	let _id = context.contexts.get("id").unwrap();
}
