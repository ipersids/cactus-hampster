import type { Ship, Projectile, GameState, PlayerInput } from './types';

// Physics constants
const THRUST_FORCE = 300;
const ROTATION_SPEED = 4;
const MAX_VELOCITY = 350;
const DRAG = 0.99;
const PROJECTILE_SPEED = 500;
const PROJECTILE_LIFETIME = 2;
const FIRE_COOLDOWN = 0.3;
const SHIP_RADIUS = 15;
const PROJECTILE_RADIUS = 4;

export function applyInput(ship: Ship, input: PlayerInput | undefined, dt: number): void {
  if (!input || !ship.isAlive) return;

  // Rotation
  if (input.rotateLeft) {
    ship.angle -= ROTATION_SPEED * dt;
  }
  if (input.rotateRight) {
    ship.angle += ROTATION_SPEED * dt;
  }

  // Thrust
  if (input.thrust) {
    ship.velocityX += Math.cos(ship.angle) * THRUST_FORCE * dt;
    ship.velocityY += Math.sin(ship.angle) * THRUST_FORCE * dt;

    // Clamp velocity
    const speed = Math.sqrt(ship.velocityX ** 2 + ship.velocityY ** 2);
    if (speed > MAX_VELOCITY) {
      ship.velocityX = (ship.velocityX / speed) * MAX_VELOCITY;
      ship.velocityY = (ship.velocityY / speed) * MAX_VELOCITY;
    }
  }

  // Apply drag when not thrusting
  if (!input.thrust) {
    ship.velocityX *= DRAG;
    ship.velocityY *= DRAG;
  }
}

export function updateShipPosition(ship: Ship, dt: number, arenaWidth: number, arenaHeight: number): void {
  if (!ship.isAlive) return;

  ship.x += ship.velocityX * dt;
  ship.y += ship.velocityY * dt;

  // Wrap around edges
  if (ship.x < 0) ship.x += arenaWidth;
  if (ship.x > arenaWidth) ship.x -= arenaWidth;
  if (ship.y < 0) ship.y += arenaHeight;
  if (ship.y > arenaHeight) ship.y -= arenaHeight;

  // Update fire cooldown
  if (ship.fireCooldown > 0) {
    ship.fireCooldown -= dt;
  }
}

export function tryFire(ship: Ship, input: PlayerInput | undefined, projectiles: Projectile[]): void {
  if (!input || !ship.isAlive || !input.fire || ship.fireCooldown > 0) return;

  ship.fireCooldown = FIRE_COOLDOWN;

  const projectile: Projectile = {
    id: `${ship.playerId}-${Date.now()}-${Math.random()}`,
    ownerId: ship.playerId,
    x: ship.x + Math.cos(ship.angle) * (SHIP_RADIUS + 5),
    y: ship.y + Math.sin(ship.angle) * (SHIP_RADIUS + 5),
    velocityX: Math.cos(ship.angle) * PROJECTILE_SPEED + ship.velocityX * 0.3,
    velocityY: Math.sin(ship.angle) * PROJECTILE_SPEED + ship.velocityY * 0.3,
    lifetime: PROJECTILE_LIFETIME,
  };

  projectiles.push(projectile);
}

export function updateProjectiles(projectiles: Projectile[], dt: number, arenaWidth: number, arenaHeight: number): Projectile[] {
  return projectiles
    .map(p => ({
      ...p,
      x: ((p.x + p.velocityX * dt) % arenaWidth + arenaWidth) % arenaWidth,
      y: ((p.y + p.velocityY * dt) % arenaHeight + arenaHeight) % arenaHeight,
      lifetime: p.lifetime - dt,
    }))
    .filter(p => p.lifetime > 0);
}

export function checkCollisions(ships: Map<string, Ship>, projectiles: Projectile[]): Projectile[] {
  const remainingProjectiles: Projectile[] = [];

  for (const projectile of projectiles) {
    let hit = false;

    for (const ship of ships.values()) {
      if (!ship.isAlive) continue;
      if (ship.playerId === projectile.ownerId) continue;

      const dx = ship.x - projectile.x;
      const dy = ship.y - projectile.y;
      const distance = Math.sqrt(dx * dx + dy * dy);

      if (distance < SHIP_RADIUS + PROJECTILE_RADIUS) {
        ship.isAlive = false;
        hit = true;
        break;
      }
    }

    if (!hit) {
      remainingProjectiles.push(projectile);
    }
  }

  return remainingProjectiles;
}

export function checkWinCondition(ships: Map<string, Ship>): { gameOver: boolean; winner: string | null } {
  const aliveShips = Array.from(ships.values()).filter(s => s.isAlive);

  if (aliveShips.length <= 1 && ships.size > 1) {
    return {
      gameOver: true,
      winner: aliveShips.length === 1 ? aliveShips[0].nickname : null,
    };
  }

  return { gameOver: false, winner: null };
}

export function updateGameState(
  state: GameState,
  inputs: Map<string, { thrust: boolean; rotate_left: boolean; rotate_right: boolean; fire: boolean }>,
  dt: number
): GameState {
  if (state.gameOver) return state;

  const newShips = new Map(state.ships);
  let newProjectiles = [...state.projectiles];


  // Convert inputs and apply to ships
  for (const [playerId, ship] of newShips) {
    const rawInput = inputs.get(playerId);
    const input: PlayerInput | undefined = rawInput
      ? {
          thrust: rawInput.thrust,
          rotateLeft: rawInput.rotate_left,
          rotateRight: rawInput.rotate_right,
          fire: rawInput.fire,
        }
      : undefined;

    applyInput(ship, input, dt);
    updateShipPosition(ship, dt, state.arenaWidth, state.arenaHeight);
    tryFire(ship, input, newProjectiles);
  }

  // Update projectiles
  newProjectiles = updateProjectiles(newProjectiles, dt, state.arenaWidth, state.arenaHeight);

  // Check collisions
  newProjectiles = checkCollisions(newShips, newProjectiles);

  // Check win condition
  const { gameOver, winner } = checkWinCondition(newShips);

  return {
    ...state,
    ships: newShips,
    projectiles: newProjectiles,
    gameOver,
    winner,
  };
}

export { SHIP_RADIUS };
