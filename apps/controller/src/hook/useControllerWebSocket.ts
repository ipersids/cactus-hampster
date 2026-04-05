import useGameWebSocket, { createEvent } from '@cactus-hampster/websocket';
import type {
  ServerToControllerEvent as ServerEvent,
  ControllerEvent,
  ControllerInputPayload,
} from '@cactus-hampster/typeshare';
import { useState, useCallback } from 'react';

const SOCKET_URL = '/ws/controller';

export const createControllerEvent = (event: ControllerEvent) => createEvent(event);

export type ControllerPhase = 'disconnected' | 'joining' | 'waiting' | 'playing' | 'error';

const useControllerWebSocket = () => {
  const [phase, setPhase] = useState<ControllerPhase>('disconnected');
  const [playerId, setPlayerId] = useState<string | null>(null);
  const [sessionCode, setSessionCode] = useState<string | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [gameType, setGameType] = useState<string | null>(null);

  const handleEvent = useCallback((event: ServerEvent) => {
    if (event.status === 'success') {
      switch (event.data.type) {
        case 'joinSuccess':
          setPlayerId(event.data.payload.player_id);
          setSessionCode(event.data.payload.session_code);
          setPhase('waiting');
          break;
        case 'gameStarted':
          setGameType(event.data.payload.game_type);
          setPhase('playing');
          break;
      }
    } else if (event.status === 'error') {
      setErrorMessage(event.data.message);
      setPhase('error');
    }
  }, []);

  const { sendEvent, isConnected, status, connect, disconnect } = useGameWebSocket<ServerEvent, ControllerEvent>({
    url: SOCKET_URL,
    onEvent: handleEvent,
    connect: false,
    parseEvent: (msg: MessageEvent<string>) => JSON.parse(msg.data) as ServerEvent,
  });

  const joinSession = useCallback((code: string, nickname: string) => {
    setPhase('joining');
    const event = createControllerEvent({
      status: 'success',
      data: {
        type: 'joinSession',
        payload: { session_code: code, nickname },
      },
    });
    sendEvent(event);
  }, [sendEvent]);

  const sendInput = useCallback((input: ControllerInputPayload) => {
    const event = createControllerEvent({
      status: 'success',
      data: {
        type: 'playerInput',
        payload: input,
      },
    });
    sendEvent(event);
  }, [sendEvent]);

  return {
    sendEvent,
    isConnected,
    status,
    connect,
    disconnect,
    phase,
    playerId,
    sessionCode,
    errorMessage,
    gameType,
    joinSession,
    sendInput,
  };
};

export default useControllerWebSocket;