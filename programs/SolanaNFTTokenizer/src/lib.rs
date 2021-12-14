use anchor_lang::prelude::*;

use anchor_spl::token::{self, Burn, MintTo, SetAuthority, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_nft_tokenizer {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, owner: PubKey) -> ProgramResult {
        let vault_account = &mut ctx.accounts.vault_account;
        vault_account.mint_fee = ctx.mint_fee;
        vault_account.owner = owner;
        vault_account.NFT_mint = ctx.NFT_mint;
        vault_account.vault_mint = ctx.vault_mint.to_account_info;
        vault_account.
        Ok(())
    }

    pub fn mint_SPL_tokens(ctx: Context<MintSPLTokens>) -> ProgramResult {
        Ok(())

    }


    pub fn exchange_tokens_for_NFT(ctx: Context<ExchangeTokensForNFT>) -> ProgramResult {
        Ok(())

    }

    pub fn collect_fees(ctx: Context<CollectFees>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault {
    #[account(init, payer = creator)]
    pub vault_account: Account<'info, VaultAccount>
    pub mint_fee: uint64,
    pub creator: Signer<'info>,
    #[account(init,
        token::mint = ownership_token_mint,
        token::authority = creator,
        seeds = [format!("{}{}{}{:?}", pool.token0Pubkey, pool.token1Pubkey, model.data, creator.signer_key()), (*"pool_tokenmint").to_string()],
        bump = pool_token_decimals,
        payer = creator)]
    pub vault_mint: Box<Account<'info, TokenAccount>>,
    pub NFT_mint: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

}


#[account]
pub struct VaultAccount {
    pub mint_fee: uint64,
    pub owner: PubKey,
    pub vault_mint: AccountInfo<'info>,
    pub NFT_mint: AccountInfo<'info>,

}

#[derive(Accounts)]
pub struct MintSPLTokens {
    pub user: Signer<'info>,
    pub vault: AccountInfo<'info>,
    pub NFT_mint: AccountInfo<'info>,
    
}

#[derive(Accounts)]
pub struct CollectFees {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, VaultAccount>,
    pub owner: Signer<'info>
}
