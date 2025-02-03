use libp2p::{
    floodsub::{Floodsub, FloodsubEvent, Topic},
    futures::StreamExt,
    identity, noise,
    swarm::{Swarm, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, SwarmBuilder,
};
use std::{collections::HashSet, error::Error};
use tracing::info;

pub struct P2PNetwork {
    pub swarm: Swarm<Floodsub>,
    discovered_peers: HashSet<PeerId>,
}

impl P2PNetwork {
    pub fn new(host: bool) -> Result<Self, Box<dyn Error>> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

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
        if host {
            swarm.listen_on("/ip4/0.0.0.0/tcp/5000".parse()?)?;
        } else {
            swarm.listen_on("/ip4/0.0.0.0/tcp/5666".parse()?)?;
        }

        // Create a channel for receiving messages
        // Do this when creating a session not entire network
        // let (message_sender, message_receiver) = mpsc::unbounded_channel();

        let network = Self {
            // local_peer_id,
            swarm,
            discovered_peers: HashSet::new(),
        };

        Ok(network)
    }

    pub async fn send_data(&mut self, data: String, topic: Topic) {
        self.swarm.behaviour_mut().publish_any(topic, data);
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            self.swarm
                .behaviour_mut()
                .publish(Topic::new("notepad"), "Chytim mikrofon");
            match self.swarm.select_next_some().await {
                SwarmEvent::Dialing {
                    peer_id,
                    connection_id: _,
                } => {
                    print!("Dialed by peer {:?}", peer_id);
                }
                SwarmEvent::IncomingConnection {
                    connection_id: _,
                    local_addr: _,
                    send_back_addr: _,
                } => {
                    println!("Incoming ")
                }
                SwarmEvent::ConnectionEstablished {
                    peer_id,
                    connection_id: _,
                    endpoint: _,
                    num_established: _,
                    concurrent_dial_errors: _,
                    established_in: _,
                } => {
                    println!("Connection came from {} ", peer_id);
                }
                SwarmEvent::Behaviour(FloodsubEvent::Message(message)) => {
                    println!("Received message: {:?}", message);
                }
                SwarmEvent::Behaviour(FloodsubEvent::Subscribed { peer_id, topic: _ }) => {
                    self.discovered_peers.insert(peer_id);
                    println!("Connected to peer: {peer_id}");
                }
                SwarmEvent::Behaviour(FloodsubEvent::Unsubscribed { peer_id, topic: _ }) => {
                    self.discovered_peers.remove(&peer_id);
                    println!("Disconnected from peer: {peer_id}");
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {address:?}");
                }
                e => {
                    println!("Received fuck all message: {:?}", e);
                }
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
