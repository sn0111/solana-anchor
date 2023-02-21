import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorHelloWorld } from "../target/types/anchor_hello_world";

describe("anchor-hello-world", () => {
  let provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorHelloWorld as Program<AnchorHelloWorld>;

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
