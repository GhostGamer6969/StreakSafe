import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StreakSafe } from "../target/types/streak_safe";
import { PublicKey } from "@solana/web3.js"
import { assert } from "chai";
import wallet from "/home/ghostgamer/.config/solana/ghost.json"

describe("streak-safe", () => {
  let provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const userC = anchor.web3.Keypair.fromSecretKey(new Uint8Array(wallet));

  const uuid = new anchor.BN(Math.floor(Math.random() * 10_000))
  const uuidB = new anchor.BN(Math.floor(Math.random() * 10_000))
  const uuidC = new anchor.BN(Math.floor(Math.random() * 10_000))

  const program = anchor.workspace.streakSafe as Program<StreakSafe>;

  const config = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  )[0]

  const streak = PublicKey.findProgramAddressSync(
    [
      Buffer.from("streak"),
      provider.wallet.publicKey.toBuffer(),
      uuid.toArrayLike(Buffer, "le", 8)
    ],
    program.programId
  )[0]

  const vault = PublicKey.findProgramAddressSync(
    [
      Buffer.from("vault"),
      streak.toBuffer()
    ],
    program.programId
  )[0]
  const latestCheckin = PublicKey.findProgramAddressSync(
    [
      Buffer.from("check_in"),
      streak.toBuffer()
    ],
    program.programId
  )[0]


  const streakB = PublicKey.findProgramAddressSync(
    [
      Buffer.from("streak"),
      provider.wallet.publicKey.toBuffer(),
      uuidB.toArrayLike(Buffer, "le", 8)
    ],
    program.programId
  )[0]

  const vaultB = PublicKey.findProgramAddressSync(
    [
      Buffer.from("vault"),
      streakB.toBuffer()
    ],
    program.programId
  )[0]
  const latestCheckinB = PublicKey.findProgramAddressSync(
    [
      Buffer.from("check_in"),
      streakB.toBuffer()
    ],
    program.programId
  )[0]

  const streakC = PublicKey.findProgramAddressSync(
    [
      Buffer.from("streak"),
      userC.publicKey.toBuffer(),
      uuidC.toArrayLike(Buffer, "le", 8)
    ],
    program.programId
  )[0]

  const vaultC = PublicKey.findProgramAddressSync(
    [
      Buffer.from("vault"),
      streakC.toBuffer()
    ],
    program.programId
  )[0]
  const latestCheckinC = PublicKey.findProgramAddressSync(
    [
      Buffer.from("check_in"),
      streakC.toBuffer()
    ],
    program.programId
  )[0]

  let [minStake, minCheckins, expirySec, minVotes] = [new anchor.BN(110000000), new anchor.BN(4), new anchor.BN(21600), 3]

  let image = "2wHuV5JSJJgns8u196mRfgjTjGHiQUr1e7J7VVLYXA4J"

  it("Config initialized!", async () => {
    const tx = await program.methods.initializeConfig(minStake, minCheckins, expirySec, minVotes)
      .accountsPartial({
        config: config,
        admin: provider.wallet.publicKey,
      }).rpc();

    const configAccount = await program.account.config.fetch(config);

    assert.ok(configAccount.minStake.eq(minStake));
    assert.ok(configAccount.minCheckins.eq(minCheckins));
    assert.ok(configAccount.expirySec.eq(expirySec));
    assert.equal(configAccount.minVotes, minVotes);
  });

  it("Config update!", async () => {
    const tx = await program.methods.updateConfig(null, new anchor.BN(2), null, 1)
      .accountsPartial({
        config: config,
        admin: provider.wallet.publicKey,
      }).rpc();
    const configAccount = await program.account.config.fetch(config);

    assert.ok(configAccount.minStake.eq(minStake));
    assert.ok(configAccount.minCheckins.eq(new anchor.BN(2)));
    assert.ok(configAccount.expirySec.eq(expirySec));
    assert.equal(configAccount.minVotes, 1);

  });

  it("fails Unauthorized Config Initialize!", async () => {
    try {
      const tx = await program.methods.initializeConfig(minStake, minCheckins, expirySec, minVotes)
        .accountsPartial({
          config: config,
          admin: userC.publicKey,
        }).rpc();
      assert.fail("Expected transaction to fail but it succeeded.");
    } catch (err) {
      assert.include(err.message, "Signature verification failed");
    }
  });


  it("fails Unauthorized Config update!", async () => {
    try {
      const tx = await program.methods.updateConfig(null, new anchor.BN(2), null, 1)
        .accountsPartial({
          config: config,
          admin: userC.publicKey,
        }).rpc();
      assert.fail("Expected transaction to fail but it succeeded.");
    } catch (err) {
      assert.include(err.message, "Signature verification failed");
    }

  });

  it("Initialize Streak A!", async () => {

    const tx = await program.methods.initializeStreak(uuid, 0, 3, new anchor.BN(120000000))
      .accountsPartial({
        user: provider.wallet.publicKey,
        streak: streak,
        vault: vault,
        config: config,
      }).rpc();
    const streakAccount = await program.account.streak.fetch(streak);
    const vaultAccount = await program.account.vault.fetch(vault);

    assert.equal(streakAccount.categories, 0);
    assert.equal(streakAccount.requiredCheckin, 3);
    assert.ok(vaultAccount.streakOwner.equals(provider.wallet.publicKey))
  });


  it("Initialize Streak B!", async () => {

    const tx = await program.methods.initializeStreak(uuidB, 0, 3, new anchor.BN(120000000))
      .accountsPartial({
        user: provider.wallet.publicKey,
        streak: streakB,
        vault: vaultB,
        config: config,
      }).rpc();
    const streakBAccount = await program.account.streak.fetch(streakB);
    const vaultBAccount = await program.account.vault.fetch(vaultB);

    assert.equal(streakBAccount.categories, 0);
    assert.equal(streakBAccount.requiredCheckin, 3);
    assert.ok(vaultBAccount.streakOwner.equals(provider.wallet.publicKey))
  });

  it("CheckinB setup for test!", async () => {
    const tx = await program.methods.checkInTest(uuidB, image)
      .accountsPartial({
        userB: provider.wallet.publicKey,
        slashReciever: provider.wallet.publicKey,
        streakB: streakB,
        vaultB: vaultB,
        latestCheckinB: latestCheckinB,
        config: config,
      }).rpc();
    const streakBAccount = await program.account.streak.fetch(streakB);
    const vaultBAccount = await program.account.vault.fetch(vaultB);
    const checkInBAccount = await program.account.latestCheckIn.fetch(latestCheckinB);

    assert.equal(streakBAccount.categories, 0);
    assert.equal(streakBAccount.requiredCheckin, 3);
    assert.ok(vaultBAccount.streakOwner.equals(provider.wallet.publicKey))
    assert.equal(checkInBAccount.image, image);

  });

  it("Initialize Streak C!", async () => {

    const tx = await program.methods.initializeStreak(uuidC, 1, 3, new anchor.BN(120000000))
      .accountsPartial({
        user: userC.publicKey,
        streak: streakC,
        vault: vaultC,
        config: config,
      }).signers([userC])
      .rpc();
    const streakCAccount = await program.account.streak.fetch(streakC);
    const vaultCAccount = await program.account.vault.fetch(vaultC);

    assert.equal(streakCAccount.categories, 1);
    assert.equal(streakCAccount.requiredCheckin, 3);
    assert.ok(vaultCAccount.streakOwner.equals(userC.publicKey))
  });

  it("fails to Unauthorized Checkin C setup for test!", async () => {
    try {
      const tx = await program.methods.checkInTest(uuidC, image)
        .accountsPartial({
          userB: userC.publicKey,
          slashReciever: provider.wallet.publicKey,
          streakB: streakC,
          vaultB: vaultC,
          latestCheckinB: latestCheckinC,
          config: config,
        }).signers([userC])
        .rpc();
      assert.fail("Expected transaction to fail but it succeeded.");
    } catch (err) {
      assert.equal(err.error.errorMessage, "A raw constraint was violated");
    }

  });



  it("Fails to Check in with different slash reciever!", async () => {
    try {
      const tx = await program.methods.checkIn(uuid, uuidB, image, true)
        .accountsPartial({
          user: provider.wallet.publicKey,
          userB: provider.wallet.publicKey,
          slashReciever: userC.publicKey,
          streak: streak,
          streakB: streakB,
          vault: vault,
          vaultB: vaultB,
          latestCheckin: latestCheckin,
          latestCheckinB: latestCheckinB,
          config: config,
        })
        .rpc();
      assert.fail("Expected transaction to fail but it succeeded.");
    } catch (err) {
      assert.equal(err.error.errorMessage, "A raw constraint was violated");
    }
  });

  it("Check in A!", async () => {
    const tx = await program.methods.checkIn(uuid, uuidB, image, true)
      .accountsPartial({
        user: provider.wallet.publicKey,
        userB: provider.wallet.publicKey,
        slashReciever: provider.wallet.publicKey,
        streak: streak,
        streakB: streakB,
        vault: vault,
        vaultB: vaultB,
        latestCheckin: latestCheckin,
        latestCheckinB: latestCheckinB,
        config: config,
      }).rpc();

    const streakAccount = await program.account.streak.fetch(streak);
    const vaultAccount = await program.account.vault.fetch(vault);
    const latestCheckinBAccount = await program.account.latestCheckIn.fetch(latestCheckinB);

    assert.equal(streakAccount.categories, 0);
    assert.equal(streakAccount.requiredCheckin, 3);
    assert.ok(vaultAccount.streakOwner.equals(provider.wallet.publicKey));
    assert.ok(latestCheckinBAccount.votes[0].eq(new anchor.BN(1)))
    assert.ok(latestCheckinBAccount.votes[1].eq(new anchor.BN(0)))
  });

  it("Fails to Check in before getting verified!", async () => {
    try {
      const tx = await program.methods.checkIn(uuid, uuidB, image, true)
        .accountsPartial({
          user: provider.wallet.publicKey,
          userB: provider.wallet.publicKey,
          slashReciever: provider.wallet.publicKey,
          streak: streak,
          streakB: streakB,
          vault: vault,
          vaultB: vaultB,
          latestCheckin: latestCheckin,
          latestCheckinB: latestCheckinB,
          config: config,
        }).rpc();
      assert.fail("Expected transaction to fail but it succeeded.");
    } catch (err) {
      const anchorErr = anchor.AnchorError.parse(err.logs);
      assert.equal(anchorErr.error.errorCode.code, "NotVerified");
      assert.equal(anchorErr.error.errorMessage, "Last check in is not verified");
    }
  });

  it("Fails to Check in for User C (not same category)!", async () => {
    try {
      const tx = await program.methods.checkIn(uuidC, uuid, image, true)
        .accountsPartial({
          user: userC.publicKey,
          userB: provider.wallet.publicKey,
          slashReciever: provider.wallet.publicKey,
          streak: streakC,
          streakB: streak,
          vault: vaultC,
          vaultB: vault,
          latestCheckin: latestCheckinC,
          latestCheckinB: latestCheckin,
          config: config,
        }).signers([userC])
        .rpc();

      assert.fail("Expected transaction to fail but it succeeded.");
    } catch (err) {
      const anchorErr = anchor.AnchorError.parse(err.logs);
      assert.equal(anchorErr.error.errorCode.code, "NotSameCategory");
      assert.equal(anchorErr.error.errorMessage, "The streak trying to verify is not of same category");
    }
  });

});
