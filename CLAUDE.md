# Cactus Hampster - Party Game Platform

## Project Overview

A platform for party games where players use their phones as controllers and the game displays on a TV/monitor. The host (account holder) creates a session, players scan a QR code to join, and games run in the browser.

**Architecture**: Host-authoritative. The server manages sessions and relays messages; the host app runs all game logic and rendering.

## Claude instructions

- Never run the server, controller or the host. Developer will run them

## Tech Stack

- **Server**: Rust (Axum, Tokio) — WebSocket server, session management
- **Host App**: TypeScript/React (Vite) — game rendering, physics, state
- **Controller App**: TypeScript/React (Vite) — player input UI
- **Shared Types**: TypeShare generates TS types from Rust structs
- **Monorepo**: pnpm workspaces

## Project Structure

```
apps/
  controller/     # Player phone app (React)
  host/           # TV display app (React)
packages/
  websocket/      # Shared WebSocket hook
  typeshare/      # Generated TS types from Rust
server/
  crates/
    server/       # Axum WebSocket server
    model/        # Shared types (Rust source of truth)
docs/             # Planning screenshots and decisions
```

## Key Commands

```bash
# Install deps + generate types
pnpm install

# Run both frontend apps (host: 5173, controller: 5174)
pnpm dev

# Run server (default: 127.0.0.1:3000)
cd server && cargo run

# Regenerate TS types from Rust
pnpm typeshare
```

## Architecture Decisions

### Communication Flow

```
Controller → Server → Host (inputs)
Host → Server → Controllers (game state updates, if needed)
```

- Server is a message relay + session manager
- Host owns game state, physics, rendering
- Server validates session membership but not game logic

### Session Model

- Subscriber creates session → gets join code/QR
- Players join via code + nickname (no auth required)
- Session persists across multiple games (scores accumulate)
- Session ends: 5h timeout, last player leaves, or host logs out
- No database needed for sessions at least for now. When session ends, its data does not persist

### Type Sharing

Rust structs in `server/crates/model/src/shared/` are the source of truth. TypeShare generates `packages/typeshare/types/index.ts` on `pnpm install`.

When adding new message types:

1. Define struct in Rust with `#[typeshare]` attribute
2. Run `pnpm typeshare`
3. Import generated types in TS apps

## Code Conventions

### Rust (Server)

- Crates: `server` (binary), `model` (types)
- Use `shared_types::` module for cross-language types
- Local-only types go in `local_types.rs`

### TypeScript (Apps)

- Hooks in `src/hook/`
- Shared packages imported via workspace (e.g., `@cactus/websocket`)
- WebSocket hooks wrap `react-use-websocket`

## Current State

**Working:**

- WebSocket connection with heartbeat (ping/pong)
- Type generation pipeline (Rust → TypeScript)
- Basic message echo

**Not yet implemented:**

- Session management (create, join, leave)
- Player/host role assignment
- Game logic framework
- Authentication
- Multiple concurrent sessions

## Development Notes

- Controller runs on port 5174, Host on 5173, Server on 3000
- CORS not yet configured — will need it for production
- No database yet — sessions are in-memory only
