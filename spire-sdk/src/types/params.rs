use crate::bytes::Decode;
use alloy::primitives::{Address, Bytes};
use anyhow::bail;
use ethabi::{decode, ParamType, Token};

pub(crate) struct MintTransaction {
    pub(crate) token_ticker: String,
    pub(crate) owner: Address,
    pub(crate) supply: u16,
}

pub(crate) struct TransferTransaction {
    pub(crate) token_ticker: String,
    pub(crate) to: Address,
    pub(crate) amount: u16,
}

impl Decode for MintTransaction {
    type Item = Self;

    fn decode(bytes: Bytes) -> anyhow::Result<Self> {
        let param_types = vec![ParamType::String, ParamType::Address, ParamType::Uint(16)];

        let tokens = decode(&param_types, &bytes)?;

        let token_ticker = match tokens.first().expect("Invalid data length: Ticker") {
            Token::String(data) => data.clone(),
            _ => bail!("Invalid ticker data"),
        };
        let owner = match tokens.get(1).expect("Invalid data length: Owner") {
            Token::Address(ref data) => Address(data.0.into()),
            _ => bail!("Invalid owner data"),
        };
        let supply = match tokens.get(2).expect("Invalid data length: Supply") {
            Token::Uint(data) => data.as_u32() as u16,
            _ => bail!("Invalid supply data"),
        };

        Ok(Self {
            token_ticker,
            owner,
            supply,
        })
    }
}

impl Decode for TransferTransaction {
    type Item = Self;

    fn decode(bytes: Bytes) -> anyhow::Result<Self> {
        let param_types = vec![ParamType::String, ParamType::Address, ParamType::Uint(16)];

        let tokens = decode(&param_types, &bytes)?;

        let token_ticker = match tokens.first().expect("Invalid data length: Ticker") {
            Token::String(data) => data.clone(),
            _ => bail!("Invalid ticker data"),
        };
        let to = match tokens.get(1).expect("Invalid data length: To") {
            Token::Address(ref data) => Address(data.0.into()),
            _ => bail!("Invalid to data"),
        };
        let amount = match tokens.get(2).expect("Invalid data length: Amount") {
            Token::Uint(data) => data.as_u32() as u16,
            _ => bail!("Invalid amount data"),
        };

        Ok(Self {
            token_ticker,
            to,
            amount,
        })
    }
}
