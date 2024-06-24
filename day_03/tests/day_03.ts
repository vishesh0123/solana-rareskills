import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day03 } from "../target/types/day_03";

describe("day_03", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day03 as Program<Day03>;

  it("boaty_mc_boatface", async () => {
    // Add your test here.
    const tx = await program.methods.boatyMcBoatface(new anchor.BN(100)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("add", async () => {
    const tx = await program.methods.add(new anchor.BN(100), new anchor.BN(200)).rpc();
    console.log("Your tx:", tx);

  })

  it("sub", async () => {
    const tx = await program.methods.sub(new anchor.BN(500), new anchor.BN(200)).rpc();
    console.log("Your tx:", tx);

  })
});
