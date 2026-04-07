/**
 * Hyperbolic Geometry Commands
 * CLI commands for hyperbolic embedding operations (Poincare ball, Lorentz model)
 *
 * NOTE: These functions require the hyperbolic geometry module to be enabled
 * in the TernVector PostgreSQL extension. Currently in development.
 */
import type { TernVectorClient } from '../client.js';
export interface PoincareDistanceOptions {
    a: string;
    b: string;
    curvature?: string;
}
export interface LorentzDistanceOptions {
    a: string;
    b: string;
    curvature?: string;
}
export interface MobiusAddOptions {
    a: string;
    b: string;
    curvature?: string;
}
export interface ExpMapOptions {
    base: string;
    tangent: string;
    curvature?: string;
}
export interface LogMapOptions {
    base: string;
    target: string;
    curvature?: string;
}
export interface ConvertOptions {
    vector: string;
    curvature?: string;
}
export declare class HyperbolicCommands {
    static poincareDistance(client: TernVectorClient, options: PoincareDistanceOptions): Promise<void>;
    static lorentzDistance(client: TernVectorClient, options: LorentzDistanceOptions): Promise<void>;
    static mobiusAdd(client: TernVectorClient, options: MobiusAddOptions): Promise<void>;
    static expMap(client: TernVectorClient, options: ExpMapOptions): Promise<void>;
    static logMap(client: TernVectorClient, options: LogMapOptions): Promise<void>;
    static poincareToLorentz(client: TernVectorClient, options: ConvertOptions): Promise<void>;
    static lorentzToPoincare(client: TernVectorClient, options: ConvertOptions): Promise<void>;
    static minkowskiDot(client: TernVectorClient, a: string, b: string): Promise<void>;
    static showHelp(): void;
}
export default HyperbolicCommands;
//# sourceMappingURL=hyperbolic.d.ts.map