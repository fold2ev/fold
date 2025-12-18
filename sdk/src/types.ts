import { PublicKey, TransactionInstruction } from '@solana/web3.js';
import BN from 'bn.js';

export enum TreeSize {
    MICRO = 14, // 16k leaves
    MINI = 17,  // 130k leaves
    STANDARD = 20, // 1M leaves
    MEGA = 24,  // 16M leaves
    GIGA = 26,  // 67M leaves
    TERA = 30   // 1B leaves
}

export interface FoldConfig {
    rpcUrl: string;
    commitment: 'processed' | 'confirmed' | 'finalized';
    maxRetries?: number;
    payer?: PublicKey;
}

export interface TreeInitParams {
    /** The depth of the Merkle Tree (height) */
    depth: number;
    /** The maximum buffer size for concurrent updates */
    maxBufferSize: number;
    /** The depth of the cached canopy */
    canopyDepth: number;
    /** Whether the tree is permissionless */
    public?: boolean;
    /** The authority that can modify the tree */
    authority?: PublicKey;
}

export interface CompressionResult {
    leaf: Buffer;
    index: number;
    proof: Buffer[];
    txId: string;
}

export interface MerkleProof {
    root: Buffer;
    path: Buffer[];
    leaf: Buffer;
    index: number;
}
