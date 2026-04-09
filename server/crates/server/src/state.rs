use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

pub enum HostMessage {
    Event(String),
    Close,
}

pub enum ControllerMessage {
    Event(String),
    Close,
}

#[derive(Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub sender: mpsc::UnboundedSender<ControllerMessage>,
}

pub struct Session {
    pub code: String,
    pub host_sender: mpsc::UnboundedSender<HostMessage>,
    pub players: HashMap<String, Player>,
}

#[derive(Default)]
pub struct AppState {
    pub sessions: RwLock<HashMap<String, Session>>,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub async fn generate_session_code(&self) -> String {
        use rand::Rng;
        let sessions = self.sessions.read().await;
        loop {
            let code: String = (0..4)
                .map(|_| {
                    let idx = rand::thread_rng().gen_range(0..26);
                    (b'A' + idx) as char
                })
                .collect();
            if !sessions.contains_key(&code) {
                return code;
            }
        }
    }

    pub async fn create_session(
        &self,
        code: String,
        host_sender: mpsc::UnboundedSender<HostMessage>,
    ) {
        let session = Session {
            code: code.clone(),
            host_sender,
            players: HashMap::new(),
        };
        self.sessions.write().await.insert(code, session);
    }

    pub async fn remove_session(&self, code: &str) {
        if let Some(session) = self.sessions.write().await.remove(code) {
            for (_, player) in session.players {
                let _ = player.sender.send(ControllerMessage::Close);
            }
        }
    }

    pub async fn add_player_to_session(
        &self,
        session_code: &str,
        player_name: String,
        sender: mpsc::UnboundedSender<ControllerMessage>,
    ) -> Result<Player, String> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(session_code)
            .ok_or_else(|| "Session not found".to_string())?;

        let player_id = Uuid::new_v4().to_string();
        let player = Player {
            id: player_id.clone(),
            name: player_name,
            sender,
        };
        session.players.insert(player_id, player.clone());
        Ok(player)
    }

    pub async fn remove_player_from_session(
        &self,
        session_code: &str,
        player_id: &str,
    ) -> Option<Player> {
        let mut sessions = self.sessions.write().await;
        sessions
            .get_mut(session_code)
            .and_then(|s| s.players.remove(player_id))
    }

    pub async fn get_host_sender(
        &self,
        session_code: &str,
    ) -> Option<mpsc::UnboundedSender<HostMessage>> {
        self.sessions
            .read()
            .await
            .get(session_code)
            .map(|s| s.host_sender.clone())
    }

    pub async fn broadcast_to_controllers(&self, session_code: &str, message: &str) {
        if let Some(session) = self.sessions.read().await.get(session_code) {
            for (_, player) in &session.players {
                let _ = player
                    .sender
                    .send(ControllerMessage::Event(message.to_string()));
            }
        }
    }
}
