/**
 * Hello world
 */

import {
  establishConnection,
  establishPayer,
  checkProgram,
  createConfig,
  InitializeDoor,
  InitializeConfig,
  lock,
  unlock,
  open,
  close,
  InitializeMultisig,
  createMultisig
} from './door';

async function main() {
  console.log("sending");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();
  
  // check and create account for door
  await checkProgram();

  //create multisig
  await createMultisig();
  
  // create Config_Account;
  await createConfig();
  
  //InitializeMultisig
  await InitializeMultisig();
  
  // Initialize Door 
  await InitializeDoor();

  // Initialize Account 
  await InitializeConfig();

  // Unlock the door
  await unlock();

  // open 
  await open();

  // close 
  await close();

  // Lock the door 
  await lock();


  
  
  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
