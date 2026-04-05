export interface Ship {
  playerId: string;
  nickname: string;
  x: number;
  y: number;
  angle: number;
  velocityX: number;
  velocityY: number;
  isAlive: boolean;
  color: string;
  fireCooldown: number;
}

export interface Projectile {
  id: string;
  ownerId: string;
  x: number;
  y: number;
  velocityX: number;
  velocityY: number;
  lifetime: number;
}

export interface GameState {
  ships: Map<string, Ship>;
  projectiles: Projectile[];
  arenaWidth: number;
  arenaHeight: number;
  gameOver: boolean;
  winner: string | null;
}

export interface PlayerInput {
  thrust: boolean;
  rotateLeft: boolean;
  rotateRight: boolean;
  fire: boolean;
}
