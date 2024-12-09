use anchor_lang::prelude::*;

declare_id!("5gwi1XCbAsNmYvXHNcYzGWZBzNuCnRrgqnmML9uVCoY8");

#[program]
pub mod cryptobar {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
