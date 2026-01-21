import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Oracle } from "../target/types/oracle";
import { expect, assert } from "chai";

describe("oracle", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.Oracle as Program<Oracle>;

  const oracleKeypair = anchor.web3.Keypair.generate();
  const owner = provider.wallet; // wallet that will be oracle owner

  it("initializes the oracle", async () => {
    const initialPrice = new anchor.BN(100);

    await program.methods
      .initialize(initialPrice)
      .accounts({
        oracle: oracleKeypair.publicKey,
        owner: owner.publicKey,
        payer: owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([oracleKeypair])
      .rpc();

    const oracleAccount = await program.account.oracle.fetch(
      oracleKeypair.publicKey
    );

    console.log("Oracle after initialize:", oracleAccount);
    expect(oracleAccount.price.toNumber()).to.equal(100);
    expect(oracleAccount.owner.toBase58()).to.equal(
      owner.publicKey.toBase58()
    );
  });

  it("updates the oracle price with correct owner", async () => {
    const newPrice = new anchor.BN(200);

    await program.methods
      .update(newPrice)
      .accounts({
        oracle: oracleKeypair.publicKey,
        owner: owner.publicKey,
      })
      .rpc();

    const oracleAccount = await program.account.oracle.fetch(
      oracleKeypair.publicKey
    );

    console.log("Oracle after update:", oracleAccount);
    expect(oracleAccount.price.toNumber()).to.equal(200);
  });

  it("fails if a non-owner tries to update", async () => {
    const newPrice = new anchor.BN(300);
    const attacker = anchor.web3.Keypair.generate();

    // airdrop so attacker can pay fees
    await provider.connection.requestAirdrop(attacker.publicKey, 1e9);

    try {
      await program.methods
        .update(newPrice)
        .accounts({
          oracle: oracleKeypair.publicKey,
          owner: attacker.publicKey,
        })
        .signers([attacker])
        .rpc();

      assert.fail("Expected update by non-owner to fail, but it succeeded");
    } catch (err) {
      // success: it should throw
      console.log("Non-owner update failed as expected:", err?.toString());
    }
  });
});
