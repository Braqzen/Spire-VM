use crate::commands::{
    balance::BalanceCommands, signature::SignatureCommands, transaction::TransactionCommands,
};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(about = "TODO")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Clone, Subcommand)]
pub(crate) enum Command {
    #[clap(short_flag = 'B')]
    Balance(Balance),

    #[clap(short_flag = 'S')]
    Signature(Signature),

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
