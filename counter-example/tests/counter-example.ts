import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { CounterExample } from "../target/types/counter_example";

describe("counter-example", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CounterExample as Program<CounterExample>;
  let counter = anchor.web3.Keypair.generate();
  let counter1 = anchor.web3.Keypair.generate();


  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({counter :counter.publicKey})
      .signers([counter])
      .rpc()
    // const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.counter.fetch(counter.publicKey)
    expect(account.count.toNumber() === 0)
  });

  it("Counter Increase",async ()=>{
    const tx = await program.methods
      .increment()
      .accounts({counter:counter.publicKey,user:provider.wallet.publicKey})
      .rpc()
    const account = await program.account.counter.fetch(counter.publicKey)
    console.log(account.count.toNumber())
    expect(account.count.toNumber() === 1)
  })

  it("Counter Decrease",async ()=>{
    const tx = await program.methods
      .decrement()
      .accounts({counter:counter.publicKey,user:provider.wallet.publicKey})
      .rpc()
    const account = await program.account.counter.fetch(counter.publicKey)
    console.log(account.count.toNumber())
    expect(account.count.toNumber() === 0)
  })

  it("Counter set count",async ()=>{
    const tx = await program.methods
      .setCount(new anchor.BN(68))
      .accounts({counter:counter.publicKey,user:provider.wallet.publicKey})
      .rpc()
    const account = await program.account.counter.fetch(counter.publicKey)
    console.log(account.count.toNumber())
    expect(account.count.toNumber() === 68)
  })
});
