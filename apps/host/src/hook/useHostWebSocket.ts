import useGameWebSocket, { createEvent } from '@cactus-hampster/websocket';
import type {
  ServerToHostEvent as ServerEvent,
  HostEvent,
  PlayerJoinedPayload,
  PlayerLeftPayload,
  PlayerInputPayload,
} from '@cactus-hampster/typeshare';
import { useState, useCallback, useRef, useEffect } from 'react';

const SOCKET_URL = '/ws/host';

export const createHostEvent = (event: HostEvent) => createEvent(event);

export interface PlayerInfo {
  playerId: string;
  nickname: string;
}

const useHostWebSocket = () => {
  const [sessionCode, setSessionCode] = useState<string | null>(null);
  const [players, setPlayers] = useState<PlayerInfo[]>([]);
  const [playerInputs, setPlayerInputs] = useState<Map<string, PlayerInputPayload>>(new Map());
  const playerInputsRef = useRef(playerInputs);

  useEffect(() => {
    playerInputsRef.current = playerInputs;
  }, [playerInputs]);

  const handleEvent = useCallback((event: ServerEvent) => {
    if (event.status === 'success') {
      switch (event.data.type) {
        case 'sessionCreated':
          setSessionCode(event.data.payload.session_code);
          break;
        case 'playerJoined': {
          const payload = event.data.payload as PlayerJoinedPayload;
          setPlayers(prev => [...prev, { playerId: payload.player_id, nickname: payload.nickname }]);
          break;
        }
        case 'playerLeft': {
          const payload = event.data.payload as PlayerLeftPayload;
          setPlayers(prev => prev.filter(p => p.playerId !== payload.player_id));
          setPlayerInputs(prev => {
            const next = new Map(prev);
            next.delete(payload.player_id);
            return next;
          });
          break;
        }
        case 'playerInput': {
          const payload = event.data.payload as PlayerInputPayload;
          // Update ref directly for immediate access by game loop
          playerInputsRef.current = new Map(playerInputsRef.current).set(payload.player_id, payload);
          break;
        }
      }
    }
  }, []);

  const { sendEvent, isConnected, status, connect, disconnect } = useGameWebSocket<ServerEvent, HostEvent>({
    url: SOCKET_URL,
    onEvent: handleEvent,
    connect: false,
    parseEvent: (msg: MessageEvent<string>) => JSON.parse(msg.data) as ServerEvent,
  });

  const createSession = useCallback(() => {
    const event = createHostEvent({
      status: 'success',
      data: { type: 'createSession' },
    });
    sendEvent(event);
  }, [sendEvent]);

  const startGame = useCallback((gameType: string) => {
    const event = createHostEvent({
      status: 'success',
      data: { type: 'startGame', payload: { game_type: gameType } },
    });
    sendEvent(event);
  }, [sendEvent]);

  const getPlayerInputs = useCallback(() => {
    return playerInputsRef.current;
  }, []);

  return {
    sendEvent,
    isConnected,
    status,
    connect,
    disconnect,
    sessionCode,
    players,
    playerInputs,
    getPlayerInputs,
    createSession,
    startGame,
  };
};

export default useHostWebSocket;