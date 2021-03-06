/**
 * Hello world
 */

import {
  establishConnection,
  establishPayer,
  checkProgram,
  allocate,
} from './allocate';

async function main() {
  console.log("Let's create a fake account ");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();
  
  await checkProgram();

  // allocate the account;
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
