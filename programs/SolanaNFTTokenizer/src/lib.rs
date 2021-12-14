use anchor_lang::prelude::*;

use anchor_spl::token::{self, Burn, MintTo, SetAuthority, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_nft_tokenizer {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> ProgramResult {
        Ok(())
    }

    pub fn mint_SPL_tokens(ctx: Context<MintSPLTokens>) -> ProgramResult {
        Ok(())

    }


    pub fn exchange_tokens_for_NFT(ctx: Context<ExchangeTokensForNFT>) -> ProgramResult {

    }
}

#[derive(Accounts)]
pub struct InitializeVault {
    #[account(init, payer = creator, has_one = )]
    pub vaultAccount: Account<'info, VaultAccount>
    pub mintFee: uint64,
    pub creator: Signer<'info>,
    #[account(init,
        token::mint = ownership_token_mint,
        token::authority = creator,
        seeds = [format!("{}{}{}{:?}", pool.token0Pubkey, pool.token1Pubkey, model.data, creator.signer_key()), (*"pool_tokenmint").to_string()],
        bump = pool_token_decimals,
        payer = creator)]
    pub vaultMint: Box<Account<'info, TokenAccount>>,
    pub NFTMint: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

}


#[account]
pub struct VaultAccount {
    pub mintFee: uint64,
    pub owner: uint64,
    pub vaultMint: AccountInfo<'info>,
    pub NFTMint: AccountInfo<'info>,

}

#[derive(Accounts)]
pub struct MintSPLTokens {
    pub mintFee: uint64,
    pub user: Signer<'info>,
    pub vault: AccountInfo<'info>,
    pub NFTMint: AccountInfo<'info>,
    
}

#[derive(Accounts)]
pub struct MintSPLTokens {
    pub mintFee: uint64,
    pub user: Signer<'info>,
    pub vault: AccountInfo<'info>,
    pub NFTMint: AccountInfo<'info>,
    
}
