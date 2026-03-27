import { useState, useEffect } from 'react'
import { usePingSocket } from './hook/usePingWebsocket'
import './App.css'

function App() {
  const [count, setCount] = useState(0)
  const { sendPing, lastMessage, isConnected, status, onConnect } = usePingSocket()

  useEffect(() => {
    if (lastMessage) {
      console.log("App.tsx received new message:", lastMessage);
    }
  }, [lastMessage]);

  useEffect(() => {
    if (!isConnected) {
      setCount(0)
    }
  }, [isConnected])

  return (
    <section id="center">
      <div>
        <h1>Get started</h1>
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
            onConnect()
          } else {
            setCount((count) => count + 1)
            sendPing()
          }
        }}
      >
        {`Press to ${isConnected ? `Ping server ${count}` : 'connect'}`}
      </button>
      {lastMessage ? <p>{lastMessage}</p> : null}
    </section>
  )
}

export default App
