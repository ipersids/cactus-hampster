import { useState } from 'react';

interface JoinScreenProps {
  onJoin: (code: string, nickname: string) => void;
  isConnecting: boolean;
}

function JoinScreen({ onJoin, isConnecting }: JoinScreenProps) {
  const [code, setCode] = useState('');
  const [nickname, setNickname] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (code.length === 4 && nickname.length >= 1) {
      onJoin(code, nickname);
    }
  };

  return (
    <div className="join-screen">
      <h1>Join Game</h1>
      <form onSubmit={handleSubmit}>
        <div className="input-group">
          <label htmlFor="code">Game Code</label>
          <input
            id="code"
            type="text"
            inputMode="numeric"
            pattern="[0-9]*"
            maxLength={4}
            placeholder="0000"
            value={code}
            onChange={e => setCode(e.target.value.replace(/\D/g, ''))}
            className="code-input"
            autoComplete="off"
          />
        </div>

        <div className="input-group">
          <label htmlFor="nickname">Nickname</label>
          <input
            id="nickname"
            type="text"
            maxLength={12}
            placeholder="Your name"
            value={nickname}
            onChange={e => setNickname(e.target.value)}
            className="nickname-input"
            autoComplete="off"
          />
        </div>

        <button
          type="submit"
          disabled={code.length !== 4 || nickname.length < 1 || isConnecting}
          className="join-button"
        >
          {isConnecting ? 'Joining...' : 'Join Game'}
        </button>
      </form>
    </div>
  );
}

export default JoinScreen;
