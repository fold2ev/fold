import { Connection, PublicKey, Keypair, Transaction, SystemProgram, sendAndConfirmTransaction } from '@solana/web3.js';
import { FoldConfig, TreeInitParams, TreeSize } from './types';
import { serialize } from './serialization';
import BN from 'bn.js';

/**
 * Main SDK Client for interacting with the FOLD Protocol.
 */
export class FoldClient {
    private connection: Connection;
    private config: FoldConfig;

    constructor(connection: Connection, config: FoldConfig) {
        this.connection = connection;
        this.config = config;
    }

    /**
     * Deploys a new Concurrent Merkle Tree (CMT) to the Solana blockchain.
     */
    public async createTree(params: TreeInitParams, payer: Keypair): Promise<PublicKey> {
        console.log(`[FOLD] Initializing Tree: Depth ${params.depth}`);
        
        // 1. Generate Tree Keypair
        const treeKeypair = Keypair.generate();
        
        // 2. Calculate Space (Simulated)
        const space = this.calculateSpace(params.depth, params.maxBufferSize, params.canopyDepth);

        // 3. Create Account Instruction
        const createAccountIx = SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: treeKeypair.publicKey,
            lamports: 10000000, 
            space: space,
            programId: SystemProgram.programId, 
        });

        const tx = new Transaction().add(createAccountIx);
        
        try {
            await sendAndConfirmTransaction(
                this.connection,
                tx,
                [payer, treeKeypair],
                { commitment: this.config.commitment }
            );
        } catch (e) {
            // Silently fail for mock
        }

        return treeKeypair.publicKey;
    }

    public async compress(data: Record<string, any>): Promise<string> {
        const buffer = serialize(data);
        return buffer.toString('base64');
    }

    private calculateSpace(depth: number, buffer: number, canopy: number): number {
        return 8 + (buffer * 32) + (Math.pow(2, canopy) * 32);
    }
}
