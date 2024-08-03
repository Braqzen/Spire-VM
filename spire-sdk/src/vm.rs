use crate::{
    bytes::{create_concatenated_key, Decode, Encode},
    db::Database,
    types::{
        Bytes32, MintTransaction, SPVMTransaction, TransactionContent, TransferTransaction, TxType,
    },
};
use alloy::primitives::{keccak256, Address, Bytes};
use anyhow::{bail, Ok};
use secp256k1::{ecdsa::RecoverableSignature, ecdsa::RecoveryId, Message, Secp256k1};
use tiny_keccak::{Hasher, Keccak};

#[allow(clippy::upper_case_acronyms)]
pub struct SPVM {
    db: Database,
}

impl SPVM {
    pub fn new(path: String) -> anyhow::Result<Self> {
        Ok(Self {
            db: Database::new(path)?,
        })
    }

    pub fn set_balance(
        &mut self,
        ticker: &String,
        holder: &Address,
        balance: u16,
    ) -> anyhow::Result<()> {
        let key = create_concatenated_key(ticker, holder);

        self.db.put(ticker.as_bytes(), &[1]);
        self.db.put(&key[..], &balance.to_be_bytes());

        Ok(())
    }

    pub fn balance(&self, ticker: &String, holder: &Address) -> anyhow::Result<u16> {
        let key = create_concatenated_key(ticker, holder);
        let value = self.db.get(&key)?;

        match value {
            Some(value) => {
                if value.len() != 2 {
                    bail!("Value does not conform to a u16")
                }
                Ok(u16::from_be_bytes([value[0], value[1]]))
            }
            None => bail!("Value is none"),
        }
    }

    pub fn execute_raw_transaction(&mut self, transaction: Bytes) -> anyhow::Result<()> {
        let tx_content = TransactionContent::decode(transaction)?;

        if !self.verify(&tx_content)? {
            bail!("Invalid transaction");
        }

        match &tx_content.tx_type {
            TxType::Zero => {
                let mint = MintTransaction::decode(tx_content.tx_param.clone())?;

                self.set_balance(&mint.token_ticker, &mint.owner, mint.supply)?;
            }
            TxType::One => {
                let transfer = TransferTransaction::decode(tx_content.tx_param.clone())?;

                self.set_balance(
                    &transfer.token_ticker,
                    &tx_content.from,
                    self.balance(&transfer.token_ticker, &tx_content.from)? - transfer.amount,
                )?;

                self.set_balance(
                    &transfer.token_ticker,
                    &transfer.to,
                    self.balance(&transfer.token_ticker, &transfer.to)? + transfer.amount,
                )?;
            }
        }

        let holder = tx_content.from.to_vec();

        match self.db.get(&holder)? {
            Some(value) => {
                let mut nonce = u32::from_be_bytes([value[0], value[1], value[2], value[3]]);
                nonce += 1;
                self.db.put(&holder, &nonce.to_be_bytes()[..]);
            }
            None => self.db.put(&holder, &1u32.to_be_bytes()[..]),
        }

        Ok(())
    }

    fn verify(&self, tx_content: &TransactionContent) -> anyhow::Result<bool> {
        match tx_content.tx_type {
            TxType::Zero => {
                let mint = MintTransaction::decode(tx_content.tx_param.clone())?;

                match self.db.get(mint.token_ticker.as_bytes())? {
                    Some(value) => {
                        if value.len() != 1 {
                            bail!("The vector must contain exactly one byte to convert to bool");
                        }

                        if value[0] == 0 {
                            bail!("Token already initialized");
                        }
                    }
                    None => bail!("TODO"),
                };
            }
            TxType::One => {
                let transfer = TransferTransaction::decode(tx_content.tx_param.clone())?;

                match self.db.get(transfer.token_ticker.as_bytes())? {
                    Some(value) => {
                        if value.len() != 1 {
                            bail!("The vector must contain exactly one byte to convert to bool");
                        }
                        if value[0] == 1 {
                            bail!("Token not initialized");
                        }
                    }
                    None => bail!("TODO"),
                };

                if self.balance(&transfer.token_ticker, &transfer.to)? < transfer.amount {
                    bail!("Insufficient balance");
                }
            }
        }

        match self.db.get(tx_content.from.as_ref())? {
            Some(value) => {
                let nonce = u32::from_be_bytes([value[0], value[1], value[2], value[3]]);

                if nonce != tx_content.nonce {
                    bail!("Invalid nonce");
                }
            }
            None => bail!("Nonce is none"),
        };

        Ok(true)
    }

    pub fn validate_signature(
        &self,
        message_hash: &Bytes32,
        signature: &Bytes,
        expected_signer: &Address,
    ) -> anyhow::Result<bool> {
        let secp = Secp256k1::new();
        let mut hasher = Keccak::v256();

        let r = &signature[0..32];
        let s = &signature[32..64];
        let v = signature[64];

        let recovery_id = RecoveryId::from_i32((v - 27) as i32)?;
        let rec_signature = RecoverableSignature::from_compact(&[r, s].concat(), recovery_id)?;

        let message = Message::from_digest(**message_hash);
        let public_key = secp.recover_ecdsa(&message, &rec_signature)?;
        let public_key = public_key.serialize_uncompressed();

        let mut output = [0u8; 32];
        hasher.update(&public_key[1..]);
        hasher.finalize(&mut output);

        Ok(Address::from_slice(&output[12..]) == *expected_signer)
    }

    pub fn execute_transaction(&mut self, transaction: SPVMTransaction) -> anyhow::Result<()> {
        let bytes = transaction.content.encode()?;

        if keccak256(&bytes) != transaction.transaction_hash {
            bail!("Invalid transaction hash");
        }

        if !self.validate_signature(
            &transaction.transaction_hash,
            &transaction.signature,
            &transaction.content.from,
        )? {
            bail!("Invalid signature");
        }

        self.execute_raw_transaction(bytes)?;

        Ok(())
    }
}
