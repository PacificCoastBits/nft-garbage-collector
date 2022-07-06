use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};
use metaplex_token_metadata::state::{PREFIX, EDITION, Metadata};

declare_id!("F3KPspDVmA17HrYfE7nTVpk21egB6c1qxv287upKAJU1");

#[program]
pub mod nft_garbage_collector {
    use super::*;

    pub fn initialize_garbage_collection(ctx: Context<InitializeGarbageCollection>, should_mint_ticket: bool) -> Result<()> {
        //TODO
        //----- 0. Confirm Token is NFT (This could probaby get moved to a helper func)
        let nft_account = &ctx.accounts.initializer_nft_account;
        let user = &ctx.accounts.initializer;
        let nft_mint_account = &ctx.accounts.initializer_nft_mint_account;
       
        //Assert the ownert of the account
        assert_eq!(nft_account.owner, user.key());
        
        //Assert the mint on the token account 
        assert_eq!(nft_account.mint, nft_mint_account.key());

        //Assert the amount on the Token account
        assert_eq!(nft_account.amount,1);
       
        
        //Verify is Master edition NFT - To Avoid getting spammed with shit..
        let master_edition_seed = &[
            PREFIX.as_bytes(),
            ctx.accounts.token_metadata_program.key.as_ref(),
            nft_account.mint.as_ref(),
            EDITION.as_bytes()
        ];
        
        //Find the program adress and bump seeds
        let (master_edition_key, master_edition_seed) = 
            Pubkey::find_program_address(master_edition_seed, ctx.accounts.token_metadata_program.key);

        // Assert that the nft metadata master edition passed in a valid master edition 
        assert_eq!(master_edition_key, ctx.accounts.nft_metadata_account.key());

        // Assert the master edition account is initalized
        if ctx.accounts.nft_metadata_account.data_is_empty(){   
            return Err(ErrorCode::AccountNotInitialized.into())
        }

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
#[instruction(metadata_program_id: Pubkey)]
pub struct InitializeGarbageCollection<'info> {
    #[account(mut)]
    // The user
    pub initializer: Signer<'info>,
    // The nft mint account
    pub initializer_nft_mint_account: Account<'info, Mint>,
    // The users nft account
    pub initializer_nft_account: Account<'info, TokenAccount>,
    // The account that holds the nft metadata
    pub nft_metadata_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(address = metadata_program_id)]
    pub token_metadata_program: AccountInfo<'info>
}