import * as anchor from "@coral-xyz/anchor";
import * as sb from "@switchboard-xyz/on-demand";

import { Program } from "@coral-xyz/anchor";
import { TokenLottery } from "../target/types/token_lottery";
import SwitchboardIDL from "../switchboard.json";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("token-lottery", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const switchboardProgram = new anchor.Program(SwitchboardIDL as anchor.Idl, provider);
  const randomnessKeypair = anchor.web3.Keypair.generate();

  // before("Load switchboard program", async () => {
  //   const switchBoardIDL = await anchor.Program.fetchIdl(sb.ON_DEMAND_MAINNET_PID, {
  //     connection: new anchor.web3.Connection("https://api.mainnet-beta.solana.com"),
  //   });

  //   var fs = require("fs");
  //   fs.writeFile("switchboard.json", JSON.stringify(switchBoardIDL, null, 2), function (err: any) {
  //     if (err) throw err;
  //     console.log("The file has been saved!");
  //   });

  //   switchboardProgram = new anchor.Program(switchBoardIDL, provider);
  // });

  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.TokenLottery as Program<TokenLottery>;

  async function buyTicket() {
    const buyTicketTx = await program.methods
      .buyTicket()
      .accounts({
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .instruction();

    const computeTx = anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
      units: 600000,
    });

    const priorityTx = anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
      microLamports: 1,
    });

    const blockhashWithContext = await provider.connection.getLatestBlockhash();
    const tx = new anchor.web3.Transaction({
      feePayer: provider.wallet.publicKey,
      blockhash: blockhashWithContext.blockhash,
      lastValidBlockHeight: blockhashWithContext.lastValidBlockHeight,
    })
      .add(buyTicketTx)
      .add(computeTx)
      .add(priorityTx);

    const signature = await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      tx,
      [wallet.payer],
      { skipPreflight: true }
    );
    3;

    console.log("Buy Ticket signature", signature);
  }

  it("Should Initialized Config", async () => {
    const slot = await provider.connection.getSlot();
    const endSlot = slot + 20;

    const initConfigTx = await program.methods
      .initializeConfig(new anchor.BN(slot), new anchor.BN(endSlot), new anchor.BN(10_000))
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

  it("Should Initialize Lottery", async () => {
    const initLotteryTx = await program.methods
      .initializeLottery()
      .accounts({
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .instruction();

    const blockhashWithContext = await provider.connection.getLatestBlockhash();

    const tx = new anchor.web3.Transaction({
      feePayer: provider.wallet.publicKey,
      blockhash: blockhashWithContext.blockhash,
      lastValidBlockHeight: blockhashWithContext.lastValidBlockHeight,
    }).add(initLotteryTx);

    const signature = await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      tx,
      [wallet.payer]
      // { skipPreflight: true }
    );
    console.log("Your initLottery transaction signature", signature);
  });

  it("Should buy ticket", async () => {
    await buyTicket();
  });

  it("Should commit randomness", async () => {
    const queue = new anchor.web3.PublicKey("A43DyUGA7s8eXPxqEjJY6EBu1KKbNgfxF8h17VAHn13w");

    const queueAccount = new sb.Queue(switchboardProgram, queue);

    try {
      await queueAccount.loadData();
    } catch (error) {
      console.log("Error: " + error);
      process.exit(1);
    }

    const [randomness, createRandomnessInstruction] = await sb.Randomness.create(
      switchboardProgram,
      randomnessKeypair,
      queue
    );

    const createRandomnessTx = await sb.asV0Tx({
      connection: provider.connection,
      ixs: [createRandomnessInstruction],
      payer: wallet.publicKey,
      signers: [wallet.payer, randomnessKeypair],
    });

    const createRandomnessSignature = await provider.connection.sendTransaction(createRandomnessTx);
    console.log("Your create randomness transaction signature", createRandomnessSignature);

    const commitInstruction = await program.methods
      .commitRandomness()
      .accounts({
        randomnessAccount: randomness.pubkey,
      })
      .instruction();

    const commitComputeInstruction = anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
      units: 100000,
    });

    const commitPriorityInstruction = anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
      microLamports: 1,
    });

    const sbcommitInstruction = await randomness.commitIx(queue);

    const commitBlockhashWithContext = await provider.connection.getLatestBlockhash();
    const commitTx = new anchor.web3.Transaction({
      feePayer: provider.wallet.publicKey,
      blockhash: commitBlockhashWithContext.blockhash,
      lastValidBlockHeight: commitBlockhashWithContext.lastValidBlockHeight,
    })
      .add(commitComputeInstruction)
      .add(commitPriorityInstruction)
      .add(sbcommitInstruction)
      .add(commitInstruction);

    const commitSignature = await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      commitTx,
      [wallet.payer]
      // { skipPreflight: true }
    );

    console.log("commit signature transaction", commitSignature);
  });

  it("Should reveal winning ticket", async () => {
    const slot = await provider.connection.getSlot();
    const endSlot = slot + 20;

    const queue = new anchor.web3.PublicKey("A43DyUGA7s8eXPxqEjJY6EBu1KKbNgfxF8h17VAHn13w");

    const [randomness, createRandomnessInstruction] = await sb.Randomness.create(
      switchboardProgram,
      randomnessKeypair,
      queue
    );

    // From Here
    const sbRevealInstruction = await randomness.revealIx();

    const revealWinnerInstruction = await program.methods
      .revealWinningTicket()
      .accounts({
        randomnessAccount: randomness.pubkey,
      })
      .instruction();

    const revealBlockhashWithContext = await provider.connection.getLatestBlockhash();

    const revealTx = new anchor.web3.Transaction({
      feePayer: provider.wallet.publicKey,
      blockhash: revealBlockhashWithContext.blockhash,
      lastValidBlockHeight: revealBlockhashWithContext.lastValidBlockHeight,
    })
      .add(sbRevealInstruction)
      .add(revealWinnerInstruction);

    let currentSlot = 0;
    while (currentSlot < endSlot) {
      const slot = await provider.connection.getSlot();
      if (slot > currentSlot) {
        currentSlot = slot;
        console.log("Current slot: " + currentSlot);
      }
    }

    const revealSignature = await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      revealTx,
      [wallet.payer],
      { skipPreflight: true }
    );

    console.log("Reveal signature: " + revealSignature);
  });
});
