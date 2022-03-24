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
import * as borsh from 'borsh';

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
 * Rectangle Account's public key
 */
let RecPubkey: PublicKey;

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


class Rectangle {
  width = 0;
  height = 0;
  area = 0;
  constructor(fields: {width: number, height: number, area: number} | undefined = undefined) {
    if (fields) {
      this.width = fields.width;
      this.height = fields.height;
      this.area = fields.area;
    }
  }
}

const RectangleSchema = new Map([
  [
      Rectangle,
      {
          kind: "struct",
          fields: [
              ['width', 'u32'],
              ['height', 'u32'],
              ['area', 'u32'],

          ]
      }
  ]
 ]);

const Rectangle_SIZE = borsh.serialize(
  RectangleSchema,
  new Rectangle(),
).length;

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
  const Rectangle_SEED = 'rec';
  RecPubkey = await PublicKey.createWithSeed(
    payer.publicKey,
    Rectangle_SEED,
    programId,
  );

  // Check if the rectangle account has already been created
  const RecAccount = await connection.getAccountInfo(RecPubkey);
  if (RecAccount === null) {
    console.log(
      'Creating account',
      RecPubkey.toBase58(),
      'to store the data',
    );
    const lamports = await connection.getMinimumBalanceForRentExemption(
      Rectangle_SIZE,
    );

    const transaction = new Transaction().add(
      SystemProgram.createAccountWithSeed({
        fromPubkey: payer.publicKey,
        basePubkey: payer.publicKey,
        seed: Rectangle_SEED,
        newAccountPubkey: RecPubkey,
        lamports,
        space: 1024,
        programId,
      }),
    );
    await sendAndConfirmTransaction(connection, transaction, [payer]);
  }
}


/**
 *  Calculate and Store 
 */
export async function calculate(): Promise<void> {

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
    keys: [
      {pubkey: RecPubkey, isSigner: false, isWritable: true}
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
 * Report Results
 */
//export async function report(): Promise<void> {
//  const accountInfo = await connection.getAccountInfo(RecPubkey);
//  if (accountInfo === null) {
//    throw 'Error: cannot find the Rectangle account';
//  }


//  const rectangle1 = borsh.deserialize(
//    RectangleSchema,
//    Rectangle,
//    accountInfo.data,
//  );
//  console.log(
//    RecPubkey.toBase58(),
//    'width:',
//    rectangle1.width,
//    'height:',
//    rectangle1.height,
//    'area:',
//    rectangle1.area
//  );
//}