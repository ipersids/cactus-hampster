import { useEffect, useRef } from 'react';
import type { GameState } from './types';
import { SHIP_RADIUS } from './physics';

interface GameCanvasProps {
  gameState: GameState;
}

function GameCanvas({ gameState }: GameCanvasProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Clear canvas
    ctx.fillStyle = '#0a0a1a';
    ctx.fillRect(0, 0, gameState.arenaWidth, gameState.arenaHeight);

    // Draw stars background
    ctx.fillStyle = '#ffffff22';
    for (let i = 0; i < 100; i++) {
      const x = (i * 137) % gameState.arenaWidth;
      const y = (i * 251) % gameState.arenaHeight;
      ctx.beginPath();
      ctx.arc(x, y, 1, 0, Math.PI * 2);
      ctx.fill();
    }

    // Draw projectiles
    for (const projectile of gameState.projectiles) {
      ctx.fillStyle = '#ffff00';
      ctx.beginPath();
      ctx.arc(projectile.x, projectile.y, 4, 0, Math.PI * 2);
      ctx.fill();

      // Glow effect
      ctx.fillStyle = '#ffff0044';
      ctx.beginPath();
      ctx.arc(projectile.x, projectile.y, 8, 0, Math.PI * 2);
      ctx.fill();
    }

    // Draw ships
    for (const ship of gameState.ships.values()) {
      if (!ship.isAlive) {
        // Draw explosion
        ctx.fillStyle = '#ff440066';
        ctx.beginPath();
        ctx.arc(ship.x, ship.y, SHIP_RADIUS * 2, 0, Math.PI * 2);
        ctx.fill();
        continue;
      }

      ctx.save();
      ctx.translate(ship.x, ship.y);
      ctx.rotate(ship.angle);

      // Ship body (triangle)
      ctx.fillStyle = ship.color;
      ctx.beginPath();
      ctx.moveTo(SHIP_RADIUS, 0);
      ctx.lineTo(-SHIP_RADIUS * 0.7, -SHIP_RADIUS * 0.7);
      ctx.lineTo(-SHIP_RADIUS * 0.4, 0);
      ctx.lineTo(-SHIP_RADIUS * 0.7, SHIP_RADIUS * 0.7);
      ctx.closePath();
      ctx.fill();

      // Ship outline
      ctx.strokeStyle = '#ffffff';
      ctx.lineWidth = 2;
      ctx.stroke();

      ctx.restore();

      // Draw nickname above ship
      ctx.fillStyle = '#ffffff';
      ctx.font = '12px monospace';
      ctx.textAlign = 'center';
      ctx.fillText(ship.nickname, ship.x, ship.y - SHIP_RADIUS - 8);
    }

    // Draw game over overlay
    if (gameState.gameOver) {
      ctx.fillStyle = '#00000088';
      ctx.fillRect(0, 0, gameState.arenaWidth, gameState.arenaHeight);

      ctx.fillStyle = '#ffffff';
      ctx.font = 'bold 48px monospace';
      ctx.textAlign = 'center';
      ctx.fillText('GAME OVER', gameState.arenaWidth / 2, gameState.arenaHeight / 2 - 30);

      ctx.font = '32px monospace';
      if (gameState.winner) {
        ctx.fillStyle = '#4ECDC4';
        ctx.fillText(`${gameState.winner} wins!`, gameState.arenaWidth / 2, gameState.arenaHeight / 2 + 30);
      } else {
        ctx.fillText('Draw!', gameState.arenaWidth / 2, gameState.arenaHeight / 2 + 30);
      }
    }
  }, [gameState]);

  return (
    <canvas
      ref={canvasRef}
      width={gameState.arenaWidth}
      height={gameState.arenaHeight}
      style={{
        display: 'block',
        maxWidth: '100%',
        maxHeight: '100vh',
        margin: '0 auto',
        backgroundColor: '#0a0a1a',
      }}
    />
  );
}

export default GameCanvas;
