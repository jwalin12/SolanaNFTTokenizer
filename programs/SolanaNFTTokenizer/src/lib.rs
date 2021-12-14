use anchor_lang::prelude::*;

use anchor_spl::token::{self, Burn, MintTo, SetAuthority, Transfer, };

use metaplex_token_metadata::state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// TODO: figure out how NFTs work on Solana

#[program]
pub mod solana_nft_tokenizer {
    const metadata_program_id = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
    const program_id = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS";
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, owner: PubKey, vault_name: String, NFT_creators: <Vec<Creator>>, NFT_symbol: String, mint_fee: uint64) -> ProgramResult {
        let vault_account = &mut ctx.accounts.vault_account;
        vault_account.mint_fee = mint_fee;
        vault_account.owner = owner;
        vault_account.vault_name = vault_name;
        vault_account.vault_mint = ctx.accounts.vault_mint.to_account_info;
        vault_account.NFT_creators = NFT_creators;
        vault_account.NFT_symbol = NFT_symbol;
        Ok(())
    }

    pub fn mint_SPL_tokens(ctx: Context<MintSPLTokens>, ) -> ProgramResult {
        let vault_account = &mut ctx.accounts.vault_account;
        let metadata_pda = Pubkey::find_program_address(&[
            PREFIX.as_bytes(),
            metadata_program_id.as_ref(),
            mint_info.key.as_ref(),
        ], metadata_program_id );
        let data Data = metadata_pda.data;

        if (data.symbol != vault_account.symbol ) {
            Err(ErrorCode::IncorrectNFTSymbolError.into());
        }

        if (data.creators != vault_account.creators ) {
            Err(ErrorCode::IncorrectNFTCreatorsError.into());
        }
        let NFT_mint = ctx.accounts.NFT_mint;
        let tokenizer_pda = PubKey::find_program_address(&[program_id, NFT_mint.PubKey], &program_id)
        //approve tokenizer to transfer NFT
        approve()


        //initialize token account in vault

        //transfer NFT to vault_account and store NFT mint


        //mint 1000 tokens from the vault

        

        Ok(());

    }


    pub fn exchange_tokens_for_NFT(ctx: Context<ExchangeTokensForNFT>) -> ProgramResult {
        Ok(())

    }

    pub fn collect_fees(ctx: Context<CollectFees>) -> ProgramResult {
        Ok(())
    }



    #[derive(Accounts)]
    pub struct InitializeVault {
        #[account(init, payer = creator)]
        pub vault_account: Account<'info, VaultAccount>
        pub vault_creator: Signer<'info>,
        #[account(init,
            token::mint = ownership_token_mint,
            token::authority = Pubkey::find_program_address(&[],&program_id),
            seeds = [format!("{}{}{}{:?}", mint_fee, vault_account, creator.signer_key()), (*"vault_tokenmint").to_string()],
            bump = pool_token_decimals,
            payer = creator)]
        pub vault_mint: Box<Account<'info, Mint>>,
        pub system_program: Program<'info, System>,

    }

    #[account]
    pub struct VaultAccount {
        pub mint_fee: uint64,
        pub owner: PubKey,
        pub vault_mint: AccountInfo<'info>,
        pub vault_name: String,
        pub NFT_creators: <Vec<Creator>>,
        pub NFT_symbol: String,
        pub NFTs_in_vault: <Vec<PubKey>>

    }

    #[derive(Accounts)]
    pub struct MintSPLTokens {
        pub user: Signer<'info>,
        pub vault: Account<'info, VaultAccount>,
        pub NFT_mint: Account<'info, TokenAccount>,
        
    }

    #[derive(Accounts)]
    pub struct CollectFees {
        #[account(mut, has_one = authority)]
        pub vault: Account<'info, VaultAccount>,
        pub owner: Signer<'info>
    }


    #[error]
    pub enum ErrorCode {
        #[msg("Incorrect NFT for vault, wrong creators")]
        IncorrectNFTCreatorsError,
        #[msg("Incorrect NFT for vault, wrong symbol")]
        IncorrectNFTSymbolError,
    }

}
