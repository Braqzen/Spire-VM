use alloy::primitives::{Address, Bytes};
use clap::{Args, Subcommand};
use spire_sdk::{Bytes32, SPVM};

#[derive(Clone, Subcommand)]
pub(crate) enum SignatureCommands {
    /// Validate a signature against a signer and message hash
    #[clap(short_flag = 'V')]
    Validate(ValidateCommand),
}

#[derive(Args, Clone)]
pub(crate) struct ValidateCommand {
    /// keccak256 hash of a message
    #[clap(short, long)]
    pub(crate) message_hash: Bytes32,

    /// User signature
    #[clap(short, long)]
    pub(crate) signature: Bytes,

    /// Address to recover against
    #[clap(short = 'a', long)]
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
