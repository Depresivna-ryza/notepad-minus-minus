use libp2p::{
    ping,
    Multiaddr,
    futures::StreamExt,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    identity, noise, swarm::{Swarm, SwarmEvent, NetworkBehaviour, IntoConnectionHandlerEvent}, tcp, yamux, PeerId, SwarmBuilder, ConnectionId, ConnectedPoint, ConnectionHandler, ConnectionHandlerEvent, NetworkBehaviourAction, PollParameters, Context
};
use tokio::sync::mpsc;
use tracing::info;
use std::{error::Error, collections::{HashSet, VecDeque}};
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

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "NotepadBehaviourEvent")]
pub struct NotepadBehaviour {
    floodsub: Floodsub,
    events: VecDeque<NotepadBehaviourEvent>,
}

#[derive(Debug)]
pub enum NotepadBehaviourEvent {
    Message(NetworkMessage),
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
}

impl NotepadBehaviour {
    pub fn new(local_peer_id: PeerId) -> Self {
        let mut floodsub = Floodsub::new(local_peer_id);
        floodsub.subscribe(Topic::new("notepad-minus-minus"));
        
        Self {
            floodsub,
            events: VecDeque::new(),
        }
    }

    pub fn publish(&mut self, message: NetworkMessage) -> Result<(), Box<dyn Error>> {
        let message_bytes = serde_json::to_vec(&message)?;
        self.floodsub.publish(Topic::new("notepad-minus-minus"), message_bytes);
        Ok(())
    }
}

impl NetworkBehaviour for NotepadBehaviour {
    type ConnectionHandler = <Floodsub as NetworkBehaviour>::ConnectionHandler;
    type OutEvent = NotepadBehaviourEvent;

    fn poll(
        &mut self,
        cx: &mut Context<'_>,
        params: &mut impl PollParameters,
    ) -> Poll<NetworkBehaviourAction<Self::OutEvent, Self::ConnectionHandler>> {
        // Poll the floodsub behaviour
        while let Poll::Ready(Some(event)) = self.floodsub.poll(cx, params) {
            if let NetworkBehaviourAction::GenerateEvent(FloodsubEvent::Message(message)) = event {
                if let Ok(msg) = serde_json::from_slice::<NetworkMessage>(&message.data) {
                    self.events.push_back(NotepadBehaviourEvent::Message(msg));
                }
            }
        }

        // Return the next event if we have one
        if let Some(event) = self.events.pop_front() {
            return Poll::Ready(NetworkBehaviourAction::GenerateEvent(event));
        }

        Poll::Pending
    }
}

/// P2PNetwork handles libp2p networking logic.
pub struct P2PNetwork {
    local_peer_id: PeerId,
    topic: Topic,
    swarm: Swarm<NotepadBehaviour>,
    discovered_peers: HashSet<PeerId>,
}

impl P2PNetwork {
    pub async fn new() -> Result<(Self, mpsc::UnboundedReceiver<NetworkMessage>), Box<dyn Error>> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        let topic = Topic::new("notepad-minus-minus");

        info!("Local peer id: {local_peer_id}");

        // Create a floodsub instance
        let behaviour = NotepadBehaviour::new(local_peer_id);

        let mut swarm = SwarmBuilder::with_new_identity()
            .with_tokio()
            //.with_websocket(noise::Config::new(&local_key), yamux::Config::default)
            .with_tcp(tcp::Config::default(), noise::Config::new,yamux::Config::default)
            .with_behaviour(behaviour)
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

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::Behaviour(NotepadBehaviourEvent::Message(message)) => {
                    println!("Received message: {:?}", message);
                }
                SwarmEvent::Behaviour(NotepadBehaviourEvent::PeerConnected(peer_id)) => {
                    self.discovered_peers.insert(peer_id);
                    println!("Connected to peer: {peer_id}");
                }
                SwarmEvent::Behaviour(NotepadBehaviourEvent::PeerDisconnected(peer_id)) => {
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

    pub fn request_join_session(&mut self, session_id: String) -> Result<(), Box<dyn Error>> {
        let message = NetworkMessage::SessionJoinRequest {
            session_id,
            peer_id: self.local_peer_id.to_string(),
        };

        self.swarm.behaviour_mut().publish(message);
        Ok(())
    }

    pub fn accept_join_request(
        &mut self,
        session_info: super::sessions::SessionInfo,
        file_content: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        let message = NetworkMessage::SessionJoinResponse {
            accepted: true,
            session_info,
            file_content,
        };

        self.swarm.behaviour_mut().publish(message);
        Ok(())
    }

    pub fn send_session_content(
        &mut self,
        session_id: String,
        content: String,
    ) -> Result<(), Box<dyn Error>> {
        let message = NetworkMessage::SessionContent {
            session_id,
            content,
            sender: self.local_peer_id.to_string(),
        };

        self.swarm.behaviour_mut().publish(message);
        Ok(())
    }

    pub fn leave_session(&mut self, session_id: String) -> Result<(), Box<dyn Error>> {
        let message = NetworkMessage::SessionLeave {
            session_id,
            peer_id: self.local_peer_id.to_string(),
        };

        self.swarm.behaviour_mut().publish(message);
        Ok(())
    }

    pub fn connect_to_peer(&mut self, addr: &str) -> Result<(), Box<dyn Error>> {
        let remote: Multiaddr = addr.parse()?;
        self.swarm.dial(remote)?;
        println!("Dialing {addr}");
        Ok(())
    }
}
