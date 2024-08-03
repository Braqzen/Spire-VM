mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Command};
use commands::{
    balance::BalanceCommands, signature::SignatureCommands, transaction::TransactionCommands,
};
use spire_sdk::SPVM;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let database_path = std::fs::canonicalize("../rocksdb")?
        .as_path()
        .to_str()
        .expect("Failed character encoding in path.")
        .into();
    let mut vm = SPVM::new(database_path)?;

    match cli.command {
        Command::Balance(args) => match args.commands {
            BalanceCommands::Set(args) => args.run(&mut vm),
            BalanceCommands::View(args) => args.run(vm),
        },
        Command::Signature(args) => match args.commands {
            SignatureCommands::Validate(args) => args.run(vm),
        },
        Command::Transaction(args) => match args.commands {
            TransactionCommands::Execute(args) => args.run(&mut vm),
            TransactionCommands::ExecuteRaw(args) => args.run(&mut vm),
        },
    }
}
