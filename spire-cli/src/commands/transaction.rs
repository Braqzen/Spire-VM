use alloy::primitives::{Address, Bytes};
use clap::{Args, Subcommand, ValueEnum};
use spire_sdk::{Bytes32, SPVMTransaction, TransactionContent, TxType, SPVM};

#[derive(Clone, Subcommand)]
pub(crate) enum TransactionCommands {
    #[clap(short_flag = 'E')]
    Execute(ExecuteCommand),

    #[clap(short_flag = 'R')]
    ExecuteRaw(ExecuteRawCommand),
}

#[derive(Args, Clone)]
#[command(about = "TODO")]
pub(crate) struct ExecuteCommand {
    #[clap(short, long)]
    pub(crate) from: Address,

    #[clap(short, long)]
    pub(crate) tx_type: Type,

    #[clap(short, long)]
    pub(crate) tx_param: Bytes,

    #[clap(short, long)]
    pub(crate) nonce: u32,

    #[clap(short, long)]
    pub(crate) transaction_hash: Bytes32,

    #[clap(short, long)]
    pub(crate) signature: Bytes,
}

#[derive(Args, Clone)]
#[command(about = "TODO")]
pub(crate) struct ExecuteRawCommand {
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
