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
} from './door';

async function main() {
  console.log("sending");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();
  
  // check and create account for door
  await checkProgram();
  
  // create Account_Account;
  await createAccount();
  
  // Initialize Door 
  await InitializeDoor();

  // Initialize Account 
  await InitializeConfig();

  // Lock the door 
  await lock();

  // Unlock the door
  await unlock();
  
  // open 
  await open();

  // close 
  await close();

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
