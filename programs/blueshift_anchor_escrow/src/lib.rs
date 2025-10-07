use anchor_lang::prelude::*;

declare_id!("9QYBVyRaoJXXKjGJLDJYs39FkiNhxMeR11nEMhW7boex");

#[program]
pub mod blueshift_anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
