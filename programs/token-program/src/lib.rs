use anchor_lang::prelude::*;

declare_id!("AsBA4x7LvWPULfGXrDu8T62RRbrTG3dVnc54bdsgxXtH");

#[program]
pub mod token_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
