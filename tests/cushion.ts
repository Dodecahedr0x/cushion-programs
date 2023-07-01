import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

import { Cushion } from "../target/types/cushion";
import { FEEDS } from "../sdk/src";
import { generateSeededKeypair } from "./utils";
import {
  getBandKey,
  getLlammaAuthorityKey,
  getLlammaKey,
  getMarketKey,
} from "../sdk/src/pdas";
import {
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintToChecked,
} from "@solana/spl-token";

const suiteName = "cushion";
describe(suiteName, () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.Cushion as Program<Cushion>;
  let users: Keypair[];
  let collateralMintKeypair = generateSeededKeypair(`${suiteName}+collateral`);

  before(async () => {
    users = await Promise.all(
      Array(3)
        .fill(0)
        .map(async (_, i) => {
          const kp = generateSeededKeypair(`${suiteName}+user${i}`);

          await connection.confirmTransaction(
            await connection.requestAirdrop(kp.publicKey, 10 * LAMPORTS_PER_SOL)
          );

          return kp;
        })
    );

    // Mint the collateral to the lender
    await createMint(
      connection,
      users[1],
      users[1].publicKey,
      users[1].publicKey,
      6,
      collateralMintKeypair
    );
    const lendersCollateralAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      users[1],
      collateralMintKeypair.publicKey,
      users[1].publicKey,
      true
    );
    await mintToChecked(
      connection,
      users[1],
      collateralMintKeypair.publicKey,
      lendersCollateralAccount.address,
      users[1],
      LAMPORTS_PER_SOL,
      6
    );
  });

  it("Borrow from a market", async () => {
    const admin = users[0];
    const lender = users[1];
    const borrower = users[2];

    const debtMintKeypair = generateSeededKeypair(`${suiteName}+stablecoin`);
    const llammaAuthorityKey = getLlammaAuthorityKey(debtMintKeypair.publicKey);
    const llammaKey = getLlammaKey(debtMintKeypair.publicKey);
    await program.methods
      .initializeLlamma()
      .accounts({
        admin: admin.publicKey,
        llamma: llammaKey,
        llammaAuthority: llammaAuthorityKey,
        debtMint: debtMintKeypair.publicKey,
      })
      .signers([debtMintKeypair])
      .rpc({ skipPreflight: true });

    const marketKey = getMarketKey(llammaKey, collateralMintKeypair.publicKey);
    await program.methods
      .createMarket()
      .accounts({
        admin: admin.publicKey,
        llamma: llammaKey,
        llammaAuthority: llammaAuthorityKey,
        market: marketKey,
        debtMint: debtMintKeypair.publicKey,
        collateralMint: collateralMintKeypair.publicKey,
        debtAccount: getAssociatedTokenAddressSync(
          debtMintKeypair.publicKey,
          llammaAuthorityKey,
          true
        ),
        collateralAccount: getAssociatedTokenAddressSync(
          collateralMintKeypair.publicKey,
          llammaAuthorityKey,
          true
        ),
        priceFeed: FEEDS.SOLUSD,
      })
      .signers([admin])
      .rpc({ skipPreflight: true });

    const bandIndex = 3;
    const bandKey = getBandKey(marketKey, bandIndex);
    await program.methods
      .createBand(bandIndex)
      .accounts({
        llamma: llammaKey,
        llammaAuthority: llammaAuthorityKey,
        market: marketKey,
        band: bandKey,
      })
      .rpc({ skipPreflight: true });
  });
});
