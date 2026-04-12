# VideoConnect — Phase 1 Complete Guide
## Rust + Axum + WebSocket | Local Chat + File Share + 5km Radius

---

## STEP 1 — Rust Install Karo

### Windows:
1. https://rustup.rs pe jao
2. `rustup-init.exe` download karo
3. Install karo (default options theek hain)
4. Terminal reopen karo
5. Check karo: `rustc --version`

### Linux / Mac:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version   # 1.70+ dikhna chahiye
```

---

## STEP 2 — Project Structure Banao

```bash
cargo new videoconnect
cd videoconnect

# Folders banao
mkdir -p src/auth
mkdir -p src/chat
mkdir -p src/location
mkdir -p src/files
mkdir -p static
mkdir -p uploads
```

---

## STEP 3 — Files Copy Karo

Neeche di gayi har file apne project mein EXACTLY usi path pe rakho:

```
videoconnect/
├── Cargo.toml
├── static/
│   └── index.html          ← Frontend
├── uploads/                ← Photos/files yahan jayenge (khali rakho)
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
```

---

## STEP 4 — Server Chalao

```bash
cd videoconnect
cargo run
```

Pehli baar mein 2-5 minute lagenge (dependencies download hongi).
Yeh dikhega jab ready ho:

```
INFO videoconnect: ==========================================
INFO videoconnect:   VideoConnect server chal raha hai!
INFO videoconnect:   http://localhost:3000
INFO videoconnect: ==========================================
```

---

## STEP 5 — Test Karo

### Browser test:
1. `http://localhost:3000` kholo
2. Signup karo — GPS button dabao ya lat/lon manually daalo
   - Mumbai test ke liye: lat = 19.0760, lon = 72.8777
3. Doosre browser ya incognito mein signup karo (alag username, same location)
4. Chat karo! File bhi bhej sakte ho 📎

### API test (optional — Postman ya curl):

**Signup:**
```bash
curl -X POST http://localhost:3000/signup \
  -H "Content-Type: application/json" \
  -d '{"username":"ali","password":"pass123","lat":19.0760,"lon":72.8777}'
```

**Login:**
```bash
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"username":"ali","password":"pass123"}'
```

**WebSocket test (wscat tool se):**
```bash
npm install -g wscat
wscat -c "ws://localhost:3000/ws?token=YOUR_TOKEN_HERE"

# Phir type karo:
{"type":"send_text","text":"Hello!"}
{"type":"get_online_users"}
```

---

## PHASE 1 DONE CHECK ✅

- [ ] 3 browsers mein chat kar sako
- [ ] Image share ho jaye
- [ ] File share ho jaye
- [ ] Online users list live update ho
- [ ] Signup/Login kaam kare

---

## COMMON ERRORS AUR FIXES

### Error: `error[E0433]: failed to resolve`
**Fix:** `cargo clean` phir `cargo run`

### Error: `Address already in use`
**Fix:** Port 3000 koi aur use kar raha hai
```bash
# Windows
netstat -ano | findstr :3000
taskkill /PID <pid> /F

# Linux/Mac
lsof -ti:3000 | xargs kill
```

### Error: Cargo.toml mein version issue
**Fix:** `cargo update` run karo

### Browser mein "Cannot connect to WebSocket"
**Fix:** Server chal raha hai? `cargo run` karo pehle

### GPS kaam nahi kar raha signup mein
**Fix:** Manually daalo — Mumbai: `19.0760, 72.8777` | Delhi: `28.6139, 77.2090`

---

## WEBSOCKET MESSAGE FORMAT

### Browser → Server:

```json
// Text bhejo
{"type": "send_text", "text": "Hello bhai!"}

// File/Image bhejo (pehle /upload se URL lo)
{"type": "send_file", "url": "/files/abc.jpg", "filename": "photo.jpg", "is_image": true}

// Online users maango
{"type": "get_online_users"}
```

### Server → Browser:

```json
// Naya message
{"event": "new_message", "message": {...}}

// Online users list
{"event": "online_users", "users": [{"username":"ali","distance_km":0.5}]}

// User join/leave
{"event": "user_joined", "username": "ali"}
{"event": "user_left",   "username": "ali"}
```

---

## NEXT STEPS — Phase 2 Ready Hone Ke Baad

- [ ] SQLite se messages save karo (ab memory mein hain, restart pe gayab)
- [ ] Rate limiting (spam rokna)
- [ ] Input validation aur better error handling
- [ ] Reconnect par message history

---

## PROJECT INFO

- **Stack:** Rust + Axum + WebSocket + HTML/CSS/JS
- **Phase:** 1 of 4
- **Port:** 3000
- **Radius:** 5 km (Haversine formula)
- **Auth:** JWT (7 days valid)
- **Storage:** In-memory (Phase 2 mein DB aayega)
