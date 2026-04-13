use serde::{Deserialize, Serialize};

// =====================================================================
// USER MODELS
// =====================================================================

/// Database mein store hone wala user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub lat: f64,
    pub lon: f64,
}

/// POST /signup ka request body
#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
    pub lat: f64,
    pub lon: f64,
}

/// POST /login ka request body
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Login/Signup ke baad client ko milne wala response
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
    pub username: String,
}

// =====================================================================
// CHAT MODELS
// =====================================================================

/// Ek chat message — text, image, ya file ho sakta hai
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub from_user: String,
    pub from_username: String,
    pub content: MessageContent,
    pub timestamp: String,
}

/// Message ka type — text, image, ya file
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageContent {
    Text { text: String },
    Image { url: String, filename: String },
    File  { url: String, filename: String },
}

// =====================================================================
// WEBSOCKET INCOMING MESSAGES
// =====================================================================

/// Client se aane wale WS messages
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsIncoming {
    /// Normal text bhejo
    SendText { text: String },
    /// File/image share karo (URL jo /upload se mila)
    SendFile { url: String, filename: String, is_image: bool },
    /// Online users ki list maango
    GetOnlineUsers,
    /// Heartbeat — connection alive rakhne ke liye
    Ping,
}

// =====================================================================
// ONLINE USER (list ke liye)
// =====================================================================

#[derive(Debug, Clone, Serialize)]
pub struct OnlineUser {
    pub user_id: String,
    pub username: String,
    pub distance_km: f64,
}

// =====================================================================
// SERVER -> CLIENT EVENTS
// =====================================================================

/// Server client ko bhejta hai yeh events
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum ServerEvent {
    /// Naya message aaya
    NewMessage { message: ChatMessage },
    /// Online users list
    OnlineUsers { users: Vec<OnlineUser> },
    /// Koi user online/offline hua
    UserJoined { username: String },
    UserLeft   { username: String },
    /// Error
    Error { message: String },
}

// =====================================================================
// FILE UPLOAD RESPONSE
// =====================================================================

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub url: String,
    pub filename: String,
    pub file_type: String, // "image" ya "file"
}
