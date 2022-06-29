import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftGarbageCollector } from "../target/types/nft_garbage_collector";

describe("nft-garbage-collector", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NftGarbageCollector as Program<NftGarbageCollector>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
