use alloy::primitives::Address;
use clap::{Args, Subcommand};
use spire_sdk::SPVM;

#[derive(Clone, Subcommand)]
pub(crate) enum BalanceCommands {
    /// Set the balance of an account
    #[clap(short_flag = 'S')]
    Set(SetCommand),

    /// View the balance of an account
    #[clap(short_flag = 'V')]
    View(ViewCommand),
}

#[derive(Args, Clone)]
pub(crate) struct SetCommand {
    /// Token ticker
    #[clap(short, long)]
    pub(crate) ticker: String,

    /// User account
    #[clap(short, long)]
    pub(crate) account: Address,

    /// Balance to set
    #[clap(short, long)]
    pub(crate) balance: u16,
}

#[derive(Args, Clone)]
pub(crate) struct ViewCommand {
    /// Token ticker
    #[clap(short, long)]
    pub(crate) ticker: String,

    /// User account
    #[clap(short, long)]
    pub(crate) account: Address,
}

impl SetCommand {
    pub(crate) fn run(&self, spvm: &mut SPVM) -> anyhow::Result<()> {
        spvm.set_balance(&self.ticker, &self.account, self.balance)?;
        Ok(())
    }
}

impl ViewCommand {
    pub(crate) fn run(&self, spvm: SPVM) -> anyhow::Result<()> {
        let balance = spvm.balance(&self.ticker, &self.account)?;
        print!("Balance: {}", balance);
        Ok(())
    }
}
