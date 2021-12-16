import * as anchor from '@project-serum/anchor';
import { Program, BN  } from '@project-serum/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
const {Keypair, Transaction, systemProgram,SYSVAR_RENT_PUBKEY } = require("@solana/web3.js");
const {TOKEN_PROGRAM_ID }  = require('@solana/spl-token')
import { SolanaNftTokenizer } from '../target/types/solana_nft_tokenizer';

describe('SolanaNFTTokenizer', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolanaNftTokenizer as Program<SolanaNftTokenizer>;
  const myAccount = anchor.web3.Keypair.generate();

  it('Is initialized!', async () => {

    const vault_account = anchor.web3.Keypair.generate();
    const [mint, mintBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("mint")],
      program.programId
    );
    console.log(TOKEN_PROGRAM_ID);
    
    // Add your test here.
    const tx = await program.rpc.initializeVault(myAccount.publicKey, "random vault",[],"RAND", new BN(1),mintBump, {
      accounts: { 
        vaultAccount: vault_account.publicKey,
        vaultCreator: myAccount.publicKey,
        vaultMint: mint,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [myAccount, vault_account],
    });
    console.log("Your transaction signature", tx);
  });
});
