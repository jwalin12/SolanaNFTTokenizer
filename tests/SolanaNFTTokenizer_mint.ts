import * as anchor from '@project-serum/anchor';
import { Program, BN  } from '@project-serum/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
const {Keypair, Transaction, systemProgram,SYSVAR_RENT_PUBKEY, LAMPORTS_PER_SOL, Connection, clusterApiUrl } = require("@solana/web3.js");
const {TOKEN_PROGRAM_ID }  = require('@solana/spl-token')
import { SolanaNftTokenizer } from '../target/types/solana_nft_tokenizer';

describe('SolanaNFTTokenizer', async () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());
  let connection = new Connection('devnet', 'confirmed');




    it ('accepts NFTS into the vault and mints tokens', async () => {

  const program = anchor.workspace.SolanaNftTokenizer as Program<SolanaNftTokenizer>;
  const myAccount = anchor.web3.Keypair.generate();

  const vault_account = anchor.web3.Keypair.generate();
    const [mint, mintBump] = await anchor.web3.PublicKey.findProgramAddress(
  [Buffer.from("mint"), vault_account.publicKey.toBuffer()],
      program.programId
    );
    const token_account = anchor.web3.Keypair.generate();

    let airdropSignature = await connection.requestAirdrop(
      myAccount.publicKey,
      LAMPORTS_PER_SOL,
  );

    

    // Add your test here.
    let tx = await program.rpc.initializeVault(mintBump,"random vault",[],"RAND",new BN(1), {
      accounts: { 
        vaultAccount: vault_account.publicKey,
        authority: myAccount.publicKey,
        vaultMint: mint,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [myAccount, vault_account],
    });
      airdropSignature = await connection.requestAirdrop(
        myAccount.publicKey,
        LAMPORTS_PER_SOL,
    );



    tx = await program.rpc.mintSplTokens({
      accounts: {
        vaultAccount: vault_account.publicKey,
        vaultMint: mint,
        user: myAccount.publicKey,
        vaultAuthority: myAccount.publicKey,
        nftMetadataAccount: 'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
        nftAccount: token_account.publicKey,
        nftMint: 'WECM8dWpzTHtVsQbqnNJXrm821iLSVWxf52grgDosdE',
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
    },
    signers: [myAccount, vault_account],
  });


  console.log("Your transaction signature", tx);




    });
  
    it('rejects NFTs with the wrong symbol', () => {
  
    });
  
    it('rejects the wrong metadata PDA', () => {
  
    });
  
    it('rejects NFTs with the wrong mint', () => {
  
    });
  
    it('rejects the wrong vailt mint', () => {
  
    });





});
