⏺ PR Breakdown

PR 1: Server Session Management & Types

Goal: Server can create sessions, players can join, messages route between host and controllers.

Changes:

- server/crates/model/src/shared/common_types.rs - Add payload structs (SessionCreatedPayload,
  JoinSessionPayload, JoinSuccessPayload, PlayerJoinedPayload, PlayerLeftPayload, PlayerInputPayload,
  ControllerInputPayload, StartGamePayload, GameStartedPayload, PlayerInfo)
- server/crates/model/src/shared/host_types.rs - Extend HostEventType and ServerToHostEventType enums
- server/crates/model/src/shared/controller_types.rs - Extend ControllerEventType and
  ServerToControllerEventType enums
- server/crates/model/src/lib.rs - Add re-exports for new types
- server/crates/server/src/state.rs - New file with AppState, Session, Player structs and session
  management methods
- server/crates/server/src/handler.rs - Split into host_ws/controller_ws handlers, message routing
- server/crates/server/src/router.rs - Add /ws/host and /ws/controller routes with state
- server/crates/server/src/main.rs - Create and pass AppState
- server/crates/server/src/lib.rs - Export state module
- server/crates/server/Cargo.toml - Add uuid, rand, futures deps
- Run pnpm typeshare

Success criteria:

- Server compiles
- TypeScript types generate
- Can connect to /ws/host and /ws/controller endpoints

---

PR 2: Host Lobby

Goal: Host can create a session and display the join code + player list.

Changes:

- apps/host/vite.config.ts - Update proxy from /ping to /ws, port 8080
- apps/host/src/hook/useHostWebSocket.ts - Handle sessionCreated, playerJoined, playerLeft events; add
  createSession(), sessionCode, players state
- apps/host/src/components/Lobby.tsx - New component with join code display, player list, game selector
- apps/host/src/App.tsx - Add phase state machine (connecting → lobby → playing), wire up Lobby
- apps/host/src/App.css - Lobby styles

Success criteria:

- Host connects and automatically creates session
- 4-digit code displays
- (Manually test with wscat or similar that joining works)

---

PR 3: Controller Join Flow

Goal: Controller can enter code + nickname and join a session.

Changes:

- apps/controller/vite.config.ts - Update proxy from /ping to /ws, port 8080
- apps/controller/src/hook/useControllerWebSocket.ts - Handle joinSuccess, gameStarted events; add
  joinSession(), phase state
- apps/controller/src/components/JoinScreen.tsx - Code + nickname input form
- apps/controller/src/components/WaitingScreen.tsx - "Connected, waiting for game" display
- apps/controller/src/App.tsx - Phase state machine, wire up components
- apps/controller/src/App.css - Join/waiting screen styles
- Delete apps/controller/src/tests/test-proto.ts (obsolete)

Success criteria:

- Controller shows join screen
- Enter code + nickname → shows waiting screen
- Host lobby shows player joined

---

PR 4: Controller Input Flow

Goal: Controller sends button inputs, host receives them.

Changes:

- apps/controller/src/components/GameController.tsx - Touch buttons for thrust, rotate L/R, fire
- apps/controller/src/App.css - Controller button styles
- apps/host/src/hook/useHostWebSocket.ts - Handle playerInput events, store in ref for game loop access
  (update ref directly, not via setState)

Success criteria:

- Controller shows game buttons when game starts (can mock gameStarted event)
- Host receives playerInput events (verify with console.log)

---

PR 5: Space Battle Game

Goal: Playable game with ships, physics, shooting.

Changes:

- apps/host/src/components/games/SpaceBattle/types.ts - Ship, Projectile, GameState, PlayerInput types
- apps/host/src/components/games/SpaceBattle/physics.ts - Movement, collision, win condition logic
- apps/host/src/components/games/SpaceBattle/GameCanvas.tsx - Canvas rendering
- apps/host/src/components/games/SpaceBattle/SpaceBattleGame.tsx - Game loop with requestAnimationFrame,
  state management
- apps/host/src/components/Lobby.tsx - Export getPlayerColor helper
- apps/host/src/App.tsx - Wire up game phase, pass getPlayerInputs to game
- apps/host/src/App.css - Game styles

Success criteria:

- Click "Space Battle" in lobby → game starts
- Ships render, respond to controller input
- Projectiles fire, collisions kill ships
- Last ship standing wins
- "Back to Lobby" returns to lobby

---

Testing Note

Remember: Host and controller must be in separate browser windows (not tabs) because
requestAnimationFrame pauses in background tabs.