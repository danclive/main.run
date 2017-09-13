use sincere::Group;

use sincere::Context;
/*
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
*/


pub struct Collect;

impl Collect {
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

    pub fn handle() -> Group {
        let mut collect_group = Group::new("/user");

        collect_group.get("/", Collect::list);
        collect_group.get("/{id:[a-z0-9]{24}}", Collect::detail);
        collect_group.post("/", Collect::new);
        collect_group.put("/{id:[a-z0-9]{24}}", Collect::commit);

        collect_group
    }
}
