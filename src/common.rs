//use std::error::Error as stdError;

use serde::Serialize;

use error::Error;

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Message {
    code: u16,
    info: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Response<D: Serialize> {
    success: bool,
    message: Option<Message>,
    data: Option<D>,
}

#[derive(Serialize, Debug)]
pub struct Empty;

impl<D: Serialize> Response<D> {
    pub fn success(data: Option<D>) -> Response<D> {
        Response {
            success: true,
            message: None,
            data: data,
        }
    }

    pub fn error(err: Error) -> Response<D> {
        #[allow(unreachable_patterns)]
        let message = match err {
            Error::CodedError(error_code) => {
                Message {
                    code: error_code.to_code(),
                    info: error_code.to_str().to_owned(),
                }
            }
            Error::IoError(_) |
            Error::Sincere(_) |
            Error::MonError(_) |
            Error::TokenError(_) |
            Error::DocError(_) |
            Error::BsonEncodeError(_) |
            Error::ParseIntError(_) |
            Error::ObjectIdError(_) => {
                Message {
                    code: 0,
                    //info: err.description().to_owned(),
                    info: format!("{}", err),
                }
            }
            
            _ => {
                Message {
                    code: 0,
                    info: "未知错误".to_owned(),
                }
            }
            
        };

        Response {
            success: false,
            message: Some(message),
            data: None,
        }
    }
}
