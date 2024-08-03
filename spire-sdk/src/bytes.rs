use alloy::primitives::{Address, Bytes};

pub(crate) trait Decode {
    type Item;

    fn decode(bytes: Bytes) -> anyhow::Result<Self::Item>;
}

pub(crate) trait Encode {
    fn encode(&self) -> anyhow::Result<Bytes>;
}

pub(crate) fn create_concatenated_key(ticker: &String, holder: &Address) -> Vec<u8> {
    let ticker = ticker.as_bytes();
    let holder = holder.to_vec();

    let mut concatenated = Vec::with_capacity(ticker.len() + holder.len());
    concatenated.extend_from_slice(ticker);
    concatenated.extend_from_slice(holder.as_slice());
    concatenated
}
