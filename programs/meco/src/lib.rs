use anchor_lang::prelude::*;

declare_id!("6CJXS1RoFaa6v53dg1NDitMgC8DSu3jwHDjkik6EYcbW");

#[program]
pub mod meco {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
