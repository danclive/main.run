use sincere_token::{self, Message, Algorithm};
use chrono::{Utc, DateTime};

use error::Result;
use error::ErrorCode;

const KEY: &str = "key";
const EXPIRATION_TIME: i64 = 60 * 60 * 24 * 30;

#[derive(Serialize, Deserialize, Debug)]
struct Token {
	user_id: String,
	date: i64,
}

impl Message for Token {}

pub fn generate_token(user_id: String) -> Result<String> {
	let utc: DateTime<Utc> = Utc::now();

	let token = Token {
		user_id: user_id,
		date: utc.timestamp()
	};

	let token = sincere_token::encode(KEY, token, Algorithm::SHA256)?;

	Ok(token)
}

pub fn verify_token(token: String) -> Result<String> {
	let utc: DateTime<Utc> = Utc::now();

	let token = sincere_token::decode::<Token>(KEY, token)?;

	if token.date + EXPIRATION_TIME < utc.timestamp() {
		return Err(ErrorCode(20001).into())
	}

	Ok(token.user_id)
}
