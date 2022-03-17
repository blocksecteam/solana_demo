/**
 * Hello world
 */

import {
  establishConnection,
  establishPayer,
  send,
} from './transfer';

async function main() {
  console.log("sending");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();


  // transfer;
  await send();
  

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
