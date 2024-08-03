use super::TransactionContent;
use alloy::primitives::{Bytes, FixedBytes};

pub type Bytes32 = FixedBytes<32>;

#[derive(Clone)]
pub struct SPVMTransaction {
    pub(crate) content: TransactionContent,
    pub(crate) transaction_hash: Bytes32,
    pub(crate) signature: Bytes,
}

impl SPVMTransaction {
    pub fn new(
        content: &TransactionContent,
        transaction_hash: &Bytes32,
        signature: &Bytes,
    ) -> Self {
        Self {
            content: content.clone(),
            transaction_hash: *transaction_hash,
            signature: signature.clone(),
        }
    }
}
