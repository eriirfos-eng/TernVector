#!/usr/bin/env node
/**
 * WASM Package Test Script
 * Tests ruvector-math-wasm and ruvector-attention-wasm in Node.js
 */

import { readFileSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));

console.log('🧪 Testing TernVector WASM Packages\n');

// ============================================================================
// Test ruvector-math-wasm
// ============================================================================
async function testMathWasm() {
  console.log('📦 Testing ruvector-math-wasm...');

  const pkgPath = join(__dirname, '../crates/ruvector-math-wasm/pkg');

  try {
    // Load WASM module
    const wasmPath = join(pkgPath, 'ruvector_math_wasm_bg.wasm');
    const wasmBuffer = readFileSync(wasmPath);

    // Import the JS bindings
    const mathWasm = await import(join(pkgPath, 'ruvector_math_wasm.js'));

    // Initialize with WASM bytes
    await mathWasm.default(wasmBuffer);

    // Test 1: Sliced Wasserstein Distance
    console.log('  ├─ Testing SlicedWasserstein...');
    const sw = new mathWasm.WasmSlicedWasserstein(100);

    // Create test point clouds (3 points in 2D each)
    const source = new Float64Array([0, 0, 1, 0, 0, 1]);
    const target = new Float64Array([2, 0, 3, 0, 2, 1]);

    const distance = sw.distance(source, target, 2);
    console.log(`  │  Distance: ${distance.toFixed(4)}`);

    if (distance > 0 && distance < 10) {
      console.log('  │  ✅ SlicedWasserstein works!');
    } else {
      throw new Error(`Unexpected distance: ${distance}`);
    }

    // Test 2: Product Manifold
    console.log('  ├─ Testing ProductManifold...');
    const manifold = new mathWasm.WasmProductManifold(4, 2, 2); // E^4 x H^2 x S^2

    // Create test points (8D total)
    const pointA = new Float64Array([1, 0, 0, 0, 0.1, 0.1, 1, 0]);
    const pointB = new Float64Array([0, 1, 0, 0, 0.2, 0.1, 0, 1]);

    const manifoldDist = manifold.distance(pointA, pointB);
    console.log(`  │  Manifold distance: ${manifoldDist.toFixed(4)}`);

    if (manifoldDist > 0) {
      console.log('  │  ✅ ProductManifold works!');
    } else {
      throw new Error(`Unexpected manifold distance: ${manifoldDist}`);
    }

    // Test 3: Spherical Space
    console.log('  ├─ Testing SphericalSpace...');
    const sphere = new mathWasm.WasmSphericalSpace(3);

    const vecA = new Float64Array([1, 0, 0]);
    const vecB = new Float64Array([0, 1, 0]);

    const sphereDist = sphere.distance(vecA, vecB);
    console.log(`  │  Spherical distance: ${sphereDist.toFixed(4)} (expected: ~1.5708 = π/2)`);

    if (Math.abs(sphereDist - Math.PI/2) < 0.01) {
      console.log('  │  ✅ SphericalSpace works!');
    } else {
      throw new Error(`Unexpected spherical distance: ${sphereDist}`);
    }

    console.log('  └─ ✅ ruvector-math-wasm: All tests passed!\n');
    return true;

  } catch (error) {
    console.error('  └─ ❌ Error:', error.message);
    return false;
  }
}

// ============================================================================
// Test ruvector-attention-wasm
// ============================================================================
async function testAttentionWasm() {
  console.log('📦 Testing ruvector-attention-wasm...');

  const pkgPath = join(__dirname, '../crates/ruvector-attention-wasm/pkg');

  try {
    // Check if pkg exists (need to build first)
    const wasmPath = join(pkgPath, 'ruvector_attention_wasm_bg.wasm');

    let wasmBuffer;
    try {
      wasmBuffer = readFileSync(wasmPath);
    } catch {
      console.log('  └─ ⚠️  Package not built. Building now...');
      const { execSync } = await import('child_process');
      execSync('wasm-pack build crates/ruvector-attention-wasm --target web --release', {
        cwd: join(__dirname, '..'),
        stdio: 'inherit'
      });
      wasmBuffer = readFileSync(wasmPath);
    }

    // Import the JS bindings
    const attentionWasm = await import(join(pkgPath, 'ruvector_attention_wasm.js'));

    // Initialize with WASM bytes
    await attentionWasm.default(wasmBuffer);

    // Test 1: Scaled Dot Product Attention
    console.log('  ├─ Testing ScaledDotProductAttention...');

    if (attentionWasm.WasmScaledDotProductAttention) {
      const attention = new attentionWasm.WasmScaledDotProductAttention(64);
      console.log('  │  ✅ ScaledDotProductAttention initialized');
    } else {
      console.log('  │  ⚠️  ScaledDotProductAttention not exported');
    }

    // Test 2: Flash Attention (if available)
    console.log('  ├─ Testing FlashAttention...');

    if (attentionWasm.WasmFlashAttention) {
      const flash = new attentionWasm.WasmFlashAttention(64, 64);
      console.log('  │  ✅ FlashAttention initialized');
    } else {
      console.log('  │  ⚠️  FlashAttention not exported');
    }

    // List available exports
    console.log('  ├─ Available exports:');
    const exports = Object.keys(attentionWasm).filter(k => k.startsWith('Wasm'));
    exports.forEach(e => console.log(`  │  - ${e}`));

    console.log('  └─ ✅ ruvector-attention-wasm: Package loaded successfully!\n');
    return true;

  } catch (error) {
    console.error('  └─ ❌ Error:', error.message);
    return false;
  }
}

// ============================================================================
// Run all tests
// ============================================================================
async function main() {
  const results = {
    math: await testMathWasm(),
    attention: await testAttentionWasm()
  };

  console.log('═══════════════════════════════════════');
  console.log('📊 Test Results:');
  console.log(`   ruvector-math-wasm:      ${results.math ? '✅ PASS' : '❌ FAIL'}`);
  console.log(`   ruvector-attention-wasm: ${results.attention ? '✅ PASS' : '❌ FAIL'}`);
  console.log('═══════════════════════════════════════');

  process.exit(results.math && results.attention ? 0 : 1);
}

main().catch(console.error);
