use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("6oE4QvenrBDTsw2hBvWhbYnz2VtVA6REJBr57M4ebGPQ");

#[program]
pub mod solana_web3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
