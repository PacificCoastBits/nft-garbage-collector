use anchor_lang::prelude::*;

declare_id!("F3KPspDVmA17HrYfE7nTVpk21egB6c1qxv287upKAJU1");

#[program]
pub mod nft_garbage_collector {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
