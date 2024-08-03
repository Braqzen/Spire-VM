use crate::commands::{
    balance::BalanceCommands, signature::SignatureCommands, transaction::TransactionCommands,
};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(about = "Tool for interacting with the Spite VM")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Clone, Subcommand)]
pub(crate) enum Command {
    /// Set or view the balance of an account
    #[clap(short_flag = 'B')]
    Balance(Balance),

    /// Check signature validity
    #[clap(short_flag = 'S')]
    Signature(Signature),

    /// Execute a regular or raw transaction
    #[clap(short_flag = 'T')]
    Transaction(Transaction),
}

#[derive(Args, Clone)]
pub(crate) struct Balance {
    #[clap(subcommand)]
    pub(crate) commands: BalanceCommands,
}

#[derive(Args, Clone)]
pub(crate) struct Transaction {
    #[clap(subcommand)]
    pub(crate) commands: TransactionCommands,
}

#[derive(Args, Clone)]
pub(crate) struct Signature {
    #[clap(subcommand)]
    pub(crate) commands: SignatureCommands,
}
