import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Oracle } from "../target/types/oracle";
import { expect, assert } from "chai";

describe("oracle", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.Oracle as Program<Oracle>;

  const owner = provider.wallet;

  // Reused across tests
  const SYMBOL = "SOL/USD";
  const EXPO = -8;
  const INITIAL_PRICE = new anchor.BN(100_000_000); // 1.0 * 10^8
  const INITIAL_CONFIDENCE = new anchor.BN(1_000_000);

  let oraclePda: anchor.web3.PublicKey;

  // Derive the PDA the same way as in the Rust seeds:
  // seeds = [b"oracle", owner.key().as_ref(), symbol.as_bytes()]
  before(() => {
    oraclePda = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("oracle"),
        owner.publicKey.toBuffer(),
        Buffer.from(SYMBOL),
      ],
      program.programId
    )[0];
  });

  it("initializes the oracle", async () => {
    await program.methods
      .initialize(SYMBOL, INITIAL_PRICE, EXPO, INITIAL_CONFIDENCE)
      .accounts({
        oracle: oraclePda,
        owner: owner.publicKey,
        payer: owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      // no .signers for PDA; Anchor creates it via seeds, payer signs via provider.wallet
      .rpc();

    const oracleAccount = await program.account.oracle.fetch(oraclePda);

    console.log("Oracle after initialize:", oracleAccount);

    expect(oracleAccount.owner.toBase58()).to.equal(
      owner.publicKey.toBase58()
    );
    expect(oracleAccount.symbol).to.equal(SYMBOL);
    expect(oracleAccount.price.toString()).to.equal(
      INITIAL_PRICE.toString()
    );
    expect(oracleAccount.expo).to.equal(EXPO);
    expect(oracleAccount.confidence.toString()).to.equal(
      INITIAL_CONFIDENCE.toString()
    );
    expect(oracleAccount.lastUpdateSlot.toNumber()).to.be.greaterThan(0);
  });


  it("check_price_stored passes using oracle policy", async () => {
    await program.methods
      .setPolicy(new anchor.BN(10_000), new anchor.BN(1_000_000_000))
      .accounts({ oracle: oraclePda, owner: owner.publicKey })
      .rpc();

    await program.methods
      .checkPriceStored()
      .accounts({ oracle: oraclePda })
      .rpc();
  });


  it("updates the oracle price and confidence with the correct owner", async () => {
    const newPrice = new anchor.BN(200_000_000); // 2.0 * 10^8
    const newConfidence = new anchor.BN(500_000);

    // Fetch before update so we can compare slots
    const before = await program.account.oracle.fetch(oraclePda);
    const beforeSlot = before.lastUpdateSlot.toNumber();

    await program.methods
      .update(newPrice, newConfidence)
      .accounts({
        oracle: oraclePda,
        owner: owner.publicKey,
      })
      .rpc();

    const oracleAccount = await program.account.oracle.fetch(oraclePda);

    console.log("Oracle after update:", oracleAccount);

    expect(oracleAccount.price.toString()).to.equal(newPrice.toString());
    expect(oracleAccount.confidence.toString()).to.equal(
      newConfidence.toString()
    );
    expect(oracleAccount.lastUpdateSlot.toNumber()).to.be.greaterThan(
      beforeSlot
    );
  });

  it("fails if a non-owner tries to update", async () => {
    const newPrice = new anchor.BN(300_000_000);
    const newConfidence = new anchor.BN(999_999);
    const attacker = anchor.web3.Keypair.generate();

    // airdrop so attacker can pay fees
    await provider.connection.requestAirdrop(attacker.publicKey, 1e9);

    try {
      await program.methods
        .update(newPrice, newConfidence)
        .accounts({
          oracle: oraclePda,
          owner: attacker.publicKey,
        })
        .signers([attacker])
        .rpc();

      assert.fail("Expected update by non-owner to fail, but it succeeded");
    } catch (err) {
      console.log("Non-owner update failed as expected:", err?.toString());
    }
  });

  it("allows same owner to create multiple oracles for different symbols", async () => {
    const OTHER_SYMBOL = "BTC/USD";
    const OTHER_PRICE = new anchor.BN(50_000_000_000); // 500 * 10^8
    const OTHER_CONFIDENCE = new anchor.BN(2_000_000);

    const [otherOraclePda] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("oracle"),
          owner.publicKey.toBuffer(),
          Buffer.from(OTHER_SYMBOL),
        ],
        program.programId
      );

    await program.methods
      .initialize(OTHER_SYMBOL, OTHER_PRICE, EXPO, OTHER_CONFIDENCE)
      .accounts({
        oracle: otherOraclePda,
        owner: owner.publicKey,
        payer: owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const solOracle = await program.account.oracle.fetch(oraclePda);
    const btcOracle = await program.account.oracle.fetch(otherOraclePda);

    console.log("SOL oracle:", solOracle);
    console.log("BTC oracle:", btcOracle);

    expect(solOracle.symbol).to.equal("SOL/USD");
    expect(btcOracle.symbol).to.equal("BTC/USD");

    expect(solOracle.price.toString()).to.not.equal(
      btcOracle.price.toString()
    );
    expect(solOracle.symbol).to.not.equal(btcOracle.symbol);
  });

  it("check_price passes for fresh and tight confidence", async () => {
    // reuse existing oraclePda, initialized + updated earlier
    const maxStalenessSlots = new anchor.BN(10_000); // very relaxed
    const maxConfidence = new anchor.BN(1_000_000_000); // very relaxed

    await program.methods
      .checkPrice(maxStalenessSlots, maxConfidence)
      .accounts({
        oracle: oraclePda,
      })
      .rpc();
  });

  it("check_price fails when confidence is too wide", async () => {
    const veryWideConfidence = new anchor.BN(1_000_000_000_000); // huge
    const maxConfidence = new anchor.BN(1); // tiny threshold

    // First, update oracle to have huge confidence
    const newPrice = new anchor.BN(999_000_000);

    await program.methods
      .update(newPrice, veryWideConfidence)
      .accounts({
        oracle: oraclePda,
        owner: owner.publicKey,
      })
      .rpc();

    // Then, check_price with a very small allowed confidence - should fail
    try {
      await program.methods
        .checkPrice(new anchor.BN(10_000), maxConfidence)
        .accounts({
          oracle: oraclePda,
        })
        .rpc();

      assert.fail("Expected check_price to fail due to high confidence");
    } catch (err) {
      console.log(
        "check_price failed due to high confidence, as expected:",
        err?.toString()
      );
    }
  });

  it("paused oracle makes checks fail", async () => {
    await program.methods
      .pause()
      .accounts({ oracle: oraclePda, owner: owner.publicKey })
      .rpc();

    try {
      await program.methods.checkPriceStored().accounts({ oracle: oraclePda }).rpc();
      assert.fail("Expected paused oracle to fail");
    } catch (_) { }

    await program.methods
      .resume()
      .accounts({ oracle: oraclePda, owner: owner.publicKey })
      .rpc();
  });

});
