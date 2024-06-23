use anchor_lang::prelude::*;

declare_id!("ERQgbSCrDR18zuqQSUXWuKegb1DUhPqRYqe6QevGsCxR");

#[program]
pub mod day_01 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
