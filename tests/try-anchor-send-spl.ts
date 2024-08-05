import * as anchor from "@coral-xyz/anchor";
import * as splToken from "@solana/spl-token";
import { Program } from "@coral-xyz/anchor";
import { TryAnchorSendSpl } from "../target/types/try_anchor_send_spl";

describe("try-anchor-send-spl", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TryAnchorSendSpl as Program<TryAnchorSendSpl>;

  const payer = (provider.wallet as any).payer;
  const signer = anchor.web3.Keypair.generate();
  const airdrop_hunter = anchor.web3.Keypair.generate();
  const airdroped_token = anchor.web3.Keypair.generate();

  let mint;
  let signer_token_account;
  let airdrop_account: anchor.web3.PublicKey;

  before(async() => {
    console.log('payer:', payer.publicKey.toBase58());
    console.log('signer:', signer.publicKey.toBase58());
    console.log('airdrop hunter:', airdrop_hunter.publicKey.toBase58());
    console.log('airdroped token:', airdroped_token.publicKey.toBase58());

    await provider.connection.requestAirdrop(airdroped_token.publicKey, 10000000000);
    await provider.connection.requestAirdrop(signer.publicKey, 10000000000);

    [airdrop_account] = anchor.web3.PublicKey.findProgramAddressSync(
      [], program.programId
    )

    mint = await splToken.createMint(
      provider.connection,
      payer,
      payer.publicKey,
      payer.publicKey,
      6
    );

    signer_token_account = await splToken.createAssociatedTokenAccount(
      provider.connection,
      payer,
      mint,
      signer.publicKey
    )
    await splToken.mintTo(
      provider.connection,
      payer,
      mint,
      signer_token_account,
      payer,
      1000000000
    )

    console.log('airdrop account:', airdrop_account.toBase58());
  })

  it('initialize airdrop account', async() => {
    const amount = new anchor.BN(10000000);
    const account = {
      signer: signer.publicKey,
      mint: mint,
      signerTokenAccount: signer_token_account,
      airdropAccount: airdrop_account,
      airdropedToken: airdroped_token.publicKey,
      tokenProgram: splToken.TOKEN_PROGRAM_ID,
    }

    const tx = await program.methods
      .initialize(amount)
      .accounts(account)
      .signers([airdroped_token, signer])
      .rpc()

    console.log('tx', tx)
    console.log('signer token balance', await provider.connection.getTokenAccountBalance(signer_token_account));
    console.log('airdroped account balance', await provider.connection.getTokenAccountBalance(airdroped_token.publicKey));
  })
})