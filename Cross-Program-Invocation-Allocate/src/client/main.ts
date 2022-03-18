/**
 * Hello world
 */

import {
  establishConnection,
  establishPayer,
  allocate,
} from './allocate';

async function main() {
  console.log("Let's allocate an PDA account");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();


  // create the account;
  await allocate();
  

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
