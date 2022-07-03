use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount,Transfer};

declare_id!("F3KPspDVmA17HrYfE7nTVpk21egB6c1qxv287upKAJU1");

#[program]
pub mod nft_garbage_collector {
    use super::*;

    pub fn initialize_garbage_collection(ctx: Context<InitializeGarbageCollection>, should_mint_ticket: bool) -> Result<()> {
        //TODO
        //0. Confirm Token is NFT
        //1. Transfer the NFT to the Garbage Wallet, with optional msg?
        //2. If should_mint_ticket Mint an museum Ticket/Recipt for the user

        Ok(())
    }
}

/* I was going to have an exchange for ticket
 * function but I think I can do it all in the init, that being 
 * 1. take the nft
 * 2. send it  to the garbage wallet
 * 4. mint a "ticket" or "reciept" back to the user"
 * I can do a "are you sure" in the client to save resources here. 
*/
#[derive(Accounts)]
pub struct InitializeGarbageCollection<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub initializer_deopsit_token_account: Account<'info, TokenAccount>,
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
