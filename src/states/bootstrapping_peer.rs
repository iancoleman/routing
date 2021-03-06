// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use super::{common::Base, joining_peer::JoiningPeerDetails};
use crate::{
    chain::{EldersInfo, NetworkParams},
    error::{Result, RoutingError},
    event::Event,
    id::FullId,
    location::{DstLocation, SrcLocation},
    messages::{BootstrapResponse, Message, MessageWithBytes, Variant, VerifyStatus},
    network_service::NetworkService,
    outbox::EventBox,
    peer_map::PeerMap,
    relocation::{RelocatePayload, SignedRelocateDetails},
    rng::MainRng,
    state_machine::{State, Transition},
    states::JoiningPeer,
    timer::Timer,
    xor_space::{Prefix, XorName},
    ConnectionInfo,
};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display, Formatter},
    iter,
    net::SocketAddr,
    time::Duration,
};

/// Time after which bootstrap is cancelled (and possibly retried).
pub const BOOTSTRAP_TIMEOUT: Duration = Duration::from_secs(20);

pub struct BootstrappingPeerDetails {
    pub network_service: NetworkService,
    pub full_id: FullId,
    pub network_cfg: NetworkParams,
    pub timer: Timer,
    pub rng: MainRng,
}

// State of Client or Node while bootstrapping.
pub struct BootstrappingPeer {
    pending_requests: HashSet<SocketAddr>,
    timeout_tokens: HashMap<u64, SocketAddr>,
    network_service: NetworkService,
    full_id: FullId,
    timer: Timer,
    rng: MainRng,
    relocate_details: Option<SignedRelocateDetails>,
    network_cfg: NetworkParams,
}

impl BootstrappingPeer {
    pub fn new(mut details: BootstrappingPeerDetails) -> Self {
        details.network_service.service_mut().bootstrap();
        Self {
            network_service: details.network_service,
            full_id: details.full_id,
            timer: details.timer,
            pending_requests: Default::default(),
            timeout_tokens: Default::default(),
            rng: details.rng,
            relocate_details: None,
            network_cfg: details.network_cfg,
        }
    }

    /// Create `BootstrappingPeer` for a node that is being relocated into another sections.
    pub fn relocate(
        details: BootstrappingPeerDetails,
        conn_infos: Vec<ConnectionInfo>,
        relocate_details: SignedRelocateDetails,
    ) -> Self {
        let mut node = Self {
            network_service: details.network_service,
            full_id: details.full_id,
            timer: details.timer,
            pending_requests: Default::default(),
            timeout_tokens: Default::default(),
            rng: details.rng,
            relocate_details: Some(relocate_details),
            network_cfg: details.network_cfg,
        };

        for conn_info in conn_infos {
            node.send_bootstrap_request(conn_info)
        }

        node
    }

    pub fn into_joining(
        self,
        elders_info: EldersInfo,
        relocate_payload: Option<RelocatePayload>,
        _outbox: &mut dyn EventBox,
    ) -> Result<State> {
        let details = JoiningPeerDetails {
            network_service: self.network_service,
            full_id: self.full_id,
            network_cfg: self.network_cfg,
            timer: self.timer,
            rng: self.rng,
            elders_info,
            relocate_payload,
        };

        Ok(State::JoiningPeer(JoiningPeer::new(details)))
    }

    fn send_bootstrap_request(&mut self, dst: ConnectionInfo) {
        if !self.pending_requests.insert(dst.peer_addr) {
            return;
        }

        debug!("{} Sending BootstrapRequest to {}.", self, dst.peer_addr);

        let token = self.timer.schedule(BOOTSTRAP_TIMEOUT);
        let _ = self.timeout_tokens.insert(token, dst.peer_addr);

        let destination = self.get_destination();

        self.send_direct_message(&dst, Variant::BootstrapRequest(destination));
        self.peer_map_mut().connect(dst);
    }

    // If we are relocating, request bootstrap to the section matching the name given to us
    // by our section. Otherwise request bootstrap to the section matching our current name.
    fn get_destination(&self) -> XorName {
        if let Some(details) = self
            .relocate_details
            .as_ref()
            .map(|msg| msg.relocate_details())
        {
            details.destination
        } else {
            *self.name()
        }
    }

    fn join_section(&mut self, info: EldersInfo) -> Result<Transition, RoutingError> {
        let old_full_id = self.full_id.clone();
        let destination = self.get_destination();

        // Use a name that will match the destination even after multiple splits
        let extra_split_count = 3;
        let name_prefix = Prefix::new(info.prefix().bit_count() + extra_split_count, destination);

        if !name_prefix.matches(self.name()) {
            let new_full_id = FullId::within_range(&mut self.rng, &name_prefix.range_inclusive());
            info!(
                "{} - Changing name to {}.",
                self,
                new_full_id.public_id().name()
            );
            self.full_id = new_full_id;
        }

        let relocate_payload = if let Some(details) = self.relocate_details.take() {
            Some(RelocatePayload::new(
                details,
                self.full_id.public_id(),
                &old_full_id,
            )?)
        } else {
            None
        };

        Ok(Transition::IntoJoining {
            info,
            relocate_payload,
        })
    }

    fn reconnect_to_new_section(&mut self, new_conn_infos: Vec<ConnectionInfo>) {
        self.network_service_mut().remove_and_disconnect_all();

        self.pending_requests.clear();
        self.timeout_tokens.clear();

        for conn_info in new_conn_infos {
            self.send_bootstrap_request(conn_info);
        }
    }

    fn request_failed(&mut self) {
        if self.pending_requests.is_empty() {
            self.network_service.service_mut().bootstrap();
        }
    }
}

impl Base for BootstrappingPeer {
    fn network_service(&self) -> &NetworkService {
        &self.network_service
    }

    fn network_service_mut(&mut self) -> &mut NetworkService {
        &mut self.network_service
    }

    fn full_id(&self) -> &FullId {
        &self.full_id
    }

    fn in_dst_location(&self, dst: &DstLocation) -> bool {
        match dst {
            DstLocation::Direct => true,
            _ => false,
        }
    }

    fn peer_map(&self) -> &PeerMap {
        &self.network_service().peer_map
    }

    fn peer_map_mut(&mut self) -> &mut PeerMap {
        &mut self.network_service_mut().peer_map
    }

    fn timer(&mut self) -> &mut Timer {
        &mut self.timer
    }

    fn rng(&mut self) -> &mut MainRng {
        &mut self.rng
    }

    fn handle_send_message(
        &mut self,
        _: SrcLocation,
        _: DstLocation,
        _: Vec<u8>,
    ) -> Result<(), RoutingError> {
        warn!("{} - Cannot handle SendMessage - not bootstrapped.", self);
        // TODO: return Err here eventually. Returning Ok for now to
        // preserve the pre-refactor behaviour.
        Ok(())
    }

    fn handle_timeout(&mut self, token: u64, _: &mut dyn EventBox) -> Transition {
        if let Some(peer_addr) = self.timeout_tokens.remove(&token) {
            debug!(
                "{} - Timeout when trying to bootstrap against {}.",
                self, peer_addr
            );

            if !self.pending_requests.remove(&peer_addr) {
                return Transition::Stay;
            }

            self.disconnect(&peer_addr);
            self.request_failed()
        }

        Transition::Stay
    }

    fn handle_bootstrapped_to(&mut self, conn_info: ConnectionInfo) -> Transition {
        self.send_bootstrap_request(conn_info);
        Transition::Stay
    }

    fn handle_bootstrap_failure(&mut self, outbox: &mut dyn EventBox) -> Transition {
        info!("{} Failed to bootstrap. Terminating.", self);
        outbox.send_event(Event::Terminated);
        Transition::Terminate
    }

    fn handle_connection_failure(
        &mut self,
        peer_addr: SocketAddr,
        _: &mut dyn EventBox,
    ) -> Transition {
        let _ = self.pending_requests.remove(&peer_addr);
        let _ = self.peer_map_mut().disconnect(peer_addr);
        self.request_failed();
        Transition::Stay
    }

    fn handle_message(
        &mut self,
        sender: Option<ConnectionInfo>,
        msg: Message,
        _: &mut dyn EventBox,
    ) -> Result<Transition> {
        let p2p_node = msg.src.to_sender_node(sender)?;

        // Ignore messages from peers we didn't send `BootstrapRequest` to.
        if !self.pending_requests.contains(p2p_node.peer_addr()) {
            debug!(
                "{} - Ignoring message from unexpected peer: {}: {:?}",
                self, p2p_node, msg,
            );
            self.disconnect(p2p_node.peer_addr());
            return Ok(Transition::Stay);
        }

        match msg.variant {
            Variant::BootstrapResponse(BootstrapResponse::Join(info)) => {
                info!(
                    "{} - Joining a section {:?} (given by {:?})",
                    self, info, p2p_node
                );
                self.join_section(info)
            }
            Variant::BootstrapResponse(BootstrapResponse::Rebootstrap(new_conn_infos)) => {
                info!(
                    "{} - Bootstrapping redirected to another set of peers: {:?}",
                    self, new_conn_infos
                );
                self.reconnect_to_new_section(new_conn_infos);
                Ok(Transition::Stay)
            }
            _ => unreachable!(),
        }
    }

    fn unhandled_message(&mut self, _sender: Option<ConnectionInfo>, msg: Message) {
        debug!("{} - Unhandled message {:?}", self, msg);
    }

    fn filter_incoming_message(&mut self, _message: &MessageWithBytes) -> bool {
        true
    }

    fn should_handle_message(&self, msg: &Message) -> bool {
        match msg.variant {
            Variant::BootstrapResponse(_) => true,
            Variant::NeighbourInfo(_)
            | Variant::UserMessage(_)
            | Variant::NodeApproval(_)
            | Variant::AckMessage { .. }
            | Variant::GenesisUpdate(_)
            | Variant::Relocate(_)
            | Variant::MessageSignature(_)
            | Variant::BootstrapRequest(_)
            | Variant::JoinRequest(_)
            | Variant::ConnectionResponse
            | Variant::MemberKnowledge { .. }
            | Variant::ParsecRequest(..)
            | Variant::ParsecResponse(..) => false,
        }
    }

    fn verify_message(&self, msg: &Message) -> Result<bool> {
        msg.verify(iter::empty())
            .and_then(VerifyStatus::require_full)?;
        Ok(true)
    }

    fn relay_message(&mut self, _message: &MessageWithBytes) -> Result<()> {
        Ok(())
    }
}

impl Display for BootstrappingPeer {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "BootstrappingPeer({})", self.name())
    }
}

#[cfg(all(test, feature = "mock_base"))]
mod tests {
    use super::*;
    use crate::{
        chain::NetworkParams,
        id::FullId,
        messages::Message,
        mock::Environment,
        quic_p2p::{Builder, Peer},
        state_machine::StateMachine,
        unwrap, NetworkConfig, NetworkEvent,
    };
    use crossbeam_channel as mpmc;

    #[test]
    // Check that losing our proxy connection while in the `BootstrappingPeer` state doesn't stall
    // and instead triggers a re-bootstrap attempt..
    fn lose_proxy_connection() {
        let mut network_cfg = NetworkParams::default();

        if cfg!(feature = "mock_base") {
            network_cfg.elder_size = 7;
            network_cfg.safe_section_size = 30;
        };

        let env = Environment::new(Default::default());
        let mut rng = env.new_rng();

        // Start a bare-bones network service.
        let (event_tx, event_rx) = mpmc::unbounded();
        let node_a_endpoint = env.gen_addr();
        let config = NetworkConfig::node().with_endpoint(node_a_endpoint);
        let node_a_network_service = unwrap!(Builder::new(event_tx).with_config(config).build());

        // Construct a `StateMachine` which will start in the `BootstrappingPeer` state and
        // bootstrap off the network service above.
        let node_b_endpoint = env.gen_addr();
        let config = NetworkConfig::node()
            .with_hard_coded_contact(node_a_endpoint)
            .with_endpoint(node_b_endpoint);
        let node_b_full_id = FullId::gen(&mut rng);

        let mut node_b_outbox = Vec::new();

        let (_node_b_action_tx, mut node_b_state_machine) = StateMachine::new(
            move |network_service, timer, _outbox2| {
                State::BootstrappingPeer(BootstrappingPeer::new(BootstrappingPeerDetails {
                    network_service,
                    full_id: node_b_full_id,
                    network_cfg,
                    timer,
                    rng,
                }))
            },
            config,
            &mut node_b_outbox,
        );

        // Check the network service received `ConnectedTo`.
        env.poll();
        match unwrap!(event_rx.try_recv()) {
            NetworkEvent::ConnectedTo {
                peer: Peer::Node { .. },
            } => (),
            ev => panic!(
                "Should have received `ConnectedTo` event, received `{:?}`.",
                ev
            ),
        }

        // The state machine should have received the `BootstrappedTo` event and this will have
        // caused it to send a `BootstrapRequest` message.
        env.poll();
        step_at_least_once(&mut node_b_state_machine, &mut node_b_outbox);

        // Check the network service received the `BootstrapRequest`
        env.poll();
        if let NetworkEvent::NewMessage { peer_addr, msg } = unwrap!(event_rx.try_recv()) {
            assert_eq!(peer_addr, node_b_endpoint);

            let message = unwrap!(Message::from_bytes(&msg));
            match message.variant {
                Variant::BootstrapRequest(_) => (),
                _ => panic!("Should have received a `BootstrapRequest`."),
            };
        } else {
            panic!("Should have received `NewMessage` event.");
        }

        // Drop the network service...
        drop(node_a_network_service);
        env.poll();

        // ...which triggers `ConnectionFailure` on the state machine which then attempts to
        // rebootstrap..
        step_at_least_once(&mut node_b_state_machine, &mut node_b_outbox);
        assert!(node_b_outbox.is_empty());
        env.poll();

        // ... but there is no one to bootstrap to, so the bootstrap fails which causes the state
        // machine to terminate.
        step_at_least_once(&mut node_b_state_machine, &mut node_b_outbox);
        assert_eq!(node_b_outbox.len(), 1);
        assert_eq!(node_b_outbox[0], Event::Terminated);
    }

    fn step_at_least_once(machine: &mut StateMachine, outbox: &mut dyn EventBox) {
        let mut sel = mpmc::Select::new();
        machine.register(&mut sel);

        // Blocking step for the first one.
        let op_index = sel.ready();
        unwrap!(machine.step(op_index, outbox));

        // Exhaust any remaining step
        loop {
            let mut sel = mpmc::Select::new();
            machine.register(&mut sel);

            if let Ok(op_index) = sel.try_ready() {
                unwrap!(machine.step(op_index, outbox));
            } else {
                break;
            }
        }
    }
}
