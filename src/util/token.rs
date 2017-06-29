use sincere_token::{self, Message, Algorithm};
use chrono::{UTC, DateTime};

use error::Result;
use error::ErrorCode;

const KEY: &str = "key";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Token {
	user_id: String,
	date: i64,
}

impl Message for Token {}

pub fn generate_token(user_id: String) -> Result<String> {
	let utc: DateTime<UTC> = UTC::now();

	let token = Token {
		user_id: user_id,
		date: utc.timestamp()
	};

	let token = sincere_token::encode(KEY, token, Algorithm::SHA256)?;

	Ok(token)
}

pub fn verify_token(token: String, expiration_time: i64) -> Result<String> {
	let utc: DateTime<UTC> = UTC::now();

	let token = sincere_token::decode::<Token>(KEY, token)?;

	if token.date + expiration_time < utc.timestamp() {
		return Err(ErrorCode(20001).into())
	}

	Ok(token.user_id)
}
