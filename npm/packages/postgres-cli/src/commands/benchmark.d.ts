/**
 * Benchmark Commands
 * CLI commands for performance benchmarking
 */
import type { TernVectorClient } from '../client.js';
export interface BenchmarkRunOptions {
    type: 'vector' | 'attention' | 'gnn' | 'all';
    size: string;
    dim: string;
}
export interface BenchmarkReportOptions {
    format: 'json' | 'table' | 'markdown';
}
export declare class BenchmarkCommands {
    static run(client: TernVectorClient, options: BenchmarkRunOptions): Promise<void>;
    static report(client: TernVectorClient, options: BenchmarkReportOptions): Promise<void>;
    static showInfo(): void;
}
export default BenchmarkCommands;
//# sourceMappingURL=benchmark.d.ts.map