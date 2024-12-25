use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use poem::http::StatusCode;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_384};

use crate::config::Config;

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
    let iat = Utc::now().timestamp() as usize;

    let access_token_claims = UserAccessTokenClaims {
        iss: "floppy".to_string(),
        sub: user_id.to_string(),
        exp: iat + (15 * 60), // 15 minutes
        iat,
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

#[allow(dead_code)]
pub struct AuthenticatedUser {
    pub access_token: String,
    pub id: Xid,
}

impl<'a> poem::FromRequest<'a> for AuthenticatedUser {
    async fn from_request(
        req: &'a poem::Request,
        _body: &mut poem::RequestBody,
    ) -> poem::Result<Self> {
        let config = req.data::<Config>().unwrap();

        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                poem::Error::from_string(
                    "This route requires authentication via the Authorization header",
                    StatusCode::FORBIDDEN,
                )
            })?;

        if let Some(access_token) = auth_header.strip_prefix("Bearer ") {
            println!("bruh: {access_token}");
            let decoded_token = jsonwebtoken::decode::<UserAccessTokenClaims>(
                access_token,
                &DecodingKey::from_secret(config.jwt_signing_key.as_bytes()),
                &Validation::new(Algorithm::default()),
            );

            if let Err(token_decode_error) = &decoded_token {
                println!(
                    "Failed to decode JWT access token: {:?}",
                    token_decode_error
                );
                return Err(poem::Error::from_string("Authorization header was invalid", StatusCode::FORBIDDEN));
            }
            let decoded_token = decoded_token.unwrap();

            Ok(AuthenticatedUser {
                access_token: access_token.to_string(),
                id: Xid::try_from(decoded_token.claims.sub.as_str()).unwrap(),
            })
        } else {
            Err(poem::Error::from_string(
                "Invalid format for Authorization header, expected: \"Bearer <token>\"",
                StatusCode::FORBIDDEN,
            ))
        }
    }
}
