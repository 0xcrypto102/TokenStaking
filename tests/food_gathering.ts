import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FoodGathering } from "../target/types/food_gathering";
import { TOKEN_PROGRAM_ID, createAccount, createAssociatedTokenAccount, getAssociatedTokenAddress , ASSOCIATED_TOKEN_PROGRAM_ID,createMint, mintTo, mintToChecked, getAccount, getMint, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { SystemProgram, Keypair, PublicKey, Transaction } from "@solana/web3.js";

describe("food_gathering", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.FoodGathering as Program<FoodGathering>;
  let connection = new anchor.web3.Connection("https://api.devnet.solana.com", "confirmed");
  let antCoin = new PublicKey("FLitGKEPBvBNqPVZbfgRPR5fwcsgSrRv6BDZjxRRFhUC"); 
  let antFood = new PublicKey("4JtesASQCh1ZYDdvCpgpMG5WLxMKyGAVt4tS4QS9L8Np"); 
  //  2TYV72CtgYXCduE5hheeoux728zHcSyxPQAbhiCNf2Yy
  let owner = Keypair.fromSecretKey(
    Uint8Array.from([113,63,93,213,68,178,22,189,136,49,33,174,196,213,238,242,164,106,9,180,15,3,238,80,159,127,118,18,231,206,240,93,21,168,99,61,85,242,222,187,12,44,91,158,122,83,103,113,125,136,28,83,108,248,78,219,197,250,38,187,70,109,130,194])
  );
  
  const [globalState, globalStateBump] = await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("GLOBAL-STATE-SEED"),
      owner.publicKey.toBuffer()
    ],
    program.programId
  );

  const [vault, vaultBump] = await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("VAULT-SEED")
    ],
    program.programId
  );
  
  const [minter, minterBump] = await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("MINTER-STATE-SEED"),
      owner.publicKey.toBuffer()
    ],
    program.programId
  );

  const [antFoodTokenVaultAccount, antFoodTokenVaultAccountBump] = await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("TOKEN-VAULT-SEED"),
      antFood.toBuffer()
    ],
    program.programId
  );

  const rentSysvar = anchor.web3.SYSVAR_RENT_PUBKEY;

  it("Is initialized!", async () => {
    // Add your test here.
    const antc_price = 735;
    const antc_expo = 10000;
    const tx = await program.rpc.initialize(
      owner.publicKey,
      new anchor.BN(antc_price),
      new anchor.BN(antc_expo),
      {
        accounts: {
          owner: owner.publicKey,
          globalState,
          vault,
          minter,
          systemProgram: SystemProgram.programId,
          rent: rentSysvar
        },
        signers: [owner]
      }
    )
    console.log("Your transaction signature", tx);
  });

  it("set antc", async () => {
    const tx = await program.rpc.setAntCoin({
      accounts: {
        minterKey: owner.publicKey,
        globalState,
        minter,
        antCoin,
        systemProgram: SystemProgram.programId,
        rent: rentSysvar
      },
      signers:[owner]
    });
  });


  it("set antFood", async () => {
    const tx = await program.rpc.setAntFoodToken({
      accounts: {
        minterKey: owner.publicKey,
        globalState,
        minter,
        newAntFoodToken: antFood,
        systemProgram: SystemProgram.programId,
        rent: rentSysvar
      },
      signers:[owner]
    });
  });
  
  it("Deposit antFood", async () => {
    const minterAntFoodTokenAccount = await getAssociatedTokenAddress(
      antFood,
      owner.publicKey
    );

    const tx = await program.rpc.depositAntFoodToken(
      new anchor.BN(1000000000),
      {
      accounts: {
        minterKey: owner.publicKey,
        globalState,
        minter,
        antFoodToken: antFood,
        antFoodTokenVaultAccount,
        minterAntFoodTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: rentSysvar
      },
      signers:[owner]
    });
    console.log(tx);
  });

});
