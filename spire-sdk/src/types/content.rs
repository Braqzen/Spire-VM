use crate::bytes::{Decode, Encode};
use alloy::primitives::{Address, Bytes};
use anyhow::bail;
use ethabi::{decode, encode, ParamType, Token};

pub struct TransactionContent {
    pub(crate) from: Address,
    pub(crate) tx_type: TxType, // only first 2 bits used
    pub(crate) tx_param: Bytes, // abi encoded parameters
    pub(crate) nonce: u32,
}

pub enum TxType {
    Zero,
    One,
}

impl TryFrom<u8> for TxType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TxType::Zero),
            1 => Ok(TxType::One),
            _ => Err("Invalid TxType"),
        }
    }
}

impl TxType {
    fn to_u8(&self) -> u8 {
        match self {
            TxType::Zero => 0,
            TxType::One => 1,
        }
    }
}

impl Decode for TransactionContent {
    type Item = Self;

    fn decode(bytes: Bytes) -> anyhow::Result<Self> {
        let param_types = vec![
            ParamType::Address,
            ParamType::Uint(8),
            ParamType::Bytes,
            ParamType::Uint(32),
        ];

        let tokens = decode(&param_types, &bytes)?;

        let from = match tokens.first().expect("Invalid data length: Address") {
            Token::Address(data) => Address(data.0.into()),
            _ => bail!("Invalid address data"),
        };
        let tx_type = match tokens.get(1).expect("Invalid data length: TxType") {
            Token::Uint(data) => {
                TxType::try_from((data.as_u32() & 0b11) as u8).expect("Invalid TxType")
            }
            _ => bail!("Invalid txtype data"),
        };
        let tx_param = match tokens.get(2).expect("Invalid data length: TxParam") {
            Token::Bytes(data) => Bytes::from(data.clone()),
            _ => bail!("Invalid bytes data"),
        };
        let nonce = match tokens.get(3).expect("Invalid data length: nonce") {
            Token::Uint(data) => data.as_u32(),
            _ => bail!("Invalid nonce data"),
        };

        Ok(Self {
            from,
            tx_type,
            tx_param,
            nonce,
        })
    }
}

impl Encode for TransactionContent {
    fn encode(&self) -> anyhow::Result<Bytes> {
        let from = Token::Address(ethabi::Address::from_slice(&self.from.0 .0));
        let tx_type = Token::Uint(self.tx_type.to_u8().into());
        let tx_param = Token::Bytes(self.tx_param.to_vec());
        let nonce = Token::Uint(self.nonce.into());

        Ok(encode(&[from, tx_type, tx_param, nonce]).into())
    }
}
