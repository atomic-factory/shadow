//! Darwinia Runtime
#![cfg(feature = "runtime")]

use crate::{
    chain::{
        ethereum::EthereumRelayHeaderParcel, proxy_type::ProxyType, RelayAffirmation,
        RelayAffirmationId, RelayVotingState,
    },
    frame::{
        ethereum::{backing::EthereumBacking, game::EthereumRelayerGame, relay::EthereumRelay},
        proxy::Proxy,
        sudo::Sudo,
        technical_committee::TechnicalCommittee,
        bridge::relay_authorities::EthereumRelayAuthorities,
    },
};

use substrate_subxt::{
    balances::{AccountData, Balances},
    extrinsic::DefaultExtra,
    sp_core,
    sp_runtime::{
        generic::Header,
        traits::{BlakeTwo256, IdentifyAccount, Verify},
        MultiSignature, OpaqueExtrinsic,
    },
    system::System,
    Runtime,
};
use crate::frame::bridge::relay_authorities::RelayAuthority;

/// Darwinia Runtime
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DarwiniaRuntime;
impl Runtime for DarwiniaRuntime {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;
}

impl Balances for DarwiniaRuntime {
    type Balance = u128;
}

impl System for DarwiniaRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = Self::AccountId;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl TechnicalCommittee for DarwiniaRuntime {}
impl Sudo for DarwiniaRuntime {}
impl EthereumRelay for DarwiniaRuntime {
    type RingBalance = u128;
    type EthereumBlockNumber = u64;
    type PendingRelayHeaderParcel = (
        <Self as System>::BlockNumber,
        EthereumRelayHeaderParcel,
        RelayVotingState<<Self as System>::AccountId>,
    );
    type RelayAffirmationId = RelayAffirmationId<Self::EthereumBlockNumber>;
}

impl EthereumRelayerGame for DarwiniaRuntime {
    type RelayAffirmation = RelayAffirmation<
        EthereumRelayHeaderParcel,
        <Self as System>::AccountId,
        <Self as Balances>::Balance,
        RelayAffirmationId<u64>,
    >;
}

impl EthereumBacking for DarwiniaRuntime {
    type EthereumTransactionIndex = u64;
    type RelayAuthoritySigner = EcdsaAddress;
    type RingBalance = u128;
    type KtonBalance = u128;
}

impl Proxy for DarwiniaRuntime {
    type ProxyType = ProxyType;
}

impl EthereumRelayAuthorities for DarwiniaRuntime {
    type RelayAuthoritySigner = EcdsaAddress;
    type RelayAuthority = RelayAuthority<
        <Self as System>::AccountId,
        Self::RelayAuthoritySigner,
        <Self as Balances>::Balance,
        <Self as System>::BlockNumber
    >;
    type RelayAuthoritySignature = EcdsaSignature;
    type RelayAuthorityMessage = EcdsaMessage;
}

use codec::{Encode, Decode};

/// EcdsaAddress
pub type EcdsaAddress = [u8; 20];

/// EcdsaSignature
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
pub struct EcdsaSignature(pub [u8; 65]);

impl Default for EcdsaSignature {
    fn default() -> Self {
        Self([0u8; 65])
    }
}

/// EcdsaMessage
pub type EcdsaMessage = [u8; 32];
