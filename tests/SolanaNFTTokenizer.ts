import * as anchor from '@project-serum/anchor';
import { Program  } from '@project-serum/anchor';
import { SolanaNftTokenizer } from '../target/types/solana_nft_tokenizer';

describe('SolanaNFTTokenizer', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolanaNftTokenizer as Program<SolanaNftTokenizer>;
  const myAccount = anchor.web3.Keypair.generate();

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initializeVault({
      accounts: { 
        systemProgram: program.programId,
      },
      signers: [myAccount],
    });
    console.log("Your transaction signature", tx);
  });
});
