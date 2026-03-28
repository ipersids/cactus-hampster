import useWebSocket, { ReadyState } from 'react-use-websocket';
import { useState, useCallback, useRef, useEffect } from 'react';

export interface GameSocketProps<TServerEvent> {
  url: string;
  onEvent?: (event: TServerEvent) => void;
  onError?: (error: Error) => void;
  connect?: boolean;
  parseEvent?: (msg: MessageEvent<string>) => TServerEvent;
}

const statusMap = {
  [ReadyState.CONNECTING]: "Connecting",
  [ReadyState.OPEN]: "Open",
  [ReadyState.CLOSING]: "Closing",
  [ReadyState.CLOSED]: "Closed",
  [ReadyState.UNINSTANTIATED]: "Uninstantiated",
};

// @INFO:
// - onError (physical connection failed (e.g., DNS error, Server down))
// - onClose (common and custom reasons to close connection)
// - onMessage (parse/handle event/handle errors, connection stays alive)

// @TODO:
//  - better error handling
//  - binary support
//  - tracing logs?
//  - unit/integration test
//  - heartbeat will keep connection alive until tab is open, so, we should
//    kill connection after a reasonable inactive period?
const useGameWebSocket = <TServerEvent = unknown, TClientEvent = unknown>({
  url,
  onEvent,
  onError,
  connect = false,
  parseEvent,
}: GameSocketProps<TServerEvent>) => {
  const [shouldConnect, setShouldConnect] = useState(connect);
  const onEventRef = useRef(onEvent);
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onEventRef.current = onEvent;
    onErrorRef.current = onError;
  }, [onEvent, onError]);

  const defaultParse = useCallback(
    (msg: MessageEvent<string>) =>
      JSON.parse(msg.data) as TServerEvent,
    []
  );

  const parser = parseEvent ?? defaultParse;

  const { sendMessage, readyState, getWebSocket } = useWebSocket(
    url,
    {
      shouldReconnect: () => shouldConnect,
      reconnectAttempts: 15,
      reconnectInterval: 2000,

      onMessage: (msg) => {
        if (msg.data === 'pong') return;
        try {
          const data = parser(msg);
          onEventRef.current?.(data);
        } catch (e) {
          console.error("Malformed message from server", e);
          onErrorRef.current?.(
            new Error("Invalid message format")
          );
        }
      },

      onError: (event) => {
        onErrorRef.current?.(
          new Error(`WebSocket error: ${event.type}`)
        );
      },

      heartbeat: {
        message: 'ping',
        returnMessage: 'pong',
        interval: 30_000,
      },
    },
    shouldConnect
  );

  const sendEvent = useCallback(
    (command: TClientEvent) => {
      if (readyState === ReadyState.OPEN) {
        sendMessage(JSON.stringify(command));
      } else {
        console.warn(
          "Tried to send event while socket was:",
          statusMap[readyState]
        );
      }
    },
    [sendMessage, readyState]
  );

  const connectSocket = useCallback(() => {
    setShouldConnect(true);
  }, []);

  const disconnectSocket = useCallback(() => {
    setShouldConnect(false);
    getWebSocket()?.close();
  }, [getWebSocket]);

  return {
    sendEvent,
    isConnected: readyState === ReadyState.OPEN,
    status: statusMap[readyState],
    connect: connectSocket,
    disconnect: disconnectSocket,
  };
};

export default useGameWebSocket;