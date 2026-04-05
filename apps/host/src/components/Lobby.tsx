import type { PlayerInfo } from '../hook/useHostWebSocket';

interface LobbyProps {
  sessionCode: string;
  players: PlayerInfo[];
  onStartGame: (gameType: string) => void;
}

const GAMES = [
  { id: 'spaceBattle', name: 'Space Battle' },
];

function Lobby({ sessionCode, players, onStartGame }: LobbyProps) {
  return (
    <div className="lobby">
      <div className="join-code-section">
        <h2>Join Code</h2>
        <div className="join-code">{sessionCode}</div>
        <p className="join-hint">Enter this code on your phone to join</p>
      </div>

      <div className="players-section">
        <h2>Players ({players.length}/8)</h2>
        {players.length === 0 ? (
          <p className="waiting">Waiting for players to join...</p>
        ) : (
          <ul className="player-list">
            {players.map((player, index) => (
              <li key={player.playerId} className="player-item">
                <span className="player-color" style={{ backgroundColor: getPlayerColor(index) }} />
                {player.nickname}
              </li>
            ))}
          </ul>
        )}
      </div>

      <div className="game-section">
        <h2>Select Game</h2>
        <div className="game-list">
          {GAMES.map(game => (
            <button
              key={game.id}
              className="game-button"
              disabled={players.length < 1}
              onClick={() => onStartGame(game.id)}
            >
              {game.name}
              {players.length < 1 && <span className="min-players">(Need at least 1 player)</span>}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}

function getPlayerColor(index: number): string {
  const colors = [
    '#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4',
    '#FFEAA7', '#DDA0DD', '#98D8C8', '#F7DC6F',
  ];
  return colors[index % colors.length];
}

export default Lobby;
export { getPlayerColor };
