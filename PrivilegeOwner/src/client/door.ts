/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */

import {
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';

import {
  struct,
  u32,
} from '@solana/buffer-layout';

import fs from 'mz/fs';
import path from 'path';
import { serialize, deserialize, deserializeUnchecked } from 'borsh';

import {getPayer, getRpcUrl, createKeypairFromFile} from './utils';

/**
 * Connection to the network
 */
let connection: Connection;

/**
 * Keypair associated to the fees' payer
 */
let payer: Keypair;

/**
 *  program id
 */
let programId: PublicKey;

/**
 * Door's public key
 */
let DoorPubkey: PublicKey; 

/**
 * Account's public key
 */
let AccountPubkey: PublicKey;

/**
 * Path to program files
 */
const PROGRAM_PATH = path.resolve(__dirname, '../../dist/program');

/**
 * Path to program shared object file which should be deployed on chain.
 * This file is created when running either:
 *   - `npm run build:program-c`
 *   - `npm run build:program-rust`
 */
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'PrivilegeOwner.so');

/**
 * Path to the keypair of the deployed program.
 * This file is created when running `solana program deploy dist/program/PrivilegeOwner.so`
 */
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'PrivilegeOwner-keypair.json');




/**
 * Establish a connection to the cluster
 */
export async function establishConnection(): Promise<void> {
  const rpcUrl = await getRpcUrl();
  connection = new Connection(rpcUrl, 'confirmed');
  const version = await connection.getVersion();
  console.log('Connection to cluster established:', rpcUrl, version);
}

/**
 * Establish an account to pay for everything
 */
export async function establishPayer(): Promise<void> {

  payer = await getPayer();
  
  let lamports = await connection.getBalance(payer.publicKey);
 

  console.log(
    'Using account',
    payer.publicKey.toBase58(),
    'containing',
    lamports / LAMPORTS_PER_SOL,
    'SOL to pay for fees',
  );
}


export async function checkProgram(): Promise<void> {
  // Read program id from keypair file
  try {
    const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
    programId = programKeypair.publicKey;
  } catch (err) {
    const errMsg = (err as Error).message;
    throw new Error(
      `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/Rectangle_Area.so\``,
    );
  }

  // Check if the program has been deployed
  const programInfo = await connection.getAccountInfo(programId);
  if (programInfo === null) {
    if (fs.existsSync(PROGRAM_SO_PATH)) {
      throw new Error(
        'Program needs to be deployed with `solana program deploy dist/program/Rectangle_Area.so`',
      );
    } else {
      throw new Error('Program needs to be built and deployed');
    }
  } else if (!programInfo.executable) {
    throw new Error(`Program is not executable`);
  }
  console.log(`Using program ${programId.toBase58()}`);

  // Derive the address (public key) of a rectangle account from the program so that it's easy to find later.
  const Door_SEED = 'door';
  DoorPubkey = await PublicKey.createWithSeed(
    payer.publicKey,
    Door_SEED,
    programId,
  );

  // Check if the rectangle account has already been created
  const DoorAccount = await connection.getAccountInfo(DoorPubkey);
  if (DoorAccount === null) {
    console.log(
      'Creating account',
      DoorPubkey.toBase58(),
      'to store the data',
    );
    const lamports = await connection.getMinimumBalanceForRentExemption(
      1024,
    );

    const transaction = new Transaction().add(
      SystemProgram.createAccountWithSeed({
        fromPubkey: payer.publicKey,
        basePubkey: payer.publicKey,
        seed: Door_SEED,
        newAccountPubkey: DoorPubkey,
        lamports,
        space: 1024,
        programId,
      }),
    );
    await sendAndConfirmTransaction(connection, transaction, [payer]);
  }
}

export async function createAccount(): Promise<void> {
  // Derive the address (public key) of a rectangle account from the program so that it's easy to find later.
  const Account_SEED = 'account';
  AccountPubkey = await PublicKey.createWithSeed(
    payer.publicKey,
    Account_SEED,
    programId,
  );

  // Check if the rectangle account has already been created
  const Account_Account = await connection.getAccountInfo(AccountPubkey);
  if (Account_Account === null) {
    console.log(
      'Creating account',
      AccountPubkey.toBase58(),
      'to store the data',
    );
    const lamports = await connection.getMinimumBalanceForRentExemption(
      1024,
    );

    const transaction = new Transaction().add(
      SystemProgram.createAccountWithSeed({
        fromPubkey: payer.publicKey,
        basePubkey: payer.publicKey,
        seed: Account_SEED,
        newAccountPubkey: AccountPubkey,
        lamports,
        space: 1024,
        programId,
      }),
    );
    await sendAndConfirmTransaction(connection, transaction, [payer]);
  }
}


/**
 *  InitializeDoor  
 */
export async function InitializeDoor(): Promise<void> {

  let allocateStruct = {
        tag: 0,
        layout: struct([
            u8('instruction'),
            publicKey('a'),
        ])
    };

  let data = Buffer.alloc(allocateStruct.layout.span);
  let layoutFields = Object.assign({ instruction: allocateStruct.tag, a: payer.publicKey });
  allocateStruct.layout.encode(layoutFields, data);

  
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: DoorPubkey, isSigner: false, isWritable: true},
    ],
    programId,
    data: data, 
  });

  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer],
  );
}

/**
 *  InitializeAccount  
 */
export async function InitializeAccount(): Promise<void> {

  let allocateStruct = {
        tag: 1,
        layout: struct([
            u8('instruction'),
            //publicKey('a'),
        ])
    };

  let data = Buffer.alloc(allocateStruct.layout.span);
  let layoutFields = Object.assign({ instruction: allocateStruct.tag });
  allocateStruct.layout.encode(layoutFields, data);

  
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: AccountPubkey, isSigner: false, isWritable: true},
      {pubkey: DoorPubkey, isSigner: false, isWritable: true},
      {pubkey: payer.publicKey, isSigner: false, isWritable: true},
    ],
    programId,
    data: data, 
  });

  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer],
  );
}

/**
 *  Open the door   
 */
export async function open(): Promise<void> {

  let allocateStruct = {
        tag: 2,
        layout: struct([
            u8('instruction'),
            //publicKey('a'),
        ])
    };

  let data = Buffer.alloc(allocateStruct.layout.span);
  let layoutFields = Object.assign({ instruction: allocateStruct.tag });
  allocateStruct.layout.encode(layoutFields, data);

  
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: DoorPubkey, isSigner: false, isWritable: true},
      {pubkey: AccountPubkey, isSigner: false, isWritable: true},
      {pubkey: payer.publicKey, isSigner: true, isWritable: true},
    ],
    programId,
    data: data, 
  });

  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer],
  );
}

/**
 *  close the door   
 */
export async function close(): Promise<void> {

  let allocateStruct = {
        tag: 3,
        layout: struct([
            u8('instruction'),
            //publicKey('a'),
        ])
    };

  let data = Buffer.alloc(allocateStruct.layout.span);
  let layoutFields = Object.assign({ instruction: allocateStruct.tag });
  allocateStruct.layout.encode(layoutFields, data);

  
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: DoorPubkey, isSigner: false, isWritable: true},
      {pubkey: AccountPubkey, isSigner: false, isWritable: true},
      {pubkey: payer.publicKey, isSigner: true, isWritable: true},
    ],
    programId,
    data: data, 
  });

  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer],
  );
}
