use alloy::primitives::{Address, Bytes};

pub(crate) trait Decode {
    type Item;

    fn decode(bytes: Bytes) -> anyhow::Result<Self::Item>;
}

pub(crate) trait Encode {
    fn encode(&self) -> anyhow::Result<Bytes>;
}

pub(crate) fn create_concatenated_key(ticker: &String, account: &Address) -> Vec<u8> {
    let ticker = ticker.as_bytes();
    let account = account.to_vec();

    let mut concatenated = Vec::with_capacity(ticker.len() + account.len());
    concatenated.extend_from_slice(ticker);
    concatenated.extend_from_slice(account.as_slice());
    concatenated
}
