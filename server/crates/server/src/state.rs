use axum::extract::ws::Message;
use rand::Rng;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

pub type ConnectionId = u64;
pub type Tx = mpsc::UnboundedSender<Message>;

#[derive(Clone)]
pub struct Player {
    pub id: String,
    pub nickname: String,
    pub conn_id: ConnectionId,
}

pub struct Session {
    pub code: String,
    pub host_conn: ConnectionId,
    pub players: HashMap<String, Player>, // player_id -> Player
    pub game_started: bool,
}

pub struct AppState {
    pub sessions: RwLock<HashMap<String, Session>>, // code -> session
    pub connections: RwLock<HashMap<ConnectionId, Tx>>,
    pub conn_to_session: RwLock<HashMap<ConnectionId, String>>, // conn_id -> session_code
    pub conn_to_player: RwLock<HashMap<ConnectionId, String>>,  // conn_id -> player_id
    next_connection_id: AtomicU64,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            sessions: RwLock::new(HashMap::new()),
            connections: RwLock::new(HashMap::new()),
            conn_to_session: RwLock::new(HashMap::new()),
            conn_to_player: RwLock::new(HashMap::new()),
            next_connection_id: AtomicU64::new(1),
        })
    }

    pub fn next_conn_id(&self) -> ConnectionId {
        self.next_connection_id.fetch_add(1, Ordering::SeqCst)
    }

    pub async fn register_connection(&self, conn_id: ConnectionId, tx: Tx) {
        self.connections.write().await.insert(conn_id, tx);
    }

    pub async fn unregister_connection(&self, conn_id: ConnectionId) {
        self.connections.write().await.remove(&conn_id);
    }

    pub async fn create_session(&self, host_conn: ConnectionId) -> String {
        let code = self.generate_unique_code().await;
        let session = Session {
            code: code.clone(),
            host_conn,
            players: HashMap::new(),
            game_started: false,
        };
        self.sessions.write().await.insert(code.clone(), session);
        self.conn_to_session
            .write()
            .await
            .insert(host_conn, code.clone());
        code
    }

    async fn generate_unique_code(&self) -> String {
        let sessions = self.sessions.read().await;
        loop {
            let code: String = (0..4)
                .map(|_| rand::rng().random_range(0..10).to_string())
                .collect();
            if !sessions.contains_key(&code) {
                return code;
            }
        }
    }

    pub async fn join_session(
        &self,
        code: &str,
        nickname: &str,
        conn_id: ConnectionId,
    ) -> Result<(String, Vec<Player>), String> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(code)
            .ok_or_else(|| "Session not found".to_string())?;

        if session.game_started {
            return Err("Game already started".to_string());
        }

        let player_id = uuid::Uuid::new_v4().to_string();
        let player = Player {
            id: player_id.clone(),
            nickname: nickname.to_string(),
            conn_id,
        };

        session.players.insert(player_id.clone(), player);
        let players: Vec<Player> = session.players.values().cloned().collect();

        drop(sessions);

        self.conn_to_session
            .write()
            .await
            .insert(conn_id, code.to_string());
        self.conn_to_player
            .write()
            .await
            .insert(conn_id, player_id.clone());

        Ok((player_id, players))
    }

    pub async fn get_host_conn(&self, session_code: &str) -> Option<ConnectionId> {
        self.sessions
            .read()
            .await
            .get(session_code)
            .map(|s| s.host_conn)
    }

    pub async fn get_session_for_conn(&self, conn_id: ConnectionId) -> Option<String> {
        self.conn_to_session.read().await.get(&conn_id).cloned()
    }

    pub async fn get_player_id_for_conn(&self, conn_id: ConnectionId) -> Option<String> {
        self.conn_to_player.read().await.get(&conn_id).cloned()
    }

    pub async fn send_to(&self, conn_id: ConnectionId, msg: Message) {
        if let Some(tx) = self.connections.read().await.get(&conn_id) {
            let _ = tx.send(msg);
        }
    }

    pub async fn broadcast_to_players(&self, session_code: &str, msg: Message) {
        let sessions = self.sessions.read().await;
        if let Some(session) = sessions.get(session_code) {
            let connections = self.connections.read().await;
            for player in session.players.values() {
                if let Some(tx) = connections.get(&player.conn_id) {
                    let _ = tx.send(msg.clone());
                }
            }
        }
    }

    pub async fn start_game(&self, session_code: &str) -> bool {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_code) {
            session.game_started = true;
            true
        } else {
            false
        }
    }

    pub async fn handle_disconnect(&self, conn_id: ConnectionId) -> Option<(String, String)> {
        // Remove from connections
        self.connections.write().await.remove(&conn_id);

        // Check if this was a player
        let player_id = self.conn_to_player.write().await.remove(&conn_id);
        let session_code = self.conn_to_session.write().await.remove(&conn_id);

        if let (Some(player_id), Some(session_code)) = (player_id, session_code.clone()) {
            // Remove player from session
            let mut sessions = self.sessions.write().await;
            if let Some(session) = sessions.get_mut(&session_code) {
                session.players.remove(&player_id);
            }
            return Some((session_code, player_id));
        }

        // Check if this was a host - if so, remove the whole session
        if let Some(code) = session_code {
            let mut sessions = self.sessions.write().await;
            if let Some(session) = sessions.get(&code) {
                if session.host_conn == conn_id {
                    sessions.remove(&code);
                }
            }
        }

        None
    }
}
