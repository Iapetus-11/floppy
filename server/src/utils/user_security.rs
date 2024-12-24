use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_384};

use super::{security::random_string, xid::Xid};

#[derive(Debug, Serialize, Deserialize)]
struct UserAccessTokenClaims {
    iss: String,
    sub: String,
    exp: usize,
    iat: usize,
}

#[derive(Serialize)]
pub struct UserAccessToken {
    pub access_token: String,
}

#[derive(Serialize)]
pub struct UserTokens {
    pub access_token: String,
    pub refresh_token: String,
}

fn generate_access_token_(jwt_signing_key: &str, user_id: &Xid) -> String {
    let access_token_claims = UserAccessTokenClaims {
        iss: "floppy".to_string(),
        sub: user_id.to_string(),
        exp: 15 * 60, // 15 minutes
        iat: Utc::now().timestamp() as usize,
    };

    let jwt_signing_key = jsonwebtoken::EncodingKey::from_secret(jwt_signing_key.as_bytes());

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &access_token_claims,
        &jwt_signing_key,
    )
    .unwrap()
}

pub fn generate_access_token(jwt_signing_key: &str, user_id: &Xid) -> UserAccessToken {
    UserAccessToken {
        access_token: generate_access_token_(jwt_signing_key, user_id),
    }
}

pub fn generate_tokens(jwt_signing_key: &str, user_id: &Xid) -> UserTokens {
    let access_token = generate_access_token_(jwt_signing_key, user_id);
    let refresh_token = random_string(128);

    UserTokens {
        access_token,
        refresh_token,
    }
}

pub fn hash_refresh_token(refresh_token: &str) -> Vec<u8> {
    let mut hasher = Sha3_384::new();
    hasher.update(refresh_token.as_bytes());
    hasher.finalize().to_vec()
}
