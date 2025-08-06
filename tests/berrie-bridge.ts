import * as anchor from "@coral-xyz/anchor";
import { IdlTypes, Program } from "@coral-xyz/anchor";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { Keypair, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { config } from "dotenv";
import IDL from "../target/idl/berrie_bridge.json";
import { BerrieBridge } from "../target/types/berrie_bridge";

config({ path: "./tests/.env" });

const logTxnSignature = (tx: string) => {
  console.log(
    "Your transaction signature",
    `https://explorer.solana.com/tx/${tx}?cluster=devnet`
  );
};

type Lock = IdlTypes<BerrieBridge>["lock"];

const ADMIN_PUBKEY = new PublicKey(
  IDL.constants.find((c) => c.name === "ADMIN_PUBKEY")!.value
);

const getPostTokenAddress = async (
  program: Program<BerrieBridge>,
  preTokenMint: PublicKey
) => {
  const LOCK_TOKEN_SEED = "lock_token";

  const [postTokenMint] = PublicKey.findProgramAddressSync(
    [Buffer.from(LOCK_TOKEN_SEED), preTokenMint.toBuffer()],
    program.programId
  );
  return postTokenMint;
};

const getLockAddress = (
  program: Program<BerrieBridge>,
  preTokenMint: PublicKey
) => {
  const LOCK_SEED = "lock";

  const [lockPublicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from(LOCK_SEED), preTokenMint.toBuffer()],
    program.programId
  );
  return lockPublicKey;
};

const getLockInfos = async (program: Program<BerrieBridge>) => {
  const lockInfos = await program.account.lock.all();

  return lockInfos;
};

describe("berrie-bridge", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BerrieBridge as Program<BerrieBridge>;

  const adminKeypair = Keypair.fromSecretKey(
    bs58.decode(process.env.ADMIN_PRIVATE_KEY!)
  );
  const admin = adminKeypair.publicKey;

  const userKeypair = Keypair.fromSecretKey(
    bs58.decode(process.env.USER_PRIVATE_KEY!)
  );
  const user = userKeypair.publicKey;

  /**---------------- ADMIN INSTRUCTIONS ----------------*/

  it.skip("Enable Token", async () => {
    const preTokenMint = new PublicKey(
      "mntAk89WGn1YacVFxzU84tVbn3zFYf1LVXMxMjhpTjC"
    );
    const accountInfo = await program.provider.connection.getAccountInfo(
      preTokenMint
    );

    const preTokenProgram = accountInfo.owner;

    const args = {
      name: "Berrie Token – berr.ie",
      symbol: "BERRIE",
      uri: "https://gateway.pinata.cloud/ipfs/QmNb6Pcyhy63Cq21aQADmDwGhf7vq822xF23uBQ6gJJjv3",
    };

    const tx = await program.methods
      .enableToken(args)
      .accounts({
        preTokenMint,
        preTokenProgram,
      })
      .rpc();

    logTxnSignature(tx);
  });

  /**---------------- USER INSTRUCTIONS ----------------*/

  it.skip("Lock Token", async () => {
    const preTokenMint = new PublicKey(
      "mntAk89WGn1YacVFxzU84tVbn3zFYf1LVXMxMjhpTjC"
    );
    const accountInfo = await program.provider.connection.getAccountInfo(
      preTokenMint
    );

    const preTokenProgram = accountInfo.owner;
    const amount = new BN(500_000_000);

    const tx = await program.methods
      .lockToken(amount)
      .accounts({
        user,
        preTokenMint,
        preTokenProgram,
      })
      .signers([userKeypair])
      .rpc();

    logTxnSignature(tx);
  });

  it.skip("Unlock Token", async () => {
    const preTokenMint = new PublicKey(
      "mntAk89WGn1YacVFxzU84tVbn3zFYf1LVXMxMjhpTjC"
    );
    const accountInfo = await program.provider.connection.getAccountInfo(
      preTokenMint
    );

    const preTokenProgram = accountInfo.owner;
    const amount = new BN(100_000_000);

    const tx = await program.methods
      .unlockToken(amount)
      .accounts({
        user,
        preTokenMint,
        preTokenProgram,
      })
      .signers([userKeypair])
      .rpc();

    logTxnSignature(tx);
  });
});
