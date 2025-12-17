use anchor_lang::prelude::*;
declare_id!("Fold111111111111111111111111111111111111111");

#[program]
pub mod fold_program {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}