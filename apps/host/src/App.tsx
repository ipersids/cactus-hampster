import { useState, useEffect, useCallback } from 'react';
import useHostWebSocket from './hook/useHostWebSocket';
import Lobby from './components/Lobby';
import SpaceBattleGame from './components/games/SpaceBattle/SpaceBattleGame';
import './App.css';

type GamePhase = 'connecting' | 'lobby' | 'playing';

function App() {
  const [phase, setPhase] = useState<GamePhase>('connecting');
  const [currentGame, setCurrentGame] = useState<string | null>(null);

  const {
    isConnected,
    status,
    connect,
    sessionCode,
    players,
    getPlayerInputs,
    createSession,
    startGame,
  } = useHostWebSocket();

  // Connect and create session on mount
  useEffect(() => {
    if (!isConnected && phase === 'connecting') {
      connect();
    }
  }, [isConnected, phase, connect]);

  // Create session once connected
  useEffect(() => {
    if (isConnected && !sessionCode) {
      createSession();
    }
  }, [isConnected, sessionCode, createSession]);

  // Move to lobby once session is created
  useEffect(() => {
    if (sessionCode && phase === 'connecting') {
      setPhase('lobby');
    }
  }, [sessionCode, phase]);

  const handleStartGame = useCallback((gameType: string) => {
    startGame(gameType);
    setCurrentGame(gameType);
    setPhase('playing');
  }, [startGame]);

  const handleGameEnd = useCallback(() => {
    setCurrentGame(null);
    setPhase('lobby');
  }, []);

  // Connecting state
  if (phase === 'connecting') {
    return (
      <section id="center">
        <div className="connecting">
          <h1>Party Games</h1>
          <p>Connecting to server... ({status})</p>
        </div>
      </section>
    );
  }

  // Lobby state
  if (phase === 'lobby' && sessionCode) {
    return (
      <section id="center">
        <Lobby
          sessionCode={sessionCode}
          players={players}
          onStartGame={handleStartGame}
        />
      </section>
    );
  }

  // Playing state
  if (phase === 'playing' && currentGame === 'spaceBattle') {
    return (
      <SpaceBattleGame
        players={players}
        getPlayerInputs={getPlayerInputs}
        onGameEnd={handleGameEnd}
      />
    );
  }

  return (
    <section id="center">
      <p>Something went wrong. Please refresh.</p>
    </section>
  );
}

export default App;
