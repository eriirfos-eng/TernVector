/**
 * Attention Commands
 * CLI commands for attention mechanism operations
 */
import type { TernVectorClient } from '../client.js';
export interface AttentionComputeOptions {
    query: string;
    keys: string;
    values: string;
    type: 'scaled_dot' | 'multi_head' | 'flash';
}
export declare class AttentionCommands {
    static compute(client: TernVectorClient, options: AttentionComputeOptions): Promise<void>;
    static listTypes(client: TernVectorClient): Promise<void>;
}
export default AttentionCommands;
//# sourceMappingURL=attention.d.ts.map