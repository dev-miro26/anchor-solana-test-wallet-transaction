import * as anchor from "@coral-xyz/anchor";
import { AnchorSolanaTransaction } from "../target/types/anchor_solana_transaction";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.AnchorSolanaTransaction as anchor.Program<AnchorSolanaTransaction>;

// Client code...
console.log(program.programId.toString());

program.methods.initialize().rpc().then(res=>console.log("res==>",res)).catch(err=>console.log("err",err));
