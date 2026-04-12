use axum::{extract::State, Json, http::StatusCode};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

use crate::{
    state::AppState,
    models::{SignupRequest, LoginRequest, AuthResponse, User},
    auth::jwt::create_token,
};

// =====================================================================
// POST /signup
// =====================================================================
// Body (JSON):
//   { "username": "ali", "password": "pass123", "lat": 19.24, "lon": 73.13 }
//
// Response (JSON):
//   { "token": "...", "user_id": "...", "username": "ali" }

pub async fn signup(
    State(state): State<AppState>,
    Json(req): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {

    // --- Validation ---
    if req.username.trim().is_empty() || req.password.len() < 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username khali nahi hona chahiye, password min 6 characters".to_string(),
        ));
    }

    // --- Username already lia hua? ---
    let taken = state.users.iter().any(|u| u.username == req.username);
    if taken {
        return Err((StatusCode::CONFLICT, "Username already exist karta hai".to_string()));
    }

    // --- Password hash karo (kabhi bhi plain text store mat karo!) ---
    let password_hash = hash(&req.password, DEFAULT_COST)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user_id = Uuid::new_v4().to_string();

    let user = User {
        id:            user_id.clone(),
        username:      req.username.clone(),
        password_hash,
        lat:           req.lat,
        lon:           req.lon,
    };

    // --- Memory mein save karo ---
    state.users.insert(user_id.clone(), user);

    tracing::info!("Naya user signup: {}", req.username);

    // --- JWT token banao ---
    let token = create_token(&user_id, &req.username, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(AuthResponse { token, user_id, username: req.username }))
}

// =====================================================================
// POST /login
// =====================================================================

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {

    // --- User dhundho ---
    let user = state
        .users
        .iter()
        .find(|u| u.username == req.username)
        .map(|u| u.clone())
        .ok_or((StatusCode::UNAUTHORIZED, "Username ya password galat hai".to_string()))?;

    // --- Password verify karo ---
    let valid = verify(&req.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "Username ya password galat hai".to_string()));
    }

    tracing::info!("User login: {}", user.username);

    let token = create_token(&user.id, &user.username, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(AuthResponse {
        token,
        user_id:  user.id.clone(),
        username: user.username.clone(),
    }))
}
