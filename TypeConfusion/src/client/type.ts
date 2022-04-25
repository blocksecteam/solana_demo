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
import fs from 'mz/fs';
import path from 'path';
import * as borsh from 'borsh';
import {
  struct,
  u8,
} from '@solana/buffer-layout';
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
 * Hello world's program id
 */
let programId: PublicKey;

/**
 * The public key of the User account 
 */
let UserPubkey: PublicKey;

/**
 * The public key of the Meta account 
 */
let MetaPubkey: PublicKey;

/**
 * The public key of the authority account 
 */
let AuthorityPubkey: PublicKey;


/**
 * Path to program files
 */
const PROGRAM_PATH = path.resolve(__dirname, '../../dist/program');

/**
 * Path to program shared object file which should be deployed on chain.
 * This file is created when running either:
 *   - `npm run build:program-rust`
 */
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'TypeConfusion.so');

/**
 * Path to the keypair of the deployed program.
 * This file is created when running `solana program deploy dist/program/TypeConfusion.so`
 */
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'TypeConfusion-keypair.json');




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
  let fees = 0;
  if (!payer) {
    payer = await getPayer();
  }

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
 * Check if the TypeConfusion BPF program has been deployed
 */
export async function checkProgram(): Promise<void> {
  // Read program id from keypair file
  try {
    const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
    programId = programKeypair.publicKey;
  } catch (err) {
    const errMsg = (err as Error).message;
    throw new Error(
      `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/TypeConfusion.so\``,
    );
  }

  // Check if the program has been deployed
  const programInfo = await connection.getAccountInfo(programId);
  if (programInfo === null) {
    if (fs.existsSync(PROGRAM_SO_PATH)) {
      throw new Error(
        'Program needs to be deployed with `solana program deploy dist/program/TypeConfusion.so`',
      );
    } else {
      throw new Error('Program needs to be built and deployed');
    }
  } else if (!programInfo.executable) {
    throw new Error(`Program is not executable`);
  }
  console.log(`Using program ${programId.toBase58()}`);
}

/**
 *  Create User Account
 */
export async function createUser(): Promise<void> {
 

  // Derive the address (public key) of a user account from the program so that it's easy to find later.
  const User_SEED = 'user';
  UserPubkey = await PublicKey.createWithSeed(
    payer.publicKey,
    User_SEED,
    programId,
  );

  // Check if the User account has already been created
  const UserAccount = await connection.getAccountInfo(UserPubkey);
  if (UserAccount === null) {
    console.log(
      'Creating account',
      UserPubkey.toBase58(),
    );
    const lamports = await connection.getMinimumBalanceForRentExemption(
      1024,
    );

    const transaction = new Transaction().add(
      SystemProgram.createAccountWithSeed({
        fromPubkey: payer.publicKey,
        basePubkey: payer.publicKey,
        seed: User_SEED,
        newAccountPubkey: UserPubkey,
        lamports,
        space: 1024,
        programId,
      }),
    );
    await sendAndConfirmTransaction(connection, transaction, [payer]);
  }
}

/**
 *  Create Metadata Account
 */
export async function createMeta(): Promise<void> {
 

  // Derive the address (public key) of a Meta account from the program so that it's easy to find later.
  const Meta_SEED = 'meta';
  MetaPubkey = await PublicKey.createWithSeed(
    payer.publicKey,
    Meta_SEED,
    programId,
  );

  // Check if the User account has already been created
  const MetaAccount = await connection.getAccountInfo(MetaPubkey);
  if (MetaAccount === null) {
    console.log(
      'Creating account',
      MetaPubkey.toBase58(),
    );
    const lamports = await connection.getMinimumBalanceForRentExemption(
      1024,
    );

    const transaction = new Transaction().add(
      SystemProgram.createAccountWithSeed({
        fromPubkey: payer.publicKey,
        basePubkey: payer.publicKey,
        seed: Meta_SEED,
        newAccountPubkey: MetaPubkey,
        lamports,
        space: 1024,
        programId,
      }),
    );
    await sendAndConfirmTransaction(connection, transaction, [payer]);
  }
}

/**
 *  Create Authority Account
 */
export async function createAuthority(): Promise<void> {
 

  // Derive the address (public key) of a Authority account from the program so that it's easy to find later.
  const Authority_SEED = 'authority';
  AuthorityPubkey = await PublicKey.createWithSeed(
    payer.publicKey,
    Authority_SEED,
    programId,
  );

  // Check if the User account has already been created
  const AuthorityAccount = await connection.getAccountInfo(AuthorityPubkey);
  if (AuthorityAccount === null) {
    console.log(
      'Creating account',
      AuthorityPubkey.toBase58(),
    );
    const lamports = await connection.getMinimumBalanceForRentExemption(
      1024,
    );

    const transaction = new Transaction().add(
      SystemProgram.createAccountWithSeed({
        fromPubkey: payer.publicKey,
        basePubkey: payer.publicKey,
        seed: Authority_SEED,
        newAccountPubkey: AuthorityPubkey,
        lamports,
        space: 1024,
        programId,
      }),
    );
    await sendAndConfirmTransaction(connection, transaction, [payer]);
  }
}

export interface InitializeUserInstructionData {
    instruction: number;
}

export const initializeUserInstructionData = struct<InitializeUserInstructionData>([
    u8('instruction'),
]);
/**
 * Initialize User 
 */
export async function initializeUser(): Promise<void> {
  
  const data = Buffer.alloc(initializeUserInstructionData.span);
  initializeUserInstructionData.encode(
      {
        instruction: 0,   
      },
      data
  );

  const instruction = new TransactionInstruction({
    keys: [{pubkey: UserPubkey, isSigner: false, isWritable: true},
           {pubkey: AuthorityPubkey, isSigner: false, isWritable: false},
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

export interface InitializeMetadataInstructionData {
    instruction: number;
}

export const initializeMetadataInstructionData = struct<InitializeMetadataInstructionData>([
    u8('instruction'),
]);
/**
 * Initialize Metadata 
 */
export async function initializeMetadata(): Promise<void> {
  
  const data = Buffer.alloc(initializeMetadataInstructionData.span);
  initializeMetadataInstructionData.encode(
      {
        instruction: 1,   
      },
      data
  );

  const instruction = new TransactionInstruction({
    keys: [{pubkey: MetaPubkey, isSigner: false, isWritable: true},
           {pubkey: payer.publicKey, isSigner: true, isWritable: false},
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


export interface TestInstructionData {
    instruction: number;
}

export const testInstructionData = struct<TestInstructionData>([
    u8('instruction'),
]);
/**
 * Test
 */
export async function test(): Promise<void> {
  
  const data = Buffer.alloc(testInstructionData.span);
  initializeMetadataInstructionData.encode(
      {
        instruction: 2,   
      },
      data
  );
  const instruction = new TransactionInstruction({
    keys: [{pubkey: MetaPubkey, isSigner: false, isWritable: true},
           {pubkey: payer.publicKey, isSigner: true, isWritable: false},
          ],
    programId,
    data: data, // All instructions are hellos
  });
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer],
  );
}

