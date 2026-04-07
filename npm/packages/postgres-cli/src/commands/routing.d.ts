/**
 * Routing/Agent Commands
 * CLI commands for Tiny Dancer agent routing and management
 */
import type { TernVectorClient } from '../client.js';
export interface RegisterAgentOptions {
    name: string;
    type: string;
    capabilities: string;
    cost: string;
    latency: string;
    quality: string;
}
export interface RegisterAgentFullOptions {
    config: string;
}
export interface UpdateMetricsOptions {
    name: string;
    latency: string;
    success: boolean;
    quality?: string;
}
export interface RouteOptions {
    embedding: string;
    optimizeFor?: string;
    constraints?: string;
}
export interface FindAgentsOptions {
    capability: string;
    limit?: string;
}
export declare class RoutingCommands {
    static registerAgent(client: TernVectorClient, options: RegisterAgentOptions): Promise<void>;
    static registerAgentFull(client: TernVectorClient, options: RegisterAgentFullOptions): Promise<void>;
    static updateMetrics(client: TernVectorClient, options: UpdateMetricsOptions): Promise<void>;
    static removeAgent(client: TernVectorClient, name: string): Promise<void>;
    static setActive(client: TernVectorClient, name: string, active: boolean): Promise<void>;
    static route(client: TernVectorClient, options: RouteOptions): Promise<void>;
    static listAgents(client: TernVectorClient): Promise<void>;
    static getAgent(client: TernVectorClient, name: string): Promise<void>;
    static findByCapability(client: TernVectorClient, options: FindAgentsOptions): Promise<void>;
    static stats(client: TernVectorClient): Promise<void>;
    static clearAgents(client: TernVectorClient): Promise<void>;
    static showHelp(): void;
}
export default RoutingCommands;
//# sourceMappingURL=routing.d.ts.map