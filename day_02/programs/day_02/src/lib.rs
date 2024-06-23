use anchor_lang::prelude::*;

declare_id!("Cz6CpA6NjFyHrx6fxMZgwCMebJspqw9NqZuowMRtGUAg");

#[program]
pub mod day_02 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, mesage: String) -> Result<()> {
        msg!("You said {:?}", mesage);
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }

    pub fn overflow(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let x: u64 = a.checked_sub(b).unwrap();
        msg!("x is {}", x);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
