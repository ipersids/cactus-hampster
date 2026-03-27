import useWebSocket, { ReadyState } from 'react-use-websocket'
import { useState } from 'react'
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const useSocket = (useWebSocket as any).default as typeof useWebSocket

const SOCKET_URL = 'ws://localhost:8080/ping';

export const usePingSocket = () => {
  const [connect, setConnect] = useState<boolean>(false);
  const { sendMessage, lastMessage, readyState, getWebSocket } = useSocket(SOCKET_URL, {
    onOpen: () => console.log("Connected to WebSocket: ", getWebSocket()?.url),
    onClose: () => {
      setConnect(false)
      console.log("Disconnected", getWebSocket()?.url)
    },
    onMessage: (event) => console.log('Received:', event.data),
    heartbeat: {
      message: "ping",
      interval: 30_000,
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

  return {
    sendPing: () => sendMessage('Ping'),
    lastMessage: lastMessage ? lastMessage.data : null,
    isConnected: readyState === ReadyState.OPEN,
    status: connectionStatus,
    onConnect,
  };
};