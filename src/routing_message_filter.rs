// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use crate::{
    crypto::Digest256, id::PublicId, location::DstLocation, message_filter::MessageFilter,
    messages::MessageWithBytes,
};
use lru_time_cache::LruCache;
use std::time::Duration;

const INCOMING_EXPIRY_DURATION_SECS: u64 = 60 * 20;
const OUTGOING_EXPIRY_DURATION_SECS: u64 = 60 * 10;

/// An enum representing a result of message filtering
#[derive(Eq, PartialEq)]
pub enum FilteringResult {
    /// We don't have the message in the filter yet
    NewMessage,
    /// We have the message in the filter
    KnownMessage,
}

impl FilteringResult {
    pub fn is_new(&self) -> bool {
        match self {
            Self::NewMessage => true,
            Self::KnownMessage => false,
        }
    }
}

// Structure to filter (throttle) incoming and outgoing `RoutingMessages`.
pub struct RoutingMessageFilter {
    incoming: MessageFilter<Digest256>,
    outgoing: LruCache<(Digest256, PublicId), ()>,
}

impl RoutingMessageFilter {
    pub fn new() -> Self {
        let incoming_duration = Duration::from_secs(INCOMING_EXPIRY_DURATION_SECS);
        let outgoing_duration = Duration::from_secs(OUTGOING_EXPIRY_DURATION_SECS);

        Self {
            incoming: MessageFilter::with_expiry_duration(incoming_duration),
            outgoing: LruCache::with_expiry_duration(outgoing_duration),
        }
    }

    // Filter incoming `RoutingMessage`. Return whether this specific message has already been seen.
    pub fn filter_incoming(&mut self, msg: &MessageWithBytes) -> FilteringResult {
        // Not filtering direct messages.
        if let DstLocation::Direct = msg.message_dst() {
            return FilteringResult::NewMessage;
        }

        let hash = msg.full_crypto_hash();

        if self.incoming.insert(hash) > 1 {
            FilteringResult::KnownMessage
        } else {
            FilteringResult::NewMessage
        }
    }

    // Filter outgoing `RoutingMessage`. Return whether this specific message has been seen recently
    // (and thus should not be sent, due to deduplication).
    //
    // Return `KnownMessage` also if hashing the message fails - that can be handled elsewhere.
    pub fn filter_outgoing(
        &mut self,
        msg: &MessageWithBytes,
        pub_id: &PublicId,
    ) -> FilteringResult {
        // Not filtering direct messages.
        if let DstLocation::Direct = msg.message_dst() {
            return FilteringResult::NewMessage;
        }

        let hash = msg.full_crypto_hash();

        if self.outgoing.insert((*hash, *pub_id), ()).is_some() {
            FilteringResult::KnownMessage
        } else {
            FilteringResult::NewMessage
        }
    }
}

impl Default for RoutingMessageFilter {
    fn default() -> Self {
        Self::new()
    }
}
