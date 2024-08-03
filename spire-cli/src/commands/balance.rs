use alloy::primitives::Address;
use clap::{Args, Subcommand};
use spire_sdk::SPVM;

#[derive(Clone, Subcommand)]
pub(crate) enum BalanceCommands {
    #[clap(short_flag = 'S')]
    Set(SetCommand),

    #[clap(short_flag = 'V')]
    View(ViewCommand),
}

#[derive(Args, Clone)]
#[command(about = "TODO")]
pub(crate) struct SetCommand {
    #[clap(short, long)]
    pub(crate) ticker: String,

    #[clap(short, long)]
    pub(crate) holder: Address,

    #[clap(short, long)]
    pub(crate) balance: u16,
}

#[derive(Args, Clone)]
#[command(about = "TODO")]
pub(crate) struct ViewCommand {
    #[clap(short, long)]
    pub(crate) ticker: String,

    #[clap(short, long)]
    pub(crate) holder: Address,
}

impl SetCommand {
    pub(crate) fn run(&self, spvm: &mut SPVM) -> anyhow::Result<()> {
        spvm.set_balance(&self.ticker, &self.holder, self.balance)?;
        Ok(())
    }
}

impl ViewCommand {
    pub(crate) fn run(&self, spvm: SPVM) -> anyhow::Result<()> {
        let balance = spvm.balance(&self.ticker, &self.holder)?;
        print!("Balance: {}", balance);
        Ok(())
    }
}
