/**
 * Vector Commands
 * CLI commands for vector operations
 */
import type { TernVectorClient } from '../client.js';
export interface VectorCreateOptions {
    dim: string;
    index: 'hnsw' | 'ivfflat';
}
export interface VectorInsertOptions {
    file?: string;
    text?: string;
}
export interface VectorSearchOptions {
    query?: string;
    text?: string;
    topK: string;
    metric: 'cosine' | 'l2' | 'ip';
}
export interface VectorDistanceOptions {
    a: string;
    b: string;
    metric: 'cosine' | 'l2' | 'ip';
}
export interface VectorNormalizeOptions {
    vector: string;
}
export declare class VectorCommands {
    static distance(client: TernVectorClient, options: VectorDistanceOptions): Promise<void>;
    static normalize(client: TernVectorClient, options: VectorNormalizeOptions): Promise<void>;
    static create(client: TernVectorClient, name: string, options: VectorCreateOptions): Promise<void>;
    static insert(client: TernVectorClient, table: string, options: VectorInsertOptions): Promise<void>;
    static search(client: TernVectorClient, table: string, options: VectorSearchOptions): Promise<void>;
}
export default VectorCommands;
//# sourceMappingURL=vector.d.ts.map