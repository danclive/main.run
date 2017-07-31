use sincere::Context;

use mon::oid::ObjectId;
use mon::bson::bson::UTCDateTime;

#[allow(dead_code)]
struct Collect {
	id: ObjectId,
	create_at: UTCDateTime,
	update_at: UTCDateTime,
}

#[allow(dead_code)]
struct Article {
	id: ObjectId,
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

pub fn new(context: &mut Context) {
	if let Some(_id) = context.request.get_param("id") {

	}
}

pub fn commit(context: &mut Context) {
	if let Some(_id) = context.request.get_param("id") {

	}
}
