/**
 * TernVector PostgreSQL CLI
 * Entry point for the library exports
 */

export { TernVectorClient } from './client.js';
export type {
  TernVectorInfo,
  VectorSearchResult,
  AttentionResult,
  GnnResult,
  GraphNode,
  GraphEdge,
  TraversalResult
} from './client.js';

export { VectorCommands } from './commands/vector.js';
export { AttentionCommands } from './commands/attention.js';
export { GnnCommands } from './commands/gnn.js';
export { GraphCommands } from './commands/graph.js';
export { LearningCommands } from './commands/learning.js';
export { BenchmarkCommands } from './commands/benchmark.js';
