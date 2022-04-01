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
  u8,
} from '@solana/buffer-layout';

import {
  publicKey
} from '@solana/buffer-layout-utils';

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
 * Config's public key
 */
let ConfigPubkey: PublicKey;

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
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'PrivilegeOwner-Attack.so');

/**
 * Path to the keypair of the deployed program.
 * This file is created when running `solana program deploy dist/program/PrivilegeOwner-Attack.so`
 */
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'PrivilegeOwner-Attack-keypair.json');



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
      `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/PrivilegeOwner-Attack.so\``,
    );
  }

  // Check if the program has been deployed
  const programInfo = await connection.getAccountInfo(programId);
  if (programInfo === null) {
    if (fs.existsSync(PROGRAM_SO_PATH)) {
      throw new Error(
        'Program needs to be deployed with `solana program deploy dist/program/PrivilegeOwner-Attack.so`',
      );
    } else {
      throw new Error('Program needs to be built and deployed');
    }
  } else if (!programInfo.executable) {
    throw new Error(`Program is not executable`);
  }
  console.log(`Using program ${programId.toBase58()}`);

  // Derive the address (public key) of a rectangle account from the program so that it's easy to find later.
  const Config_SEED = 'config';
  ConfigPubkey = await PublicKey.createWithSeed(
    payer.publicKey,
    Config_SEED,
    programId,
  );

  // Check if the rectangle account has already been created
  const ConfigAccount = await connection.getAccountInfo(ConfigPubkey);
  if (ConfigAccount === null) {
    console.log(
      'Creating account',
      ConfigPubkey.toBase58(),
      'to store the data',
    );

    const lamports = await connection.getMinimumBalanceForRentExemption(
      1024,
    );
    
    console.log(
      'lamports',
      lamports
    );
    
    const transaction = new Transaction().add(
      SystemProgram.createAccountWithSeed({
        fromPubkey: payer.publicKey,
        basePubkey: payer.publicKey,
        seed: Config_SEED,
        newAccountPubkey: ConfigPubkey,
        lamports,
        space: 1024,
        programId,
      }),
    );
    await sendAndConfirmTransaction(connection, transaction, [payer]);
  }
}


/**
 * allocate account
 */
export async function allocate(): Promise<void> {
  
  
  const instruction = new TransactionInstruction({
    keys: [{pubkey: ConfigPubkey, isSigner: false, isWritable: false},{pubkey: payer.publicKey, isSigner: true, isWritable: false}],
    programId,
    data: Buffer.alloc(0), // All instructions are hellos  
  });
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer],
  );
}








