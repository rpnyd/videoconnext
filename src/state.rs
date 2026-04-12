use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::broadcast;
use crate::models::{User, ChatMessage};

// =====================================================================
// CONNECTED USER — ek live WebSocket connection
// =====================================================================

#[derive(Debug, Clone)]
pub struct ConnectedUser {
    pub user_id:  String,
    pub username: String,
    pub lat: f64,
    pub lon: f64,
    /// Is channel pe message daalo → us user ke WebSocket pe jayega
    pub sender: broadcast::Sender<ChatMessage>,
}

// =====================================================================
// APP STATE — poore server ki shared memory
// =====================================================================
// Arc<DashMap> use kar rahe hai kyunki:
//   - Arc  = multiple threads share kar sakein bina copy kiye
//   - DashMap = thread-safe HashMap (normal HashMap crash karta hai)

#[derive(Clone)]
pub struct AppState {
    /// Saare registered users (user_id → User)
    pub users: Arc<DashMap<String, User>>,

    /// Abhi connected users (user_id → ConnectedUser)
    pub online: Arc<DashMap<String, ConnectedUser>>,

    /// JWT sign karne ke liye secret
    /// Production mein env variable se lo!
    pub jwt_secret: String,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            users:      Arc::new(DashMap::new()),
            online:     Arc::new(DashMap::new()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "super_secret_change_in_production".to_string()),
        }
    }
}
