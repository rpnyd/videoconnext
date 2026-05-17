VideoConnect — Phase 1 Complete Guide

Rust + Axum + WebSocket | Local Chat + File Sharing + 5km Radius


---

STEP 1 — Install Rust

Windows:

1. Go to [Rustup](https://rustup.rs?utm_source=chatgpt.com)


2. Download rustup-init.exe


3. Install it (default options are fine)


4. Reopen the terminal


5. Verify installation:



rustc --version

Linux / macOS:

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version   # Should show 1.70+


---

STEP 2 — Create the Project Structure

cargo new videoconnect
cd videoconnect

# Create folders
mkdir -p src/auth
mkdir -p src/chat
mkdir -p src/location
mkdir -p src/files
mkdir -p static
mkdir -p uploads


---

STEP 3 — Copy the Files

Place every file below in the EXACT same path inside your project:

videoconnect/
├── Cargo.toml
├── static/
│   └── index.html          ← Frontend
├── uploads/                ← Uploaded photos/files will go here (keep empty)
└── src/
    ├── main.rs
    ├── state.rs
    ├── models.rs
    ├── auth/
    │   ├── mod.rs
    │   ├── handlers.rs
    │   └── jwt.rs
    ├── chat/
    │   ├── mod.rs
    │   └── ws.rs
    ├── location/
    │   ├── mod.rs
    │   └── haversine.rs
    └── files/
        ├── mod.rs
        └── handlers.rs


---

STEP 4 — Run the Server

cd videoconnect
cargo run

The first run may take 2–5 minutes because dependencies will be downloaded.

When the server is ready, you should see:

INFO videoconnect: ==========================================
INFO videoconnect:   VideoConnect server is running!
INFO videoconnect:   http://localhost:3000
INFO videoconnect: ==========================================


---

STEP 5 — Test the App

Browser Test

1. Open: http://localhost:3000


2. Sign up

Press the GPS button OR manually enter latitude/longitude

Example for Mumbai:

lat = 19.0760

lon = 72.8777




3. Open another browser or incognito window


4. Sign up using a different username but the same location


5. Start chatting! You can also share files 📎




---

API Test (Optional — Postman or curl)

Signup

curl -X POST http://localhost:3000/signup \
  -H "Content-Type: application/json" \
  -d '{"username":"ali","password":"pass123","lat":19.0760,"lon":72.8777}'

Login

curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"username":"ali","password":"pass123"}'

WebSocket Test (Using wscat)

Install wscat:

npm install -g wscat

Connect:

wscat -c "ws://localhost:3000/ws?token=YOUR_TOKEN_HERE"

Send messages:

{"type":"send_text","text":"Hello!"}
{"type":"get_online_users"}


---

PHASE 1 COMPLETION CHECKLIST ✅

[ ] Chat works across 3 browser windows

[ ] Image sharing works

[ ] File sharing works

[ ] Online users update live

[ ] Signup/Login works correctly



---

COMMON ERRORS AND FIXES

Error: error[E0433]: failed to resolve

Fix:

cargo clean
cargo run


---

Error: Address already in use

Cause:

Port 3000 is already being used.

Fix:

Windows

netstat -ano | findstr :3000
taskkill /PID <pid> /F

Linux/macOS

lsof -ti:3000 | xargs kill


---

Error: Version issue in Cargo.toml

Fix:

cargo update


---

Browser says “Cannot connect to WebSocket”

Fix:

Make sure the server is running:

cargo run


---

GPS does not work during signup

Fix:

Enter coordinates manually.

Examples:

Mumbai → 19.0760, 72.8777

Delhi → 28.6139, 77.2090



---

WEBSOCKET MESSAGE FORMAT

Browser → Server

Send text message

{"type": "send_text", "text": "Hello bro!"}

Send file/image

(Upload first using /upload and get the URL)

{
  "type": "send_file",
  "url": "/files/abc.jpg",
  "filename": "photo.jpg",
  "is_image": true
}

Request online users

{"type": "get_online_users"}


---

Server → Browser

New message

{"event": "new_message", "message": {...}}

Online users list

{
  "event": "online_users",
  "users": [
    {
      "username":"ali",
      "distance_km":0.5
    }
  ]
}

User joined/left

{"event": "user_joined", "username": "ali"}
{"event": "user_left",   "username": "ali"}


---

NEXT STEPS — After Phase 1

[ ] Save messages using SQLite (currently stored in memory only)

[ ] Add rate limiting (prevent spam)

[ ] Improve input validation and error handling

[ ] Restore chat history after reconnect



---

PROJECT INFO

Stack: Rust + Axum + WebSocket + HTML/CSS/JS

Current Phase: 1 of 4

Port: 3000

Chat Radius: 5 km (using Haversine formula)

Authentication: JWT (valid for 7 days)

Storage: In-memory (database coming in Phase 2)