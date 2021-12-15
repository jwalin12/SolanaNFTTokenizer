use anchor_lang::prelude::*;

use anchor_spl::token::{self, Burn, MintTo,Mint, SetAuthority, Transfer,TokenAccount, Token, mint_to};

use metaplex_token_metadata::state::*;

use std::vec;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


#[program]
pub mod solana_nft_tokenizer {
   
    use super::*;

    const metadata_program_id:Pubkey= Pubkey::new("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s".as_bytes());
    const program_id: Pubkey = Pubkey::new("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS".as_bytes());

    pub fn initialize_vault(ctx: Context<InitializeVault>, authority: Pubkey, vault_name: String, NFT_creators: Vec<Creator>, NFT_symbol: String, mint_fee: u64) -> ProgramResult {
        let vault_account = &mut ctx.accounts.vault_account;
        vault_account.mint_fee = mint_fee;
        vault_account.authority = authority;
        vault_account.vault_name = vault_name;
        vault_account.vault_mint = *ctx.accounts.vault_mint.to_account_info().key;
        vault_account.NFT_creators = NFT_creators;
        vault_account.NFT_symbol = NFT_symbol;
        Ok(())
    }

    pub fn mint_SPL_tokens(ctx: Context<MintSPLTokens>, ) -> ProgramResult {
        let vault_account = &mut ctx.accounts.vault_account;
        let (metadata_pda,_) = Pubkey::find_program_address(&[
            PREFIX.as_bytes(),
            metadata_program_id.as_ref(),
            ctx.accounts.NFT_mint.key().as_ref(),
        ], &metadata_program_id );

        if (metadata_pda != *ctx.accounts.NFT_metadata_account.key) {
            Err(ErrorCode::IncorrectNFTMetadata.into());
        }
        let metadata = Metadata::from_account_info(&ctx.accounts.NFT_metadata_account).ok().unwrap_unchecked();
    

        if (metadata.mint != ctx.accounts.NFT_mint.key()) {
            Err(ErrorCode::IncorrectNFTMintError.into());
        }

        let data = metadata.data;

        

        if (data.symbol != vault_account.NFT_symbol ) {
            Err(ErrorCode::IncorrectNFTSymbolError.into());
        };

        if (data.creators != vault_account.NFT_creators ) {
            Err(ErrorCode::IncorrectNFTCreatorsError.into());
        };
        let NFT_mint = ctx.accounts.NFT_mint;
        let tokenizer_pda = Pubkey::find_program_address(&[program_id.as_ref(), NFT_mint.key().as_ref()], &program_id);
        //approve tokenizer to transfer NFT

        if (ctx.accounts.user.key != ctx.accounts.depositer.key) {
            Err(ErrorCode::IncorrectDepositorError.into());
        }

        if(*ctx.accounts.vault_mint.key != vault_account.vault_mint) {
            Err(ErrorCode::IncorrectVaultMint.into());
        }

        //transfer NFT to vault_account and store NFT mint
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.NFT_mint.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx
                        .accounts
                        .depositer,
                    to: ctx.accounts.NFT_account //TODO: this should be vault_account's token account
                        .to_account_info(),
                    authority: ctx
                    .accounts
                    .depositer,
                },
            ),
            1,
        );

        vault_account.NFTs_accounts.push(ctx.accounts.NFT_account //TODO: this should be vault_account's token account
        .key());
        //mint 1000 tokens from the vault
        mint_to(
            CpiContext::new(
                ctx.accounts.NFT_mint.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.vault_mint,
                    to: ctx.accounts.depositer.to_account_info(),
                    authority: vault_account.to_account_info(),
                },
            ),
            1000,
        );
    
        Ok(())

    }

}


    // pub fn exchange_tokens_for_NFT(ctx: Context<ExchangeTokensForNFT>) -> ProgramResult {
    //     Ok(())

    // }

    // pub fn collect_fees(ctx: Context<CollectFees>) -> ProgramResult {
    //     Ok(())
    // }



    #[derive(Accounts)]
    pub struct InitializeVault<'info> {
        #[account(init, payer = vault_creator, space = 64)]
        pub vault_account: Account<'info, VaultAccount>,
        pub vault_creator: Signer<'info>,
        #[account(init,
            payer = vault_creator,
            mint::decimals = 16,
        mint::authority = vault_account
        )]
        pub vault_mint: Account<'info, Mint>,
        pub rent: Sysvar<'info, Rent>,
        pub token_program: Program<'info, Token>,
        pub system_program: Program<'info, System>,

    }

    #[account]
    pub struct VaultAccount{
        pub mint_fee: u64,
        pub vault_mint: Pubkey,
        pub authority: Pubkey,
        pub vault_name: String,
        pub NFT_creators: Vec<Creator>,
        pub NFT_symbol: String,
        pub NFTs_accounts: Vec<Pubkey>

    }

    #[derive(Accounts)]
    pub struct MintSPLTokens<'info> {
        pub user: Signer<'info>,
        pub depositer: AccountInfo<'info>,
        pub vault_account: Account<'info, VaultAccount>,
        pub vault_mint: AccountInfo<'info>,
        pub NFT_metadata_account: AccountInfo<'info>,
        #[account(init,
            payer = user,
            token::mint = NFT_mint,
            token::authority = vault_account)]
        pub NFT_account: Account<'info,TokenAccount>,
        #[account(mut)]
        pub NFT_mint:  Account<'info, Mint>,
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
        #[msg("Incorrect NFT for vault, wrong creators")]
        IncorrectNFTCreatorsError,
        #[msg("Incorrect NFT for vault, wrong symbol")]
        IncorrectNFTSymbolError,
        #[msg("Signer is not depositor")]
        IncorrectDepositorError,
        #[msg("Incorrect Vault Mint passed")]
        IncorrectVaultMint,
        #[msg("Incorrect NFT Metadata passed")]
        IncorrectNFTMetadata,
        #[msg("Incorrect NFT Mint passed")]
        IncorrectNFTMintError,

    }


