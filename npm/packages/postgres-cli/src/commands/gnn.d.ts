/**
 * GNN Commands
 * CLI commands for Graph Neural Network operations
 */
import type { TernVectorClient } from '../client.js';
export interface GnnCreateOptions {
    type: 'gcn' | 'graphsage' | 'gat' | 'gin';
    inputDim: string;
    outputDim: string;
}
export interface GnnForwardOptions {
    features: string;
    edges: string;
}
export declare class GnnCommands {
    static create(client: TernVectorClient, name: string, options: GnnCreateOptions): Promise<void>;
    static forward(client: TernVectorClient, layer: string, options: GnnForwardOptions): Promise<void>;
    static listTypes(): Promise<void>;
}
export default GnnCommands;
//# sourceMappingURL=gnn.d.ts.map