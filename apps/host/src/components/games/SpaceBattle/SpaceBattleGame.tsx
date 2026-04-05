import { useState, useEffect, useRef, useCallback } from 'react';
import type { GameState, Ship } from './types';
import type { PlayerInputPayload } from '@cactus-hampster/typeshare';
import { updateGameState } from './physics';
import GameCanvas from './GameCanvas';
import { getPlayerColor } from '../../Lobby';

interface SpaceBattleGameProps {
  players: Array<{ playerId: string; nickname: string }>;
  getPlayerInputs: () => Map<string, PlayerInputPayload>;
  onGameEnd: () => void;
}

const ARENA_WIDTH = 1280;
const ARENA_HEIGHT = 720;

function createInitialState(players: Array<{ playerId: string; nickname: string }>): GameState {
  const ships = new Map<string, Ship>();

  players.forEach((player, index) => {
    // Spawn ships in a circle
    const angle = (index / players.length) * Math.PI * 2;
    const spawnRadius = Math.min(ARENA_WIDTH, ARENA_HEIGHT) * 0.35;

    ships.set(player.playerId, {
      playerId: player.playerId,
      nickname: player.nickname,
      x: ARENA_WIDTH / 2 + Math.cos(angle) * spawnRadius,
      y: ARENA_HEIGHT / 2 + Math.sin(angle) * spawnRadius,
      angle: angle + Math.PI, // Face center
      velocityX: 0,
      velocityY: 0,
      isAlive: true,
      color: getPlayerColor(index),
      fireCooldown: 0,
    });
  });

  return {
    ships,
    projectiles: [],
    arenaWidth: ARENA_WIDTH,
    arenaHeight: ARENA_HEIGHT,
    gameOver: false,
    winner: null,
  };
}

function SpaceBattleGame({ players, getPlayerInputs, onGameEnd }: SpaceBattleGameProps) {
  const [gameState, setGameState] = useState<GameState>(() => createInitialState(players));
  const gameStateRef = useRef(gameState);
  const lastTimeRef = useRef<number>(0);
  const frameRef = useRef<number | undefined>(undefined);

  useEffect(() => {
    gameStateRef.current = gameState;
  }, [gameState]);

  const gameLoop = useCallback((timestamp: number) => {
    if (lastTimeRef.current === 0) {
      lastTimeRef.current = timestamp;
    }

    const dt = Math.min((timestamp - lastTimeRef.current) / 1000, 0.1); // Cap at 100ms
    lastTimeRef.current = timestamp;

    const inputs = getPlayerInputs();
    const newState = updateGameState(gameStateRef.current, inputs, dt);
    setGameState(newState);

    if (!newState.gameOver) {
      frameRef.current = requestAnimationFrame(gameLoop);
    }
  }, [getPlayerInputs]);

  useEffect(() => {
    frameRef.current = requestAnimationFrame(gameLoop);

    return () => {
      if (frameRef.current) {
        cancelAnimationFrame(frameRef.current);
      }
    };
  }, [gameLoop]);

  return (
    <div className="space-battle">
      <GameCanvas gameState={gameState} />
      {gameState.gameOver && (
        <div className="game-over-controls">
          <button onClick={onGameEnd} className="back-to-lobby">
            Back to Lobby
          </button>
        </div>
      )}
    </div>
  );
}

export default SpaceBattleGame;
