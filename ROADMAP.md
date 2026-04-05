# Roadmap

## Phase 1: Session Infrastructure

The foundation — players can create/join sessions and communicate.

### 1.1 Session Management (Server)

- [ ] Session struct: id, join code, host connection, player connections, created_at
- [ ] SessionManager: create, join, leave, get, cleanup expired (5h timeout)
- [ ] Generate 4-6 character join codes (avoid ambiguous chars: 0/O, 1/I/l)
- [ ] In-memory storage (HashMap with RwLock)

### 1.2 Connection Routing

- [ ] Separate WebSocket endpoints: `/ws/host` and `/ws/controller`
- [ ] Host connection flow: authenticate → create session → receive join code
- [ ] Controller connection flow: submit code + nickname → join session
- [ ] Track connection state per session (who's connected)

### 1.3 Message Relay

- [ ] Forward controller inputs to host WebSocket
- [ ] Forward host broadcasts to all controllers in session
- [ ] Handle disconnection gracefully (notify other participants)

### 1.4 Session State Sync

- [ ] Define session state enum: Lobby, InGame, GameOver
- [ ] Host can transition states
- [ ] Broadcast state changes to all participants

**Deliverable**: Host creates session, players join via code, messages relay between them.

---

## Phase 2: Authentication

Subscribers only — keep it simple but extensible.

### 2.1 Auth Provider Integration

- [ ] Choose provider (recommendation: Auth0 or Supabase Auth)
  - Handles email/password, OAuth, email verification
  - Has subscription/role support via metadata
  - SDKs for both Rust and TypeScript
- [ ] JWT validation middleware on server

### 2.2 Subscription Check

- [ ] Verify active subscription before allowing session creation
- [ ] Store subscription status in JWT claims or fetch from provider

### 2.3 Host App Auth Flow

- [ ] Login/signup UI
- [ ] Persist auth token
- [ ] Include token in WebSocket handshake (Sec-WebSocket-Protocol or query param)

**Deliverable**: Only logged-in subscribers can host. Players join without auth.

---

## Phase 3: Game Framework

Generic infrastructure for any game.

### 3.1 Game Trait (Server - minimal)

```rust
trait Game {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn min_players(&self) -> usize;
    fn max_players(&self) -> usize;
}
```

Server just needs metadata for lobby display. All logic is on host.

### 3.2 Game Registry

- [ ] List available games
- [ ] Host selects game → broadcast to controllers
- [ ] Controllers confirm ready / skip

### 3.3 Game Lifecycle Messages

- [ ] `GameSelected { game_id }` — host picks a game
- [ ] `PlayerReady { player_id }` — player opts in
- [ ] `GameStart` — host starts (enough players ready)
- [ ] `GameEnd { results }` — host reports final scores
- [ ] `ReturnToLobby` — back to game selection

### 3.4 Session Score Persistence

- [ ] Accumulate scores across games within session
- [ ] Store in session state (in-memory for now)
- [ ] Display leaderboard in lobby

### 3.5 Controller Input Protocol

- [ ] Generic input message: `{ type: "input", payload: any }`
- [ ] Host interprets payload based on active game
- [ ] Low-latency path: server forwards immediately, no parsing

**Deliverable**: Host can select game, players ready up, game starts/ends, scores persist in session.

---

## Phase 4: First Game — Space Combat

2D spaceship combat with Newtonian physics. Runs entirely on host.

### 4.1 Host Game Engine

- [ ] Game loop (requestAnimationFrame or fixed timestep)
- [ ] Canvas or WebGL rendering
- [ ] Newtonian physics: thrust, inertia, rotation
- [ ] Collision detection (ships, projectiles, boundaries)

### 4.2 Controller UI

- [ ] Rotation controls (left/right or joystick)
- [ ] Thrust button
- [ ] Fire button
- [ ] Touch-optimized layout

### 4.3 Game Logic

- [ ] Ship spawning (distribute around arena)
- [ ] Health/lives system
- [ ] Projectile mechanics
- [ ] Win condition (last ship standing or most kills in time)
- [ ] Round system with respawns

### 4.4 Visual Polish

- [ ] Ship sprites per player (color-coded)
- [ ] Particle effects (thrust, explosions)
- [ ] Arena boundaries visualization
- [ ] Score/health HUD

**Deliverable**: Playable space combat game — players control ships from phones, action displays on TV.

---

## Phase 5: Production Readiness

### 5.1 Infrastructure

- [ ] Database for user accounts (Postgres recommended)
- [ ] Redis for session state (optional, enables horizontal scaling)
- [ ] Deployment setup (Docker, hosting TBD)
- [ ] HTTPS + WSS

### 5.2 Scaling

- [ ] Connection limits per server instance
- [ ] Graceful shutdown (drain sessions)
- [ ] Health checks / monitoring
- [ ] Rate limiting (prevent input spam)

### 5.3 Error Handling

- [ ] Reconnection logic (controller reconnects to active session)
- [ ] Host disconnect handling (end session? transfer? timeout?)
- [ ] Network error UI on clients

### 5.4 QR Code Flow

- [ ] Generate QR on host showing join URL + session code
- [ ] Controller landing page: auto-fill code from URL, prompt nickname

### 5.5 Landing Page / Marketing Site

- [ ] What is this, how it works
- [ ] Subscription signup
- [ ] Link to host app

---

## Future Ideas (Backlog)

- PWA for controller (add to home screen)
- Spectator mode
- Custom game uploads (user-generated games)
- Tournament mode (brackets)
- Voice chat integration
- Game replays
- More games: trivia, drawing, racing, word games

---

## Open Questions

- **Hot joining**: Allow mid-game joins? Spectate only? Defer to Phase 5+
- **Host transfer**: If host disconnects, can another player become host?
- **Binary protocol**: Switch to MessagePack for lower latency? Benchmark first.
- **Offline/LAN mode**: Run server locally for no-internet play?
