/**
 * Learning Commands
 * CLI commands for self-learning and ReasoningBank operations
 */
import type { TernVectorClient } from '../client.js';
export interface TrainOptions {
    file: string;
    epochs: string;
}
export interface PredictOptions {
    input: string;
}
export declare class LearningCommands {
    static train(client: TernVectorClient, options: TrainOptions): Promise<void>;
    static predict(client: TernVectorClient, options: PredictOptions): Promise<void>;
    static status(client: TernVectorClient): Promise<void>;
    static showInfo(): void;
}
export default LearningCommands;
//# sourceMappingURL=learning.d.ts.map