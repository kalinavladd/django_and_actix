use jsonwebtoken;
use jsonwebtoken::{Algorithm, TokenData};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    token_type: String,
    exp: i64,
    iat: i64,
    jti: String,
    user_id: i64,
}


pub async fn get_user_id_from_jwt_token(token: &str) -> Result<i64, String> {
    let django_secret_key = std::env::var("SECRET_KEY").unwrap();
    let secret = jsonwebtoken::DecodingKey::from_secret(django_secret_key.as_ref());
    let decoded = jsonwebtoken::decode::<Claims>(
        &token, &secret, &jsonwebtoken::Validation::new(Algorithm::HS256));
    match decoded {
        Ok(token_data) => Ok(token_data.claims.user_id),
        Err(_) => Err("Invalid token".to_string())
    }
}