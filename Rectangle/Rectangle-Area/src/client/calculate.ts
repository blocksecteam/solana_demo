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
} from ("@solana/buffer-layout");

import fs from 'mz/fs';
import path from 'path';

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
 * Rectangle_Area's program id
 */
let programId: PublicKey;


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
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'Rectangle_Area.so');

/**
 * Path to the keypair of the deployed program.
 * This file is created when running `solana program deploy dist/program/Rectangle_Area.so`
 */
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'Rectangle_Area-keypair.json');



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

  let payer = await getPayer();
  
  let lamports = await connection.getBalance(payer.publicKey);
 

  console.log(
    'Using account',
    payer.publicKey.toBase58(),
    'containing',
    lamports / LAMPORTS_PER_SOL,
    'SOL to pay for fees',
  );
}


/**
 * transfer lamports
 */
export async function calculate(): Promise<void> {
  
  
  // Read program id from keypair file
  try {
    const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
    programId = programKeypair.publicKey;
  } catch (err) {
    const errMsg = (err as Error).message;
    throw new Error(
      `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/cross_program_invocation_transfer.so\``,
    );
  }

  let allocateStruct = {
        // index: 0,
        layout: struct([
            // u8('instruction'),
            u32('a'),
            u32('b'),
        ])
    };

  let data = Buffer.alloc(allocateStruct.layout.span);
  let layoutFields = Object.assign({ a: 5, b: 10 });
  allocateStruct.layout.encode(layoutFields, data);

  
  const instruction = new TransactionInstruction({
    keys: [],
    programId,
    data: data, 
  });

  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer],
  );
}

