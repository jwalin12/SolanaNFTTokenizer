use anchor_lang::prelude::*;

use anchor_spl::token::{self, Burn, MintTo,Mint, SetAuthority, Transfer,TokenAccount, Token, mint_to};

use metaplex_token_metadata::state::*;

use std::vec;


declare_id!("4Vra8dFsZQWY7DpPJaUWBnZd4RsLN1J8LXJ28WnBFVP3");


#[program]
pub mod solana_nft_tokenizer {
   
    use super::*;

    //TODO: pass in authority as account instead of pubkey
    pub fn initialize_vault(ctx: Context<InitializeVault>, _mint_bump: u8, vault_name: String, nft_creators: Vec<Pubkey>, nft_symbol: String, mint_fee: u64) -> ProgramResult {
        let vault_account = &mut ctx.accounts.vault_account;
        vault_account.mint_fee = mint_fee;
        vault_account.authority = *ctx.accounts.authority.key;
        vault_account.vault_name = vault_name;
        vault_account.vault_mint = *ctx.accounts.vault_mint.to_account_info().key;
        vault_account.nft_creators = nft_creators;
        vault_account.nft_symbol = nft_symbol;
        Ok(())
    }

    pub fn mint_spl_tokens(ctx: Context<MintSPLTokens>) -> ProgramResult {
        let vault_account = &mut ctx.accounts.vault_account;

        if (*ctx.accounts.vault_authority.key != vault_account.authority) {
            return Err(ErrorCode::IncorrectVaultAuthorityError.into());
        }

        let metadata_program_id = Pubkey::new("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s".as_bytes());

        let (metadata_pda,_) = Pubkey::find_program_address(&[
            PREFIX.as_bytes(),
            metadata_program_id.as_ref(),
            ctx.accounts.nft_mint.key().as_ref(),
        ], &metadata_program_id );

        if (metadata_pda != *ctx.accounts.nft_metadata_account.key) {
            return Err(ErrorCode::IncorrectnftMetadata.into());
        }
        
        let metadata= Metadata::from_account_info(&ctx.accounts.nft_metadata_account)?;
    
        if (metadata.mint != ctx.accounts.nft_mint.key()) {
            return Err(ErrorCode::IncorrectnftMintError.into());
        }

        let data = metadata.data;

        

        if (data.symbol != vault_account.nft_symbol ) {
            return Err(ErrorCode::IncorrectnftSymbolError.into());
        };

        let creators = data.creators.unwrap_or(Vec::new());

        let mut creator_pub_keys =Vec::new();

        for creator in creators.iter() {
            creator_pub_keys.push(creator.address);

        }


        if (creator_pub_keys != vault_account.nft_creators) {
            return Err(ErrorCode::IncorrectnftCreatorsError.into());
        };


        let nft_mint = &ctx.accounts.nft_mint;
        let tokenizer_pda = Pubkey::find_program_address(&[ctx.program_id.as_ref(), nft_mint.key().as_ref()], ctx.program_id);
        //approve tokenizer to transfer nft

        if(ctx.accounts.vault_mint.key() != vault_account.vault_mint) {
            return Err(ErrorCode::IncorrectVaultMint.into());
        }

        //transfer nft to vault_account and store nft mint
        let transfer_res = anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.nft_mint.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx
                        .accounts
                        .user.to_account_info(),
                    to: ctx.accounts.nft_account //TODO: this should be vault_account's token account
                        .to_account_info(),
                    authority: ctx
                    .accounts
                    .user.to_account_info(),
                },
            ),
            1,
        );

        if (transfer_res.is_err()){
            return Err(transfer_res.err().unwrap().into());

        }

        vault_account.nft_accounts.push(ctx.accounts.nft_account //TODO: this should be vault_account's token account
        .key());
        //mint 1000 tokens from the vault
        let mint_res = mint_to(
            CpiContext::new(
                ctx.accounts.nft_mint.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.vault_mint.to_account_info(),
                    to: ctx.accounts.user.to_account_info(),
                    authority: vault_account.to_account_info(),
                },
            ),
            1000 * (1-vault_account.mint_fee/1000),
        );

        let mint_res = mint_to(
            CpiContext::new(
                ctx.accounts.nft_mint.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.vault_mint.to_account_info(),
                    to: ctx.accounts.vault_authority.to_account_info(),
                    authority: vault_account.to_account_info(),
                },
            ),
            1000 * (vault_account.mint_fee/1000),
        );

        if (mint_res.is_err()) {
            return Err(mint_res.err().unwrap().into());

        }
    
        Ok(())

    }

}


    // pub fn exchange_tokens_for_nft(ctx: Context<ExchangeTokensFornft>) -> ProgramResult {
    //     Ok(())

    // }

    // pub fn collect_fees(ctx: Context<CollectFees>) -> ProgramResult {
    //     Ok(())
    // }



    #[derive(Accounts)]
    #[instruction(_mint_bump: u8)]
    pub struct InitializeVault<'info> {
        #[account(init, payer = authority, space = 64+64)]
        pub vault_account: Account<'info, VaultAccount>,
        #[account(mut)]
        pub authority: Signer<'info>,
        #[account(init,
            payer = authority,
            mint::decimals = 16,
        mint::authority = vault_account,
        seeds = [b"mint".as_ref(), vault_account.key().as_ref()],
        bump = _mint_bump,
        )]
        pub vault_mint: Account<'info, Mint>,
        pub rent: Sysvar<'info, Rent>,
        pub token_program: Program<'info, Token>,
        pub system_program: Program<'info, System>,

    }

    #[account]
    pub struct VaultAccount{
        pub mint_fee: u64, //divided by 1000 for minting fee, so 1 is 0.1% fee
        pub vault_mint: Pubkey,
        pub authority: Pubkey,
        pub vault_name: String,
        pub nft_creators: Vec<Pubkey>,
        pub nft_symbol: String,
        pub nft_accounts: Vec<Pubkey>

    }

    #[derive(Accounts)]
    pub struct MintSPLTokens<'info> {
        pub user: Signer<'info>,
        pub vault_account: Account<'info, VaultAccount>,
        pub vault_mint: Account<'info, Mint>,
        pub vault_authority: AccountInfo<'info>,
        pub nft_metadata_account: AccountInfo<'info>,
        #[account(init,
            payer = user,
            token::mint = nft_mint,
            token::authority = vault_account)]
        pub nft_account: Account<'info,TokenAccount>,
        #[account(mut)]
        pub nft_mint:  Account<'info, Mint>,
        pub token_program: Program<'info, Token>,
        pub system_program: Program<'info, System>,
        pub rent: Sysvar<'info, Rent>
        
    }

    #[derive(Accounts)]
    pub struct CollectFees<'info> {
        #[account(mut, has_one = authority)]
        pub vault: Account<'info, VaultAccount>,
        pub authority: Signer<'info>
    }


    #[error]
    pub enum ErrorCode {
        #[msg("Incorrect nft for vault, wrong creators")]
        IncorrectnftCreatorsError,
        #[msg("Incorrect nft for vault, wrong symbol")]
        IncorrectnftSymbolError,
        #[msg("Signer is not depositor")]
        IncorrectDepositorError,
        #[msg("Incorrect Vault Mint passed")]
        IncorrectVaultMint,
        #[msg("Incorrect nft Metadata passed")]
        IncorrectnftMetadata,
        #[msg("Incorrect nft Mint passed")]
        IncorrectnftMintError,
        #[msg("Incorrect Vault Authority passed")]
        IncorrectVaultAuthorityError,


    }


