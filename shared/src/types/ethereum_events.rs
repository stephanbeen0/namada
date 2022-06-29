//! Types to do with interfacing with the Ethereum blockchain
use std::fmt::Debug;

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use sha2::{Digest, Sha256};

use crate::proto::MultiSigned;
use crate::types::address::Address;
use crate::types::token::Amount;

/// An Ethereum event to be processed by the Anoma ledger
#[derive(
    Debug, PartialEq, Eq, Clone, BorshSerialize, BorshDeserialize, BorshSchema,
)]
pub enum EthereumEvent {
    /// Event transferring batches of ether from Ethereum to wrapped ETH on
    /// Anoma
    TransfersToNamada(Vec<TransferToNamada>),
}

impl EthereumEvent {
    fn hash(&self) -> Result<[u8; 32], std::io::Error> {
        let bytes = self.try_to_vec()?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let hash: [u8; 32] = hasher.finalize().into();
        Ok(hash)
    }
}

/// Representation of address on Ethereum
#[derive(
    Clone, PartialEq, Eq, Debug, BorshSerialize, BorshDeserialize, BorshSchema,
)]
pub struct EthAddress(pub [u8; 20]);

/// An event transferring some kind of value from Ethereum to Anoma
#[derive(
    Debug, PartialEq, Eq, Clone, BorshSerialize, BorshDeserialize, BorshSchema,
)]
pub struct TransferToNamada {
    /// Quantity of ether in the transfer
    pub amount: Amount,
    /// Address on Ethereum of the asset
    pub asset: EthereumAsset,
    /// The Namada address receiving wrapped assets on Anoma
    pub receiver: Address,
}

/// Represents Ethereum assets on the Ethereum blockchain
#[derive(
    Debug, PartialEq, Eq, Clone, BorshSerialize, BorshDeserialize, BorshSchema,
)]
pub enum EthereumAsset {
    /// An ERC20 token and the address of its contract
    ERC20(EthAddress),
}

/// This is created by the block proposer based on the Ethereum events included
/// in the vote extensions of the previous Tendermint round
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct MultiSignedEthEvent {
    /// Address and voting power of the signing validators
    pub signers: Vec<(Address, u64)>,
    /// Events as signed by validators
    pub event: MultiSigned<EthereumEvent>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethereum_event_hash() {
        let event = EthereumEvent::TransfersToNamada(vec![]);

        let hash = event.hash().unwrap();

        assert_eq!(
            hash,
            [
                136, 85, 80, 138, 173, 225, 110, 197, 115, 210, 30, 106, 72,
                93, 253, 10, 118, 36, 8, 92, 26, 20, 181, 236, 221, 100, 133,
                222, 12, 104, 57, 164
            ]
        );
    }
}
