//! Darwinia Bridge Relay Authorities
use codec::{Encode, Decode};
use substrate_subxt::{system::{System, SystemEventsDecoder}};
use substrate_subxt_proc_macro::{module, Event, Store, Call};
use frame_support::sp_runtime::app_crypto::sp_core::H256;
use core::marker::PhantomData;

/// Relay Authority
#[derive(Clone, Encode, Decode, Default, Debug)]
pub struct RelayAuthority<AccountId, RelayAuthoritySigner, RingBalance, BlockNumber> {
    /// account_id
    pub account_id: AccountId,
    /// signer
    pub signer: RelayAuthoritySigner,
    /// Stake balance
    pub stake: RingBalance,
    /// BlockNumber
    pub term: BlockNumber,
}

/// Bridge Relay Authorities Pallet
#[module]
pub trait EthereumRelayAuthorities: System {
    /// Relay Authority
    type RelayAuthority: 'static + Encode + Decode + Send + Default;
    /// Relay authority signer
    type RelayAuthoritySigner: 'static + Encode + Decode + Send + Default;
    /// Relay signature
    type RelayAuthoritySignature: 'static + Encode + Decode + Send + Sync + Default;
    /// Relay signature
    type RelayAuthorityMessage: 'static + Encode + Decode + Send + Default;
}

//////
// Calls
//////

/// Submit authorities signature
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SubmitSignedAuthorities<T: EthereumRelayAuthorities> {
    /// signature
    pub signature: T::RelayAuthoritySignature,
}

/// Submit redeem call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SubmitSignedMmrRoot<T: EthereumRelayAuthorities> {
    /// block_number
    pub block_number: <T as System>::BlockNumber,
    /// mmr_root
    pub mmr_root: H256,
    /// signature
    pub signature: T::RelayAuthoritySignature,
}

//////
// Events
//////

/// A New MMR Root Request to be Signed. [block number]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NewMMRRoot<T: EthereumRelayAuthorities> {
    /// BlockNumber
    pub block_number: <T as System>::BlockNumber,
}

/// Authorities Signed. [term, new authorities, signatures]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct AuthoritiesSetSigned<T: EthereumRelayAuthorities> {
    /// term
    pub term: u32,
    /// new authorities
    pub new_authorities: Vec<T::RelayAuthoritySigner>,
    /// signatures
    pub signatures: Vec<(<T as System>::AccountId, T::RelayAuthoritySignature)>,
}

/// MMR Root Signed. [block number, mmr root, message, signatures]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct MMRRootSigned<T: EthereumRelayAuthorities> {
    /// block number
    pub block_number: u32,
    /// mmr root
    pub mmr_root: H256,
    /// The redeemed balance
    pub signatures: Vec<(<T as System>::AccountId, T::RelayAuthoritySignature)>,
}

/// NewAuthorities. [message to sign]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NewAuthorities<T: EthereumRelayAuthorities> {
    /// message
    pub message: T::RelayAuthorityMessage,
}

//////
// Store
//////

/// Relay Authorities Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct Authorities<T: EthereumRelayAuthorities> {
    #[store(returns = Vec<T::RelayAuthority>)]
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}
