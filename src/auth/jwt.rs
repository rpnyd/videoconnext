use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::Utc;

// =====================================================================
// JWT CLAIMS — token ke andar kya store hota hai
// =====================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub:      String, // user_id
    pub username: String,
    pub exp:      usize,  // expiry (Unix timestamp)
}

// =====================================================================
// TOKEN BANANA
// =====================================================================

/// Login ke baad client ko yeh token do — 7 din valid rahega
pub fn create_token(user_id: &str, username: &str, secret: &str) -> Result<String, String> {
    let expiry = Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("time overflow")
        .timestamp() as usize;

    let claims = Claims {
        sub:      user_id.to_string(),
        username: username.to_string(),
        exp:      expiry,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| format!("Token create error: {}", e))
}

// =====================================================================
// TOKEN VERIFY KARNA
// =====================================================================

/// Har protected request pe yeh call karo
/// OK = valid token, uske claims lo
/// Err = invalid/expired token, 401 do
pub fn verify_token(token: &str, secret: &str) -> Result<Claims, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| format!("Token verify error: {}", e))
}
