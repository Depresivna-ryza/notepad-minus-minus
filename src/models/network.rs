use libp2p::{
    Multiaddr,
    futures::StreamExt,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    identity, noise, swarm::{Swarm, SwarmEvent}, tcp, yamux, PeerId, SwarmBuilder
};
use tokio::sync::mpsc;
use tracing::info;
use std::{error::Error, collections::HashSet};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    SessionJoinRequest {
        session_id: String,
        peer_id: String,
    },
    SessionJoinResponse {
        accepted: bool,
        session_info: super::sessions::SessionInfo,
        file_content: Option<String>,
    },
    SessionContent {
        session_id: String,
        content: String,
        sender: String,
    },
    SessionLeave {
        session_id: String,
        peer_id: String,
    },
}


/// P2PNetwork handles libp2p networking logic.
pub struct P2PNetwork {
    local_peer_id: PeerId,
    topic: Topic,
    swarm: Swarm<Floodsub>,
    discovered_peers: HashSet<PeerId>,
}

impl P2PNetwork {
    pub async fn new() -> Result<(Self, mpsc::UnboundedReceiver<NetworkMessage>), Box<dyn Error>> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        let topic = Topic::new("notepad-minus-minus");

        info!("Local peer id: {local_peer_id}");

        // Create a floodsub instance
        let behaviour = Floodsub::new(local_peer_id);

        let mut swarm = SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_| behaviour)?
        .build();
        // Listen on all interfaces
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        
        // Create a channel for receiving messages
        let (message_sender, message_receiver) = mpsc::unbounded_channel();
        
        let network = Self {
            local_peer_id,
            topic,
            swarm,
            discovered_peers: HashSet::new(),
        };

        Ok((network, message_receiver))
    }

    pub async fn send_data(&mut self,data: String, topic: Topic){
        self.swarm.behaviour_mut().publish_any(topic, data);
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::Behaviour(FloodsubEvent::Message(message)) => {
                    println!("Received message: {:?}", message);
                }
                SwarmEvent::Behaviour(FloodsubEvent::Subscribed{peer_id, topic}) => {
                    self.discovered_peers.insert(peer_id);
                    println!("Connected to peer: {peer_id}");
                }
                SwarmEvent::Behaviour(FloodsubEvent::Unsubscribed{peer_id, topic}) => {
                    self.discovered_peers.remove(&peer_id);
                    println!("Disconnected from peer: {peer_id}");
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {address:?}");
                }
                _ => {}
            }
        }
    }

    pub fn connect_to_peer(&mut self, addr: &str) -> Result<(), Box<dyn Error>> {
        let remote: Multiaddr = addr.parse()?;
        self.swarm.dial(remote)?;
        println!("Dialing {addr}");
        Ok(())
    }
}
