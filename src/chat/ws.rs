use axum::{
    extract::{State, WebSocketUpgrade, Query},
    response::Response,
    http::StatusCode,
};
use axum::extract::ws::{WebSocket, Message};
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;
use serde_json;

use crate::{
    state::{AppState, ConnectedUser},
    models::{ChatMessage, WsIncoming, MessageContent, OnlineUser, ServerEvent},
    auth::jwt::verify_token,
    location::haversine::{within_radius, distance_km},
};

// =====================================================================
// QUERY PARAMS — ws://localhost:3000/ws?token=JWT_TOKEN
// =====================================================================

#[derive(serde::Deserialize)]
pub struct WsQuery {
    pub token: String,
}

// =====================================================================
// WEBSOCKET UPGRADE HANDLER
// =====================================================================
// Browser yahan connect karta hai
// Pehle token verify hota hai, phir actual WebSocket shuru hota hai

pub async fn ws_handler(
    ws:               WebSocketUpgrade,
    Query(params):    Query<WsQuery>,
    State(state):     State<AppState>,
) -> Response {

    // Token verify karo
    let claims = match verify_token(&params.token, &state.jwt_secret) {
        Ok(c)  => c,
        Err(_) => {
            // Invalid token — connection reject karo
            return (StatusCode::UNAUTHORIZED, "Invalid token").into_response();
        }
    };

    // User ki info state se lo
    let (user_lat, user_lon) = state
        .users
        .get(&claims.sub)
        .map(|u| (u.lat, u.lon))
        .unwrap_or((0.0, 0.0));

    ws.on_upgrade(move |socket| {
        handle_socket(socket, state, claims.sub, claims.username, user_lat, user_lon)
    })
}

// =====================================================================
// MAIN WEBSOCKET HANDLER — yahan asli kaam hota hai
// =====================================================================

async fn handle_socket(
    socket:   WebSocket,
    state:    AppState,
    user_id:  String,
    username: String,
    lat:      f64,
    lon:      f64,
) {
    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Broadcast channel — is channel pe koi bhi message daale,
    // yeh user ke WebSocket pe automatically aayega
    let (tx, mut rx) = broadcast::channel::<ChatMessage>(128);

    // --- Online list mein add karo ---
    state.online.insert(user_id.clone(), ConnectedUser {
        user_id:  user_id.clone(),
        username: username.clone(),
        lat, lon,
        sender: tx,
    });

    tracing::info!("{} online hua ({}, {})", username, lat, lon);

    // --- Baaki nearby users ko batao yeh join hua ---
    notify_nearby(
        &state, &user_id, lat, lon,
        ServerEvent::UserJoined { username: username.clone() },
    );

    // --- Turant online users ki list bhejo naye user ko ---
    let online_list = get_nearby_users(&state, &user_id, lat, lon);
    let list_event  = serde_json::to_string(
        &ServerEvent::OnlineUsers { users: online_list }
    ).unwrap_or_default();

    let _ = ws_sender.send(Message::Text(list_event)).await;

    // ----------------------------------------------------------------
    // TASK 1: Browser se incoming messages receive karo
    // ----------------------------------------------------------------
    let state_recv = state.clone();
    let uid        = user_id.clone();
    let uname      = username.clone();

    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                Message::Text(text) => {
                    handle_incoming_text(&state_recv, &uid, &uname, lat, lon, &text).await;
                }
                Message::Close(_) => break,
                _ => {} // Ping/Pong ignore karo
            }
        }
    });

    // ----------------------------------------------------------------
    // TASK 2: Broadcast channel se messages lo aur WS pe bhejo
    // ----------------------------------------------------------------
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let event = ServerEvent::NewMessage { message: msg };
            let json  = match serde_json::to_string(&event) {
                Ok(j)  => j,
                Err(_) => continue,
            };
            if ws_sender.send(Message::Text(json)).await.is_err() {
                break; // Connection closed
            }
        }
    });

    // ----------------------------------------------------------------
    // Jab bhi koi task khatam ho (disconnect) — cleanup karo
    // ----------------------------------------------------------------
    tokio::select! {
        _ = recv_task => {}
        _ = send_task => {}
    }

    // Online list se hatao
    state.online.remove(&user_id);
    tracing::info!("{} offline hua", username);

    // Baaki users ko batao
    notify_nearby(
        &state, &user_id, lat, lon,
        ServerEvent::UserLeft { username: username.clone() },
    );
}

// =====================================================================
// INCOMING MESSAGE PROCESS KARNA
// =====================================================================

async fn handle_incoming_text(
    state:    &AppState,
    user_id:  &str,
    username: &str,
    lat:      f64,
    lon:      f64,
    text:     &str,
) {
    // JSON parse karo
    let incoming: WsIncoming = match serde_json::from_str(text) {
        Ok(v)  => v,
        Err(e) => {
            tracing::warn!("Invalid WS message from {}: {}", username, e);
            return;
        }
    };

    match incoming {
        // ----- Text message -----
        WsIncoming::SendText { text: content } => {
            if content.trim().is_empty() { return; }

            let msg = ChatMessage {
                from_user:     user_id.to_string(),
                from_username: username.to_string(),
                content:       MessageContent::Text { text: content },
                timestamp:     chrono::Utc::now().to_rfc3339(),
            };

            broadcast_to_nearby(state, user_id, lat, lon, msg);
        }

        // ----- File / Image -----
        WsIncoming::SendFile { url, filename, is_image } => {
            let content = if is_image {
                MessageContent::Image { url, filename }
            } else {
                MessageContent::File  { url, filename }
            };

            let msg = ChatMessage {
                from_user:     user_id.to_string(),
                from_username: username.to_string(),
                content,
                timestamp:     chrono::Utc::now().to_rfc3339(),
            };

            broadcast_to_nearby(state, user_id, lat, lon, msg);
        }

        // ----- Online users list -----
        WsIncoming::GetOnlineUsers => {
            let users = get_nearby_users(state, user_id, lat, lon);
            // Note: yeh direct sender ko bhejne ke liye alag mechanism chahiye
            // Abhi sirf log karte hai — Phase 2 mein fix karenge
            tracing::info!("{} ne {} nearby users dekhe", username, users.len());
        }
    }
}

// =====================================================================
// HELPER: 5 km ke andar sabko ChatMessage bhejo
// =====================================================================

fn broadcast_to_nearby(
    state:        &AppState,
    from_user_id: &str,
    from_lat:     f64,
    from_lon:     f64,
    msg:          ChatMessage,
) {
    let mut sent = 0;
    for entry in state.online.iter() {
        let user = entry.value();
        if user.user_id == from_user_id { continue; }
        if within_radius(from_lat, from_lon, user.lat, user.lon, 5.0) {
            let _ = user.sender.send(msg.clone());
            sent += 1;
        }
    }
    tracing::info!("Message {} logon ko bheja", sent);
}

// =====================================================================
// HELPER: ServerEvent 5 km ke andar sabko bhejo
// =====================================================================

fn notify_nearby(
    state:        &AppState,
    from_user_id: &str,
    lat:          f64,
    lon:          f64,
    event:        ServerEvent,
) {
    let json = match serde_json::to_string(&event) {
        Ok(j)  => j,
        Err(_) => return,
    };

    // Ek dummy ChatMessage banana padega kyunki sender ChatMessage expect karta hai
    // Phase 2 mein hum alag event channel banayenge
    // Abhi ke liye ServerEvent ko Text ke roop mein bhej rahe hai
    let dummy_msg = ChatMessage {
        from_user:     "__server__".to_string(),
        from_username: "__server__".to_string(),
        content:       MessageContent::Text { text: json },
        timestamp:     chrono::Utc::now().to_rfc3339(),
    };

    for entry in state.online.iter() {
        let user = entry.value();
        if user.user_id == from_user_id { continue; }
        if within_radius(lat, lon, user.lat, user.lon, 5.0) {
            let _ = user.sender.send(dummy_msg.clone());
        }
    }
}

// =====================================================================
// HELPER: Nearby online users ki list
// =====================================================================

fn get_nearby_users(
    state:        &AppState,
    my_user_id:   &str,
    my_lat:       f64,
    my_lon:       f64,
) -> Vec<OnlineUser> {
    let mut users: Vec<OnlineUser> = state
        .online
        .iter()
        .filter(|e| e.user_id != my_user_id)
        .filter_map(|e| {
            let u    = e.value();
            let dist = distance_km(my_lat, my_lon, u.lat, u.lon);
            if dist <= 5.0 {
                Some(OnlineUser {
                    user_id:     u.user_id.clone(),
                    username:    u.username.clone(),
                    distance_km: (dist * 10.0).round() / 10.0, // 1 decimal
                })
            } else {
                None
            }
        })
        .collect();

    // Distance ke hisaab se sort karo (nearest first)
    users.sort_by(|a, b| a.distance_km.partial_cmp(&b.distance_km).unwrap());
    users
}

// axum ka into_response use karne ke liye
use axum::response::IntoResponse;
