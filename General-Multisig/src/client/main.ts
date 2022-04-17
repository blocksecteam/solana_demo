/**
 * General-Multisig 
 */

import {
  establishConnection,
  establishPayer,
  checkProgram,
  createMultisig,
  InitializeMultisig,
  CreateTransaction,
  Approve1,
  Approve2,
  ExecuteTransaction,
} from './general';

async function main() {
  console.log("sending");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();
  
  // check and create account for door
  await checkProgram();
  
  // create Multisig;
  await createMultisig();
  
  // Initialize Multisig 
  //await InitializeMultisig();

  // Create Transaction 
  //await CreateTransaction();

  // Approve1
  //await Approve1();
  
  // Approve2
  //await Approve2();

  // Execute Transaction
  //await ExecuteTransaction();

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
