use anchor_lang::prelude::*;

declare_id!("33gkQS5JjgMQ7dneNwcaEaCeSHv3v4PV3i3uCFWtkdTQ");

#[program]
pub mod day_04 {
    use super::*;

    pub fn limit_range(ctx_then: Context<LimitRange>, a: u64) -> Result<()> {
        require!(a >= 10, MyError::AisTooSmall);
        require!(a <= 100, MyError::AisTooBig);
        msg!("Result = {}", a);
        Ok(())
    }

    // NEW FUNCTION
    pub fn func(ctx: Context<LimitRange>) -> Result<()> {
        msg!("Will this print?");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LimitRange {}

#[error_code]
pub enum MyError {
    #[msg("a is too small")]
    AisTooSmall,
    #[msg("a is too big")]
    AisTooBig,
    #[msg("Always errors")] // NEW ERROR, what do you think the error code will be?
    AlwaysErrors,
}
