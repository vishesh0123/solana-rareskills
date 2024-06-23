use anchor_lang::prelude::*;

declare_id!("Cz6CpA6NjFyHrx6fxMZgwCMebJspqw9NqZuowMRtGUAg");

#[program]
pub mod day_02 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
