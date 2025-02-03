use std::time::Duration;

use libp2p::{floodsub::Topic, identity, multiaddr::Protocol, Multiaddr, PeerId};
use serde::{Deserialize, Serialize};

use super::network::P2PNetwork;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub host_id: String,
    pub host_addr: String,
    topic: String,
    pub peers: Vec<PeerId>,
}

pub struct Sessions {
    pub local_key: identity::Keypair,
    pub peer_id: PeerId,
    pub current_session: Option<Session>,
    pub is_host: bool,
}

impl Session {
    pub fn new(host_id: String, host_addr: String, file_name: String) -> Self {
        let session_id = host_id.to_string();

        Self {
            id: session_id,
            host_id,
            host_addr,
            topic: file_name.clone(),
            peers: Vec::new(),
        }
    }

    pub fn generate_join_link(&self) -> String {
        format!("notepad-minus-minus://join/{}/{}", self.id, self.host_addr)
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

impl Default for Sessions {
    fn default() -> Self {
        Self::new()
    }
}

impl Sessions {
    pub fn new() -> Self {
        let local_key = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(local_key.public());

        if let Some(addr) = std::env::args().nth(1) {
            if let Ok(mut network) = P2PNetwork::new(false) {
                network
                    .swarm
                    .behaviour_mut()
                    .subscribe(Topic::new("notepad"));
                loop {
                    let mut multiaddr = addr.parse::<Multiaddr>().unwrap();
                    if let Protocol::P2p(_peer_id) = multiaddr.pop().expect("Valid peer Id") {
                        match network.swarm.dial(multiaddr.clone()) {
                            Ok(_) => {
                                print!("Dial of {} succesfull.", multiaddr);
                            }
                            Err(deal_error) => {
                                print!("Dial of {} was with error {}.", multiaddr, deal_error);
                            }
                        };

                        println!("In addrr loop. Info {:?}", network.swarm.network_info());
                        for peer in network.swarm.connected_peers() {
                            println!("Connected peer {peer}");
                        }
                        std::thread::sleep(Duration::new(1, 0));
                    };
                }
            };
        } else if let Ok(mut network) = P2PNetwork::new(true) {
            tokio::spawn(async move {
                network
                    .swarm
                    .behaviour_mut()
                    .subscribe(Topic::new("notepad"));
                let _ = network.run().await;
                network
                    .swarm
                    .behaviour_mut()
                    .publish_any(Topic::new("notepad"), "zeds mi kokot");
                std::thread::sleep(Duration::new(1, 0));
            });
        }

        Self {
            local_key,
            peer_id,
            current_session: None,
            is_host: false,
        }
    }

    pub fn create_session(&mut self, file_name: String) -> Session {
        let session = Session::new(
            self.peer_id.to_string(),
            format!("/ip4/0.0.0.0/tcp/0/p2p/{}", self.peer_id),
            file_name,
        );
        self.current_session = Some(session.clone());
        self.is_host = true;
        session
    }

    pub fn join_session(&mut self, connection_string: String) {
        let session: Session = serde_json::from_str(&connection_string).unwrap();
        self.current_session = Some(session);
        self.is_host = false;
    }

    pub fn join_session_from_addr(&mut self, _addr: String) {
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
