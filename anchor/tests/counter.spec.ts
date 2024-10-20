import { BankrunProvider} from "anchor-bankrun";
import {startAnchor} from "solana-bankrun";
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair,PublicKey } from '@solana/web3.js';
import { Crud } from '../target/types/crud';


const IDL = require('../target/idl/crud.json');
const CrudAddress = new PublicKey("8MyKPcdT5TUWNgTiRmtdbiKQrDL1LnoQysKfb8jeZJwQ");
describe('crud', () => {

  it('Create Journal',async() => {

    const context = await startAnchor("",[{name: "Crud",programId: CrudAddress}],[]);
    const provider = new BankrunProvider(context);

    const crudProgram = new Program<Crud>(
      IDL,
      provider,
    );

    await crudProgram.methods.createJournal(
      "Aqua Culture",
      "Vanammei is the Booming Insustry in the Last 15 Years",
    ).rpc();
  },10000)

});
