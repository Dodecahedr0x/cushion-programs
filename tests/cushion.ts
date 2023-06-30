import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Cushion } from "../target/types/cushion";
import { FEEDS } from "../sdk/src";

describe("cushion", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Cushion as Program<Cushion>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({ priceFeed: FEEDS.SOLUSD })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
