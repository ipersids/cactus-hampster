import useWebSocket, { ReadyState } from 'react-use-websocket'
import { useState } from 'react'
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const useSocket = (useWebSocket as any).default as typeof useWebSocket

import type { HostEvent } from '@cactus-hampster/typeshare'

const SOCKET_URL = 'ws://localhost:8080/ping';

function createHostEvent(event: HostEvent): HostEvent {
  return event;
}

export const usePingSocket = () => {
  const [connect, setConnect] = useState<boolean>(false);
  const { sendMessage, lastMessage, readyState, getWebSocket } = useSocket(SOCKET_URL, {
    // @TODO: handle
    // - onError (physical connection failed (e.g., DNS error, Server down))
    // - onClose (common and custom reasons to close connection)
    // - onMessage (game backeng logic errors, connection stays alive)
    // - silent heartbeat errors and reconnection
    onOpen: () => console.log("Connected to WebSocket: ", getWebSocket()?.url),
    onClose: () => {
      setConnect(false)
      console.log("Disconnected", getWebSocket()?.url)
    },
    onMessage: (event) => console.log('Received:', event.data),
    heartbeat: {
      message: "ping",
      interval: 30_000,
      returnMessage: "pong"
    }
  }, connect);

  const onConnect = () => {
    setConnect(true)
  }

  const connectionStatus = {
    [ReadyState.CONNECTING]: 'Connecting',
    [ReadyState.OPEN]: 'Open',
    [ReadyState.CLOSING]: 'Closing',
    [ReadyState.CLOSED]: 'Closed',
    [ReadyState.UNINSTANTIATED]: 'Uninstantiated',
  }[readyState];

  const ping: HostEvent = createHostEvent({
    status: "success",
    data: {
      type: 'ping',
      payload: {
        message: "Hello from Host!"
      }
    }
  })

  return {
    sendPing: () => sendMessage(JSON.stringify(ping)),
    lastMessage: lastMessage ? lastMessage.data : null,
    isConnected: readyState === ReadyState.OPEN,
    status: connectionStatus,
    onConnect,
  };
};