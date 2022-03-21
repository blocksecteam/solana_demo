/**
 * Hello world
 */

import {
  establishConnection,
  establishPayer,
  calculate,
} from './calculate';

async function main() {
  console.log("sending");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();


  // transfer;
  await calculate();
  

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
