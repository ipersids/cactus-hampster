import useGameWebSocket, { createEvent } from '@cactus-hampster/websocket';
import type { ServerToHostEvent as ServerEvent, HostEvent } from '@cactus-hampster/typeshare';
import { useState } from 'react';

const SOCKET_URL = '/ping';

export const createHostEvent = (event: HostEvent) => createEvent(event);

const useHostWebSocket = () => {
  const [lastMessage, setLastMessage] = useState<ServerEvent | null>(null);
  const { sendEvent, isConnected, status, connect, disconnect } = useGameWebSocket<ServerEvent, HostEvent>({
    url: SOCKET_URL,
    onEvent: (event: ServerEvent) => setLastMessage(event),
    connect: false,
    parseEvent: (msg: MessageEvent<string>) => JSON.parse(msg.data) as ServerEvent,
  });

  return {
    sendEvent,
    lastMessage,
    isConnected,
    status,
    connect,
    disconnect,
  };
};

export default useHostWebSocket;