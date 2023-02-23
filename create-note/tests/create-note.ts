import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CreateNote } from "../target/types/create_note";

describe("create-note", () => {
  let provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.CreateNote as Program<CreateNote>;

  let noter = anchor.web3.Keypair.generate()

  const note = {
    title: "First title",
    body: "Wow what a good movie it was real great",
    date: new Date().toString(),
  }

  const noteUpdate = {
    title: "First title",
    body: "Wow what a good movie it was real great note updated",
    date: new Date().toString(),
  }

  const [notePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(note.title), provider.wallet.publicKey.toBuffer()],
    program.programId
  )

  const [notePdaUpdate] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(noteUpdate.title), provider.wallet.publicKey.toBuffer()],
    program.programId
  )

  // it("Note initialized!", async () => {
  //   const tx = await program.methods
  //     .initialize("First note","Note description",new Date().toString())
  //     .accounts({note :noter.publicKey})
  //     .signers([noter])
  //     .rpc();
  //   console.log("Your transaction signature", tx);
  //   const account = await program.account.note.fetch(noter.publicKey)
  //   console.log(account)
  // });

  // it("Update Note!", async () => {
  //   const tx = await program.methods
  //     .updateNote("First note update","Note description updating",new Date().toString())
  //     .accounts({note :noter.publicKey})
  //     .rpc();
  //   console.log("Your transaction signature", tx);
  //   const account = await program.account.note.fetch(noter.publicKey)
  //   console.log(account)
  // });

  it("Note with seeds!", async () => {
    const tx = await program.methods
      .noteWithSeeds(note.title,note.body,note.date)
      .accounts({note:notePda,
        systemProgram: anchor.web3.SystemProgram.programId,})
      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.note.fetch(notePda)
    console.log(account)
  });

  it("Update Note with seeds!", async () => {
    const tx = await program.methods
      .updateNoteWithSeeds(noteUpdate.title,noteUpdate.body,noteUpdate.date)
      .accounts({note:notePdaUpdate,
        systemProgram: anchor.web3.SystemProgram.programId,})
      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.note.fetch(notePdaUpdate)
    console.log(account)
  });


  it("Delete Note with seeds!", async () => {
    const tx = await program.methods
      .deleteNoteSeeds(note.title)
      .accounts({note:notePda,
        systemProgram: anchor.web3.SystemProgram.programId,})
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
