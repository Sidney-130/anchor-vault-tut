use anchor_lang::prelude::*;

declare_id!("GntMvsDC1M87sTGQaPCB2apQjkzt2ReeBJbKBzhf3exv");

#[program]
pub mod anchor_vault_tut {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
