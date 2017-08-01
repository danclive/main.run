//use std::error::Error as stdError;

use serde::Serialize;

use error::Error;

#[derive(Serialize, Debug)]
pub struct Message {
    code: u16,
    info: String,
}

#[derive(Serialize, Debug)]
pub struct Response<D: Serialize> {
    success: bool,
    message: Option<Message>,
    data: Option<D>,
}

#[derive(Serialize, Debug)]
pub struct Empty;

impl<D: Serialize> Response<D> {
    pub fn success() -> Response<D> {
        Response {
            success: true,
            message: None,
            data: None,
        }
    }

    pub fn from_data(data: D) -> Response<D> {
        Response {
            success: true,
            message: None,
            data: Some(data),
        }
    }

    pub fn from_error(err: Error) -> Response<D> {
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
            Error::DocError(_) |
            Error::BsonEncodeError(_) => {
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
