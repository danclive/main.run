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
            _ => {
                Message {
                    code: 0,
                    info: format!("{}", err),
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
