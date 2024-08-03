use alloy::primitives::{Address, Bytes};
use clap::{Args, Subcommand};
use spire_sdk::{Bytes32, SPVM};

#[derive(Clone, Subcommand)]
pub(crate) enum SignatureCommands {
    #[clap(short_flag = 'V')]
    Validate(ValidateCommand),
}

#[derive(Args, Clone)]
#[command(about = "TODO")]
pub(crate) struct ValidateCommand {
    #[clap(short, long)]
    pub(crate) message_hash: Bytes32,

    #[clap(short, long)]
    pub(crate) signature: Bytes,

    #[clap(short, long)]
    pub(crate) signer: Address,
}

impl ValidateCommand {
    pub(crate) fn run(&self, spvm: SPVM) -> anyhow::Result<()> {
        match spvm.validate_signature(&self.message_hash, &self.signature, &self.signer)? {
            true => println!("Valid signature"),
            false => eprintln!("Invalid signature"),
        };
        Ok(())
    }
}
