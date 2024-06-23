import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day02 } from "../target/types/day_02";

describe("day_02", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day02 as Program<Day02>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(new anchor.BN(100), new anchor.BN(100), "HEllo World").rpc();
    console.log("Your transaction signature", tx);
  });

  it("Array test", async () => {
    const tx = await program.methods.array([new anchor.BN(777), new anchor.BN(120)]).rpc();
    console.log("Your transaction signature", tx);
  })

  // failed due to overflow as used checked in program
  it("overflow checks", async () => {
    const tx = await program.methods.overflow(new anchor.BN(0), new anchor.BN(1)).rpc();
    console.log("Your transaction signature", tx);
  })
});
