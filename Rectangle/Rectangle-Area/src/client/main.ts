/**
 * Hello world
 */

import {
  establishConnection,
  establishPayer,
  checkProgram,
  calculate,
  report
} from './calculate';

async function main() {
  console.log("sending");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();
  
  // check and create account for data
  await checkProgram();
  
  // calculate;
  await calculate();
  
  // report the result 
  await report();

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
