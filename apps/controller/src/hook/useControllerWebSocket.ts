import useGameWebSocket, { createEvent } from '@cactus-hampster/websocket'
import type { ServerToControllerEvent as ServerEvent, ControllerEvent } from '@cactus-hampster/typeshare'
import { useState } from 'react';

const SOCKET_URL = 'ws://localhost:8080/ping';

export const createControllerEvent = (event: ControllerEvent) => createEvent(event);

const useControllerWebSocket = () => {
  const [lastMessage, setLastMessage] = useState<ServerEvent | null>(null)
  const { sendEvent, isConnected, status, connect, disconnect } = useGameWebSocket<ServerEvent, ControllerEvent>({
    url: SOCKET_URL,
    onEvent: (event: ServerEvent) => setLastMessage(event),
    onError: (_) => disconnect(),
    connect: false,
    parseEvent: (msg: MessageEvent<string>) => JSON.parse(msg.data) as ServerEvent,
  })

  return {
    sendEvent,
    lastMessage,
    isConnected,
    status,
    connect,
    disconnect,
  }
}

export default useControllerWebSocket;