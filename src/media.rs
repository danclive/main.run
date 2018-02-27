use sincere::app::context::Context;
use sincere::app::Group;
use sincere::http::plus::server::FilePart;
use sincere::http::plus::random_alphanumeric;

use chrono::{Utc};

use common::{Response, Empty};

use qiniu::{Config, PutPolicy};

use reqwest::multipart::{Form, Part};

use HTTP_CLIENT;

pub struct Media;

impl Media {
    hand!(upload, {|context: &mut Context| {

        if context.request.has_file() {
            upload_file(context.request.files());
        }

        Ok(Response::<Empty>::success(None))
    }});

    pub fn handle() -> Group {

        let mut group = Group::new("/media");

        group.post("/upload", Self::upload);

        group
    }
}

fn upload_file(files: &Vec<FilePart>) {

    const ACCESS_KEY: &str = "RHigUX-2wovAQf-SSc_9U5mGw9BdKcSGImGbXXHU";
    const SECRET_KEY: &str = "Rqdb_n9sMAhIGlSGolUAgbzWT39ypgM_ulTwDY4N";

    // get timestamp
    let now = Utc::now();
    let timestamp = now.timestamp();

    // new config and put policy
    let config = Config::new(ACCESS_KEY, SECRET_KEY);
    let mut put_policy = PutPolicy::new("scene", (timestamp + 3600) as u32);

    // set return body
    let return_body = r#"{"key": $(key), "hash": $(etag), "filename": $(fname), "filesize": $(fsize), "mime_type": $(mimeType), "extension": $(ext), "width": $(imageInfo.width), "height": $(imageInfo.height)}"#;
    put_policy.return_body = Some(return_body.to_owned());
    put_policy.save_key = Some("$(etag)$(ext)".to_owned());

    // generate upload token
    let token = put_policy.generate_uptoken(&config);

    for file_part in files {

        let reader = ::std::io::Cursor::new(file_part.data.clone());
        let filename = file_part.filename.clone().unwrap_or(random_alphanumeric(32));
        let part = Part::reader_with_length(reader, file_part.data.len() as u64).mime(file_part.content_type.clone()).file_name(filename);

        // new form and set file
        let form = Form::new().text("token", token.clone()).part("file", part);

        let mut response = HTTP_CLIENT.post("http://upload.qiniup.com/").multipart(form).send().unwrap();

        println!("{:?}", response);

        let mut buf: Vec<u8> =  Vec::new();

        response.copy_to(&mut buf).unwrap();

        println!("{:?}", String::from_utf8_lossy(&buf));
    }
}
