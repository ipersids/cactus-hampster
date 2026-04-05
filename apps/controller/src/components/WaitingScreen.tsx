interface WaitingScreenProps {
  sessionCode: string;
}

function WaitingScreen({ sessionCode }: WaitingScreenProps) {
  return (
    <div className="waiting-screen">
      <div className="connected-badge">Connected!</div>
      <h2>Session {sessionCode}</h2>
      <p className="waiting-text">Waiting for host to start the game...</p>
      <div className="pulse-dot" />
    </div>
  );
}

export default WaitingScreen;
