import { useState, useEffect } from 'react';
import useHostWebSocket, { createHostEvent } from './hook/useHostWebSocket';
import './App.css';

function App() {
  const [count, setCount] = useState(0);
  const [message, setMessage] = useState('');

  const { sendEvent, lastMessage, isConnected, status, connect } = useHostWebSocket();

  useEffect(() => {
    if (!isConnected) {
      setCount(0);
    }
  }, [isConnected]);

  useEffect(() => {
    if (lastMessage) {
      const dateString = new Date().toLocaleString();
      const data = JSON.stringify(lastMessage);
      setMessage(`${dateString} - ${data}`);
    }
  }, [lastMessage]);

  const ping = createHostEvent({
    status: "success",
    data: {
      type: 'ping',
      payload: {
        message: "Hello from Host!",
      }
    },
  });

  return (
    <section id="center">
      <div>
        <h1>I'm Host</h1>
      </div>

      <div>
        <p>
          {`Websocket status: ${isConnected ? "connected" : "disconnected"} (${status})`}
        </p>
      </div>
      <button
        className="counter"
        onClick={() => {
          if (!isConnected) {
            connect();
          } else {
            setCount((count) => count + 1);
            sendEvent(ping);
          }
        }}
      >
        {`Press to ${isConnected ? `Ping server ${count}` : 'connect'}`}
      </button>
      {message ? <p>{message}</p> : null}
    </section>
  );
}

export default App;
