use anchor_lang::prelude::*;

declare_id!("32ZqLSSpnx9jdvG8GyAKTXZG6Cx5s6kNMtfxQxvfBnvR");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
