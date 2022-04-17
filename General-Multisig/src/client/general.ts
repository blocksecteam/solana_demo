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
 * signer2's Keypair
 */
let signer2: Keypair;

/**
 * signer3's Keypair
 */
let signer3: Keypair;

/**
 *  program id
 */
let programId: PublicKey;

/**
 * Transaction's public key
 */
let TransactionPubkey: PublicKey; 

/**
 * Multisig's public key
 */
let MultisigPubkey: PublicKey;

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
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'General_Multisig.so');

/**
 * Path to the keypair of the deployed program.
 * This file is created when running `solana program deploy dist/program/General_Multisig.so`
 */
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'General_Multisig-keypair.json');



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

  signer2 = Keypair.generate();
  signer3 = Keypair.generate();
  

  console.log(signer2.publicKey.toBase58());
  console.log(signer3.publicKey.toBase58());
}


export async function checkProgram(): Promise<void> {
  // Read program id from keypair file
  try {
    const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
    programId = programKeypair.publicKey;
  } catch (err) {
    const errMsg = (err as Error).message;
    throw new Error(
      `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/General_Multisig.so\``,
    );
  }

  // Check if the program has been deployed
  const programInfo = await connection.getAccountInfo(programId);
  if (programInfo === null) {
    if (fs.existsSync(PROGRAM_SO_PATH)) {
      throw new Error(
        'Program needs to be deployed with `solana program deploy dist/program/General_Multisig.so`',
      );
    } else {
      throw new Error('Program needs to be built and deployed');
    }
  } else if (!programInfo.executable) {
    throw new Error(`Program is not executable`);
  }
  console.log(`Using program ${programId.toBase58()}`);

  // Derive the address (public key) of a rectangle account from the program so that it's easy to find later.
  const Transaction_SEED = 'transaction';
  TransactionPubkey = await PublicKey.createWithSeed(
    payer.publicKey,
    Transaction_SEED,
    programId,
  );

  // Check if the rectangle account has already been created
  const TransactionAccount = await connection.getAccountInfo(TransactionPubkey);
  if (TransactionAccount === null) {
    console.log(
      'Creating account',
      TransactionPubkey.toBase58(),
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
        seed: Transaction_SEED,
        newAccountPubkey: TransactionPubkey,
        lamports,
        space: 1024,
        programId,
      }),
    );
    await sendAndConfirmTransaction(connection, transaction, [payer]);
  }
}


export interface MultisigInstructionData {
    instruction: number;
}

export const multisigInstructionData = struct<MultisigInstructionData>([
    u8('instruction'),
]);

export async function createMultisig(): Promise<void> {
  
  const data = Buffer.alloc(multisigInstructionData.span);
  multisigInstructionData.encode(
      {
        instruction: 0,   
      },
      data
  );
  

  let [MultisigPubkey, bump] = await PublicKey.findProgramAddress([Buffer.from('You pass butter', 'utf8')], programId);
 

  // Check if the Config account has already been created
  const Multisig_Account = await connection.getAccountInfo(MultisigPubkey);
  if (Multisig_Account === null) {
    console.log(
      'Creating account',
      MultisigPubkey.toBase58(),
      'to store the data',
    );
    
    let syskey = SystemProgram.programId;

    const instruction = new TransactionInstruction({
    keys: [{pubkey: syskey, isSigner: false, isWritable: false},
    {pubkey: MultisigPubkey, isSigner: false, isWritable: true},
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
      MultisigPubkey.toBase58(),
      'to store the data',
    ); 
   } 
}


export interface InitializeMultisigInstructionData {
    instruction: number;
    m: number;
}

export const initializeMultisigInstructionData = struct<InitializeMultisigInstructionData>([
    u8('instruction'),
    u8('m'),
]);

/**
 *  InitializeMultisig  
 */
export async function InitializeMultisig(): Promise<void> {
  let [MultisigPubkey, bump] = await PublicKey.findProgramAddress([Buffer.from('You pass butter', 'utf8')], programId);
  const data = Buffer.alloc(initializeMultisigInstructionData.span);
  initializeMultisigInstructionData.encode(
      {
        instruction: 1,
        m: 2,   
      },
      data
  );
  
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: MultisigPubkey, isSigner: false, isWritable: true},
      {pubkey: payer.publicKey, isSigner: true, isWritable: false},
      {pubkey: signer2.publicKey, isSigner: false, isWritable: false},
      {pubkey: signer3.publicKey, isSigner: false, isWritable: false},
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

export interface CreateTransactionInstructionData {
    instruction: number;
    key: PublicKey;
    _data: number;
}

export const createTransactionInstructionData = struct<CreateTransactionInstructionData>([
    u8('instruction'),
    publicKey('key'),
    u8('_data'),
]);

/**
 *  CreateTransaction  
 */
export async function CreateTransaction(): Promise<void> {
  let [MultisigPubkey, bump] = await PublicKey.findProgramAddress([Buffer.from('You pass butter', 'utf8')], programId);
  const target = new PublicKey("4yFLHEscAeePiSUVArAFBwp4yRQcHTr5VjdrZLKPNv98");
  const config = new PublicKey("J888voF8TWtREtXsXVUQbobJPtxxQrs5u1miyWkCUkgp");

  const data = Buffer.alloc(createTransactionInstructionData.span);
  createTransactionInstructionData.encode(
      {
        instruction: 2,
        key: target,
        _data: 3,   
      },
      data
  );

  
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: TransactionPubkey, isSigner: false, isWritable: true},
      {pubkey: config, isSigner: false, isWritable: true},
      {pubkey: MultisigPubkey, isSigner: true, isWritable: false},
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


export interface Approve1InstructionData {
    instruction: number;
}

export const approve1InstructionData = struct<Approve1InstructionData>([
    u8('instruction'),
]);

/**
 *  Approve 1   
 */
export async function Approve1(): Promise<void> {
  let [MultisigPubkey, bump] = await PublicKey.findProgramAddress([Buffer.from('You pass butter', 'utf8')], programId);
  
  const data = Buffer.alloc(approve1InstructionData.span);
  approve1InstructionData.encode(
      {
        instruction: 3,   
      },
      data
  );
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: TransactionPubkey, isSigner: false, isWritable: true},
      {pubkey: MultisigPubkey, isSigner: false, isWritable: false},
      {pubkey: signer2.publicKey, isSigner: true, isWritable: false},
      
    ],
    programId,
    data: data, 
  });
  
  let airdropSignature2 = await connection.requestAirdrop(
  signer2.publicKey,
  1000000000,
  );

  await connection.confirmTransaction(airdropSignature2);

  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [signer2],
  );
  console.log(
      "success, approve1"
    );
}

export interface Approve2InstructionData {
    instruction: number;
}

export const approve2InstructionData = struct<Approve2InstructionData>([
    u8('instruction'),
]);

/**
 *  Approve 2   
 */
export async function Approve2(): Promise<void> {
  let [MultisigPubkey, bump] = await PublicKey.findProgramAddress([Buffer.from('You pass butter', 'utf8')], programId);
  
  const data = Buffer.alloc(approve2InstructionData.span);
  approve2InstructionData.encode(
      {
        instruction: 3,   
      },
      data
  );
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: TransactionPubkey, isSigner: false, isWritable: true},
      {pubkey: MultisigPubkey, isSigner: false, isWritable: false},
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
  console.log(
      "success, approve2"
    );
}


export interface ExecuteTransactionInstructionData {
    instruction: number;
}

export const executeTransactionInstructionData = struct<ExecuteTransactionInstructionData>([
    u8('instruction'),
]);

/**
 *  Execute   
 */
export async function ExecuteTransaction(): Promise<void> {
  let [MultisigPubkey, bump] = await PublicKey.findProgramAddress([Buffer.from('You pass butter', 'utf8')], programId);
  
  const target = new PublicKey("4yFLHEscAeePiSUVArAFBwp4yRQcHTr5VjdrZLKPNv98");
  const config = new PublicKey("J888voF8TWtREtXsXVUQbobJPtxxQrs5u1miyWkCUkgp");
  const data = Buffer.alloc(executeTransactionInstructionData.span);
  executeTransactionInstructionData.encode(
      {
        instruction: 4,   
      },
      data
  );
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: TransactionPubkey, isSigner: false, isWritable: true},
      {pubkey: MultisigPubkey, isSigner: false, isWritable: false},
      {pubkey: config, isSigner: false, isWritable: true},
      {pubkey: target, isSigner: false, isWritable: false},
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


