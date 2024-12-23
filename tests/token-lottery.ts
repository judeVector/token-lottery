import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenLottery } from "../target/types/token_lottery";

describe("token-lottery", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.TokenLottery as Program<TokenLottery>;

  it("Should Initialized Config", async () => {
    const initConfigTx = await program.methods
      .initializeConfig(new anchor.BN(0), new anchor.BN(1834954927), new anchor.BN(10_000))
      .instruction();

    const blockhashWithContext = await provider.connection.getLatestBlockhash();

    const tx = new anchor.web3.Transaction({
      feePayer: provider.wallet.publicKey,
      blockhash: blockhashWithContext.blockhash,
      lastValidBlockHeight: blockhashWithContext.lastValidBlockHeight,
    }).add(initConfigTx);

    const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [
      wallet.payer,
    ]);
    console.log("Your transaction signature", signature);
  });
});
