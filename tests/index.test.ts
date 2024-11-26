// No imports needed: web3, anchor, pg and more are globally available

describe("Test", () => {
  it("initialize_escrow", async () => {
    const client = pg.wallets.wallet1

    const escort = pg.wallets.wallet2

   // Air-drop SOL to client for funding the escrow
   //await pg.connection.requestAirdrop(client.publicKey, 2 * web3.LAMPORTS_PER_SOL);

   const bookingId = "booking123"; // Test booking ID
   const verifyCode = "123456";   // Test verification code
   const escrowAmount = 0.25 * web3.LAMPORTS_PER_SOL;

    // Send transaction
    const [escrowPda, escrowBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), Buffer.from(bookingId), escort.publicKey.toBuffer()],
      pg.PROGRAM_ID
    );

    console.log("Initializing Escrow...");

    const txHash = await pg.program.methods.initializeEscrow(bookingId, verifyCode, new anchor.BN(escrowAmount))
    .accounts({
      escrowAccount: escrowPda,
      client: client.publicKey,
      escort: escort.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([client.keypair])
    .rpc()

    console.log('txHash', txHash)

  // Fetch and verify escrow account
  const escrowAccount = await pg.program.account.escrowAccount.fetch(escrowPda);
  //assert.equal(escrowAccount.client.toBase58(), client.publicKey.toBase58(), "Client mismatch");
  //assert.equal(escrowAccount.escort.toBase58(), escort.publicKey.toBase58(), "Escort mismatch");
  assert.equal(escrowAccount.bookingId, bookingId, "Booking ID mismatch");
  assert.equal(escrowAccount.verifyCode, verifyCode, "Verification code mismatch");

  console.log("Escrow Initialized Successfully!");

      
  });

  it("release_escrow", async () => {
    //const client = pg.wallets.wallet1

    const escort = pg.wallets.wallet2
    const treasury = pg.wallets.tresurry

    const bookingId = "booking123"; // Test booking ID
    const verifyCode = "123456";   // Test verification code

    // Send transaction
    const [escrowPda, escrowBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), Buffer.from(bookingId), escort.publicKey.toBuffer()],
      pg.PROGRAM_ID
    );

    console.log("Initializing Escrow...");

    const txHash = await pg.program.methods.releaseEscrow(verifyCode)
    .accounts({
      escrowAccount: escrowPda,
      signer: escort.publicKey,
      treasury:  treasury.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([escort.keypair])
    .rpc()

    console.log('txHash', txHash)

    // Verify escrow account is closed
    try {
      await pg.program.account.escrowAccount.fetch(escrowPda);
      assert.fail("Escrow account should be closed");
    } catch (e) {
      console.log("Escrow account successfully closed!");
    }

    // Verify funds were transferred to the escort
    const escortBalance = await pg.connection.getBalance(escort.publicKey);
    assert(escortBalance > 0, "Escort did not receive funds");

    console.log("Funds Released Successfully!");
  })
});
