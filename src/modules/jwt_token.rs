use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};

use crate::{models::auth::Claims, state::AppState};

pub struct JwtToken;

impl JwtToken {
    pub fn encode_token(id: usize, role: String, app_state: &AppState) -> Result<String, JwtError> {
        let exp: usize = (Utc::now() + Duration::days(1)).timestamp() as usize;
        let claims: Claims = Claims { id, role, exp };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(app_state.jwt_secret.as_str().as_ref()),
        )
    }

    pub fn decode_token(token: &str, app_state: &AppState) -> Result<TokenData<Claims>, JwtError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(app_state.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
    }
}
