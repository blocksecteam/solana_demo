/**
 * Hello world
 */

import {
  establishConnection,
  establishPayer,
  create,
} from './hello_world';

async function main() {
  console.log("Let's create an PDA account");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();


  // create the account;
  await create();
  

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
