/**
 * TypeConfusion
 */

import {
  establishConnection,
  establishPayer,
  checkProgram,
  createUser,
  createMeta,
  createAuthority,
  initializeUser,
  initializeMetadata,
  test,
} from './type';

async function main() {
  console.log("Executing");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();
  
  // check program
  await checkProgram();
  
  // create User Account;
  await createUser();
  
  // create Meta Account;
  await createMeta();

  // create Authority Account;
  await createAuthority();


  // Initialize User 
  await initializeUser();

  // Initialize Metadata 
  await initializeMetadata();
  
  // Test 
  await test();

  console.log('Successfully passed');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
