use anchor_lang::prelude::*;

declare_id!("GpmX2Botpq52CLg27wbEXBEN2TCod67Jrovhdsgto455");

#[program]
pub mod anchor_solana_transaction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
