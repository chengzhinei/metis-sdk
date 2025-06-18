use serde::{Deserialize, Serialize};

use crate::message::signed::SignedMessage;

/// The different kinds of messages that can appear in blocks, ie. the transactions
/// we can receive from Tendermint through the ABCI.
///
/// Unlike Filecoin, we don't have `Unsigned` messages here. In Filecoin, the messages
/// signed by BLS signatures are aggregated to the block level, and their original
/// signatures are stripped from the messages, to save space. Tendermint Core will
/// not do this for us (perhaps with ABCI++ Vote Extensions we could do it), though.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChainMessage {
    /// A message that can be passed on to the FVM as-is.
    Signed(SignedMessage),
}
//
// #[cfg(test)]
// mod tests {
//     use crate::chain::ChainMessage;
//     use quickcheck_macros::quickcheck;
//
//     #[quickcheck]
//     fn chain_message_cbor(value0: ChainMessage) {
//         let repr = fvm_ipld_encoding::to_vec(&value0).expect("failed to encode");
//         let value1: ChainMessage =
//             fvm_ipld_encoding::from_slice(repr.as_ref()).expect("failed to decode");
//
//         assert_eq!(value1, value0)
//     }
// }
