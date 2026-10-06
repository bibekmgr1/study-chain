describe("StudyChain Test", () => {
  it("Creates, completes, and fails commitments", async () => {
    // ============ TEST 1: CREATE + COMPLETE ============
    const recordKp = new web3.Keypair();
    const futureDeadline = Math.floor(Date.now() / 1000) + 3600;

    await pg.program.methods
      .createCommitment("Finish Rust Enums", new anchor.BN(futureDeadline))
      .accounts({
        record: recordKp.publicKey,
        student: pg.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([recordKp])
      .rpc();

    console.log(" Created commitment:", recordKp.publicKey.toString());

    await pg.program.methods
      .completeCommitment()
      .accounts({
        record: recordKp.publicKey,
        student: pg.wallet.publicKey,
      })
      .rpc();

    console.log(" Completed commitment");

    // ============ TEST 2: FAIL AFTER DEADLINE ============
    const failKp = new web3.Keypair();
    const pastDeadline = Math.floor(Date.now() / 1000) - 3600;

    await pg.program.methods
      .createCommitment("Missed study goal", new anchor.BN(pastDeadline))
      .accounts({
        record: failKp.publicKey,
        student: pg.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([failKp])
      .rpc();

    console.log(" Created commitment with past deadline");

    await pg.program.methods
      .failCommitment()
      .accounts({
        record: failKp.publicKey,
        caller: pg.wallet.publicKey,
      })
      .rpc();

    console.log(" Failed commitment (permissionless)");
    console.log(" All tests passed");
  });
});
