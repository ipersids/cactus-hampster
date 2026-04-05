import { useEffect, useCallback } from 'react';
import useControllerWebSocket from './hook/useControllerWebSocket';
import JoinScreen from './components/JoinScreen';
import WaitingScreen from './components/WaitingScreen';
import GameController from './components/GameController';
import './App.css';

function App() {
  const {
    isConnected,
    connect,
    phase,
    sessionCode,
    errorMessage,
    joinSession,
    sendInput,
  } = useControllerWebSocket();

  // Connect on mount
  useEffect(() => {
    if (!isConnected) {
      connect();
    }
  }, [isConnected, connect]);

  const handleJoin = useCallback((code: string, nickname: string) => {
    joinSession(code, nickname);
  }, [joinSession]);

  // Error state
  if (phase === 'error') {
    return (
      <section id="center">
        <div className="error-screen">
          <h1>Error</h1>
          <p>{errorMessage || 'Something went wrong'}</p>
          <button onClick={() => window.location.reload()} className="retry-button">
            Try Again
          </button>
        </div>
      </section>
    );
  }

  // Connecting state - only show when WebSocket is not connected
  if (!isConnected) {
    return (
      <section id="center">
        <div className="connecting">
          <p>Connecting...</p>
        </div>
      </section>
    );
  }

  // Join screen (when connected but not yet in a session)
  if (phase !== 'waiting' && phase !== 'playing') {
    return (
      <section id="center">
        <JoinScreen onJoin={handleJoin} isConnecting={phase === 'joining'} />
      </section>
    );
  }

  // Waiting for game to start
  if (phase === 'waiting' && sessionCode) {
    return (
      <section id="center">
        <WaitingScreen sessionCode={sessionCode} />
      </section>
    );
  }

  // Playing
  if (phase === 'playing') {
    return <GameController onInput={sendInput} />;
  }

  // Default join screen
  return (
    <section id="center">
      <JoinScreen onJoin={handleJoin} isConnecting={false} />
    </section>
  );
}

export default App;
