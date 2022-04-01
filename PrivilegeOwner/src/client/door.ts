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
    
    console.log(
      'lamports',
      lamports
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


export interface ConfigInstructionData {
    instruction: number;
}

export const configInstructionData = struct<ConfigInstructionData>([
    u8('instruction'),
]);

export async function createConfig(): Promise<void> {
  
  const data = Buffer.alloc(configInstructionData.span);
  configInstructionData.encode(
      {
        instruction: 6,   
      },
      data
  );
  

  let [ConfigPubkey, bump] = await PublicKey.findProgramAddress([Buffer.from('You pass butter', 'utf8')], programId);
 

  // Check if the Config account has already been created
  const Config_Account = await connection.getAccountInfo(ConfigPubkey);
  if (Config_Account === null) {
    console.log(
      'Creating account',
      ConfigPubkey.toBase58(),
      'to store the data',
    );
    
    let syskey = SystemProgram.programId;

    const instruction = new TransactionInstruction({
    keys: [{pubkey: syskey, isSigner: false, isWritable: false},
    {pubkey: ConfigPubkey, isSigner: false, isWritable: true},
    {pubkey: payer.publicKey, isSigner: true, isWritable: false}],
    programId,
    data: data, 
  });
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer],
    );
  } else {
     console.log(
      'Using account',
      ConfigPubkey.toBase58(),
      'to store the data',
    ); 
   } 
}


export interface InitializeDoorInstructionData {
    instruction: number;
    key: PublicKey;
}

export const initializeDoorInstructionData = struct<InitializeDoorInstructionData>([
    u8('instruction'),
    publicKey('key'),
]);

/**
 *  InitializeDoor  
 */
export async function InitializeDoor(): Promise<void> {

  const data = Buffer.alloc(initializeDoorInstructionData.span);
  initializeDoorInstructionData.encode(
      {
        instruction: 0,
        key: payer.publicKey,   
      },
      data
  );
  
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


export interface InitializeConfigInstructionData {
    instruction: number;
    key: PublicKey;
}

export const initializeConfigInstructionData = struct<InitializeConfigInstructionData>([
    u8('instruction'),
    publicKey('key'),
]);
/**
 *  InitializeAccount  
 */
export async function InitializeConfig(): Promise<void> {
  let [ConfigPubkey, bump] = await PublicKey.findProgramAddress([Buffer.from('You pass butter', 'utf8')], programId);
  const data = Buffer.alloc(initializeConfigInstructionData.span);
  initializeConfigInstructionData.encode(
      {
        instruction: 1,
        key: payer.publicKey,   
      },
      data
  );

  
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: ConfigPubkey, isSigner: false, isWritable: true},
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




export interface LockInstructionData {
    instruction: number;
}

export const lockInstructionData = struct<LockInstructionData>([
    u8('instruction'),
]);


/**
 *  Lock the door   
 */
export async function lock(): Promise<void> {
  let [ConfigPubkey, bump] = await PublicKey.findProgramAddress([Buffer.from('You pass butter', 'utf8')], programId);
  const data = Buffer.alloc(lockInstructionData.span);
  lockInstructionData.encode(
      {
        instruction: 2,   
      },
      data
  );

  
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: ConfigPubkey, isSigner: false, isWritable: true},
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



export interface UnLockInstructionData {
    instruction: number;
}

export const unlockInstructionData = struct<UnLockInstructionData>([
    u8('instruction'),
]);


/**
 *  unlock the door   
 */
export async function unlock(): Promise<void> {
  let [ConfigPubkey, bump] = await PublicKey.findProgramAddress([Buffer.from('You pass butter', 'utf8')], programId);
  const data = Buffer.alloc(unlockInstructionData.span);
  unlockInstructionData.encode(
      {
        instruction: 3,   
      },
      data
  );

  
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: ConfigPubkey, isSigner: false, isWritable: true},
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







export interface OpenInstructionData {
    instruction: number;
}

export const openInstructionData = struct<OpenInstructionData>([
    u8('instruction'),
]);
/**
 *  Open the door   
 */
export async function open(): Promise<void> {
  let [ConfigPubkey, bump] = await PublicKey.findProgramAddress([Buffer.from('You pass butter', 'utf8')], programId);
  const data = Buffer.alloc(openInstructionData.span);
  openInstructionData.encode(
      {
        instruction: 4,   
      },
      data
  );

  
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: DoorPubkey, isSigner: false, isWritable: true},
      {pubkey: ConfigPubkey, isSigner: false, isWritable: false},
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




export interface CloseInstructionData {
    instruction: number;
}

export const closeInstructionData = struct<CloseInstructionData>([
    u8('instruction'),
]);
/**
 *  close the door   
 */
export async function close(): Promise<void> {
  let [ConfigPubkey, bump] = await PublicKey.findProgramAddress([Buffer.from('You pass butter', 'utf8')], programId);
  const data = Buffer.alloc(closeInstructionData.span);
  closeInstructionData.encode(
      {
        instruction: 5,   
      },
      data
  );

  
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: DoorPubkey, isSigner: false, isWritable: true},
      {pubkey: ConfigPubkey, isSigner: false, isWritable: false},
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
