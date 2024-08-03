use alloy::primitives::{Address, Bytes};
use clap::{Args, Subcommand, ValueEnum};
use spire_sdk::{Bytes32, SPVMTransaction, TransactionContent, TxType, SPVM};

#[derive(Clone, Subcommand)]
pub(crate) enum TransactionCommands {
    /// Execute a transaction in SPVM format
    #[clap(short_flag = 'E')]
    Execute(ExecuteCommand),

    /// Execute a transaction directly from Bytes
    #[clap(short_flag = 'R')]
    ExecuteRaw(ExecuteRawCommand),
}

#[derive(Args, Clone)]
pub(crate) struct ExecuteCommand {
    /// User account
    #[clap(short, long)]
    pub(crate) from: Address,

    /// Type of transaction: mint or transfer
    ///
    /// Encoded as Zero (mint) and One (transfer)
    #[clap(short, long)]
    pub(crate) tx_type: Type,

    /// Transaction parameters encoded as bytes
    #[clap(short = 'p', long)]
    pub(crate) tx_param: Bytes,

    /// User nonce
    #[clap(short, long)]
    pub(crate) nonce: u32,

    /// Transaction hash
    #[clap(short = 'x', long)]
    pub(crate) transaction_hash: Bytes32,

    /// User signature
    #[clap(short, long)]
    pub(crate) signature: Bytes,
}

#[derive(Args, Clone)]
pub(crate) struct ExecuteRawCommand {
    /// Transaction encoded into raw bytes
    #[clap(short, long)]
    pub(crate) transaction: Bytes,
}

impl ExecuteCommand {
    pub(crate) fn run(&self, spvm: &mut SPVM) -> anyhow::Result<()> {
        let tx_content = TransactionContent::new(
            &self.from,
            &self.tx_type.clone().into(),
            &self.tx_param,
            self.nonce,
        );
        let transaction =
            SPVMTransaction::new(&tx_content, &self.transaction_hash, &self.signature);

        spvm.execute_transaction(transaction)?;
        Ok(())
    }
}

impl ExecuteRawCommand {
    pub(crate) fn run(&self, spvm: &mut SPVM) -> anyhow::Result<()> {
        spvm.execute_raw_transaction(self.transaction.clone())?;
        Ok(())
    }
}

// Do not pollute external crate by adding ValueEnum from clap
// Create a wrapper for clap locally
#[derive(Clone, ValueEnum)]
pub enum Type {
    Zero,
    One,
}

impl From<Type> for TxType {
    fn from(wrapper: Type) -> Self {
        match wrapper {
            Type::Zero => Self::Zero,
            Type::One => Self::One,
        }
    }
}
