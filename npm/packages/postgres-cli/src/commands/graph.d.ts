/**
 * Graph Commands
 * CLI commands for graph operations and Cypher queries
 */
import type { TernVectorClient } from '../client.js';
export interface CreateNodeOptions {
    labels: string;
    properties: string;
}
export interface TraverseOptions {
    start: string;
    depth: string;
    type: 'bfs' | 'dfs';
}
export declare class GraphCommands {
    static query(client: TernVectorClient, cypher: string): Promise<void>;
    static createNode(client: TernVectorClient, options: CreateNodeOptions): Promise<void>;
    static traverse(client: TernVectorClient, options: TraverseOptions): Promise<void>;
    static showSyntax(): void;
}
export default GraphCommands;
//# sourceMappingURL=graph.d.ts.map