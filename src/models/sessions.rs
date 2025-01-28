use super::files::DirectoryItem;
use libp2p::{identity, Multiaddr, PeerId};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    pub host_id: String,
    pub host_addr: String,
    pub name: String,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub info: SessionInfo,
    pub peers: Vec<PeerId>,
    pub shared_file_name: String,
}

#[derive(Debug)]
pub struct Sessions {
    pub local_key: identity::Keypair,
    pub peer_id: PeerId,
    pub current_session: Option<Session>,
    pub is_host: bool,
}

impl Session {
    pub fn new(host_id: String, host_addr: String, name: String, file_name: String) -> Self {
        let session_id = format!(
            "{}_{}",
            host_id,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        Self {
            info: SessionInfo {
                id: session_id,
                host_id,
                host_addr,
                name,
                created_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            peers: Vec::new(),
            shared_file_name: file_name,
        }
    }

    pub fn generate_join_link(&self) -> String {
        format!(
            "notepad-minus-minus://join/{}/{}",
            self.info.id, self.info.host_addr
        )
    }

    pub fn add_peer(&mut self, peer_id: PeerId) {
        if !self.peers.contains(&peer_id) {
            self.peers.push(peer_id);
        }
    }

    pub fn remove_peer(&mut self, peer_id: &PeerId) {
        self.peers.retain(|p| p != peer_id);
    }
}

impl Sessions {
    pub fn new() -> Self {
        let local_key = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(local_key.public());

        Self {
            local_key,
            peer_id,
            current_session: None,
            is_host: false,
        }
    }

    pub fn create_session(&mut self, name: String, file_name: String) -> Session {
        let session = Session::new(
            self.peer_id.to_string(),
            format!("/ip4/0.0.0.0/tcp/0/p2p/{}", self.peer_id),
            name,
            file_name,
        );
        self.current_session = Some(session.clone());
        self.is_host = true;
        session
    }

    pub fn join_session(&mut self, session_info: SessionInfo) {
        let session = Session {
            info: session_info,
            peers: Vec::new(),
            shared_file_name: String::new(), // Will be updated when receiving file from host
        };
        self.current_session = Some(session);
        self.is_host = false;
    }

    pub fn leave_session(&mut self) {
        self.current_session = None;
        self.is_host = false;
    }

    pub fn parse_join_link(link: &str) -> Option<(String, String)> {
        let parts: Vec<&str> = link.split("/").collect();
        if parts.len() >= 2 && parts[0] == "notepad-minus-minus:" {
            Some((
                parts[parts.len() - 2].to_string(),
                parts[parts.len() - 1].to_string(),
            ))
        } else {
            None
        }
    }
}
