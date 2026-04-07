# GPU Acceleration Guide

## Overview

The `ruvector-onnx-embeddings` crate provides optional GPU acceleration for compute-intensive operations using WebGPU (via wgpu) and optional CUDA-WASM support.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [API Reference](#api-reference)
- [Performance](#performance)
- [Shaders](#shaders)
- [Troubleshooting](#troubleshooting)

## Features

| Feature | Description | Status |
|---------|-------------|--------|
| WebGPU Backend | Cross-platform GPU acceleration | ✅ Ready |
| CUDA-WASM | CUDA code transpiled to WebGPU | 🔄 Planned |
| CPU Fallback | Automatic fallback when GPU unavailable | ✅ Ready |
| Batch Similarity | GPU-accelerated cosine/dot/euclidean | ✅ Ready |
| Pooling | Mean, Max, CLS pooling on GPU | ✅ Ready |
| Vector Ops | Normalize, matmul, add, scale | ✅ Ready |

## Installation

### Enable GPU Feature

```toml
[dependencies]
ruvector-onnx-embeddings = { version = "0.1", features = ["gpu"] }
```

### Feature Flags

| Flag | Description | Dependencies |
|------|-------------|--------------|
| `gpu` | Enable WebGPU backend | wgpu, bytemuck |
| `cuda-wasm` | Enable CUDA-WASM (includes `gpu`) | wgpu, bytemuck |
| `webgpu` | Alias for `gpu` | wgpu, bytemuck |

## Quick Start

### Basic Usage

```rust
use ruvector_onnx_embeddings::gpu::{GpuAccelerator, GpuConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create GPU accelerator with auto-detection
    let gpu = GpuAccelerator::new(GpuConfig::auto()).await?;

    // Check GPU availability
    println!("GPU available: {}", gpu.is_available());
    println!("Device: {:?}", gpu.device_info());

    // GPU-accelerated similarity search
    let query = vec![0.1, 0.2, 0.3, /* ... */];
    let candidates: Vec<&[f32]> = vec![/* ... */];

    let similarities = gpu.batch_cosine_similarity(&query, &candidates)?;

    Ok(())
}
```

### Hybrid Accelerator (Auto CPU/GPU)

```rust
use ruvector_onnx_embeddings::gpu::HybridAccelerator;

#[tokio::main]
async fn main() {
    // Automatically uses GPU when available, falls back to CPU
    let hybrid = HybridAccelerator::new().await;

    println!("Using GPU: {}", hybrid.using_gpu());

    let query = vec![0.1, 0.2, 0.3];
    let candidates = vec![vec![0.1, 0.2, 0.3], vec![0.4, 0.5, 0.6]];

    // Automatically dispatches to GPU or CPU
    let results = hybrid.batch_cosine_similarity(&query, &candidates);
}
```

## Configuration

### GpuConfig Options

```rust
use ruvector_onnx_embeddings::gpu::{GpuConfig, GpuMode, PowerPreference};

let config = GpuConfig::default()
    // GPU execution mode
    .with_mode(GpuMode::Auto)           // Auto, WebGpu, CudaWasm, CpuOnly

    // Power preference for device selection
    .with_power_preference(PowerPreference::HighPerformance)

    // Maximum GPU memory (0 = unlimited)
    .with_max_memory(1024 * 1024 * 1024) // 1GB

    // Workgroup size for compute shaders
    .with_workgroup_size(256)

    // Minimum batch size to use GPU (smaller uses CPU)
    .with_min_batch_size(16)

    // Minimum dimension to use GPU
    .with_min_dimension(128)

    // Enable profiling
    .with_profiling(true)

    // Fallback to CPU on GPU errors
    .with_fallback(true)

    // Select specific GPU device
    .with_device(0);
```

### Preset Configurations

```rust
// High performance (discrete GPU, large workgroups)
let config = GpuConfig::high_performance();

// Low power (integrated GPU, smaller workgroups)
let config = GpuConfig::low_power();

// CPU only (disable GPU)
let config = GpuConfig::cpu_only();

// WebGPU specific
let config = GpuConfig::webgpu();
```

## API Reference

### GpuAccelerator

The main GPU acceleration interface.

#### Pooling Operations

```rust
// Mean pooling
let pooled = gpu.mean_pool(
    &token_embeddings,  // [batch * seq * hidden]
    &attention_mask,    // [batch * seq]
    batch_size,
    seq_length,
    hidden_size,
)?;

// CLS pooling (first token)
let cls = gpu.cls_pool(
    &token_embeddings,
    batch_size,
    hidden_size,
)?;

// Max pooling
let max_pooled = gpu.max_pool(
    &token_embeddings,
    &attention_mask,
    batch_size,
    seq_length,
    hidden_size,
)?;
```

#### Similarity Operations

```rust
// Batch cosine similarity
let similarities = gpu.batch_cosine_similarity(&query, &candidates)?;

// Batch dot product
let dots = gpu.batch_dot_product(&query, &candidates)?;

// Batch Euclidean distance
let distances = gpu.batch_euclidean_distance(&query, &candidates)?;

// Top-K similar vectors
let top_k = gpu.top_k_similar(&query, &candidates, 10)?;
```

#### Vector Operations

```rust
// L2 normalize batch
gpu.normalize_batch(&mut vectors, dimension)?;

// Matrix-vector multiplication
let result = gpu.matmul(&matrix, &vector, rows, cols)?;

// Batch addition
let sum = gpu.batch_add(&a, &b)?;

// Batch scaling
gpu.batch_scale(&mut vectors, 2.0)?;
```

### GpuInfo

Device information structure.

```rust
let info = gpu.device_info();

println!("Name: {}", info.name);
println!("Vendor: {}", info.vendor);
println!("Backend: {}", info.backend);
println!("Total Memory: {} MB", info.total_memory / 1024 / 1024);
println!("Max Workgroup: {}", info.max_workgroup_size);
println!("Supports Compute: {}", info.supports_compute);
println!("Supports F16: {}", info.supports_f16);
```

## Performance

### Benchmarks

Run benchmarks with:

```bash
# CPU-only benchmarks
cargo bench --bench embedding_benchmark

# GPU benchmarks (requires gpu feature)
cargo bench --bench gpu_benchmark --features gpu
```

### Performance Comparison

| Operation | CPU (rayon) | WebGPU | Speedup |
|-----------|-------------|--------|---------|
| Cosine Similarity (10K×384) | 45ms | 12ms | 3.7x |
| Mean Pooling (128×256×384) | 8ms | 2ms | 4.0x |
| Normalize (10K×384) | 15ms | 4ms | 3.8x |
| Top-K (10K vectors, K=10) | 52ms | 15ms | 3.5x |

*Benchmarks on NVIDIA RTX 3080, Intel i9-12900K*

### When GPU is Faster

| Scenario | GPU Advantage |
|----------|---------------|
| Batch size ≥ 16 | ✅ Significant |
| Vector dimension ≥ 128 | ✅ Significant |
| Number of candidates ≥ 100 | ✅ Significant |
| Small batches (< 8) | ❌ CPU often faster |
| Simple operations | ❌ Transfer overhead |

### Memory Considerations

- GPU memory is limited - monitor with `gpu.device_info().total_memory`
- Large batches may need chunking
- CPU fallback handles out-of-memory gracefully

## Shaders

### Available Shaders

| Shader | Purpose | Workgroup Size |
|--------|---------|----------------|
| `cosine_similarity` | Single cosine similarity | 256 |
| `batch_cosine_similarity` | Batch cosine similarity | 256 |
| `dot_product` | Batch dot product | 256 |
| `euclidean_distance` | Batch Euclidean distance | 256 |
| `l2_normalize` | L2 normalization | 256 |
| `mean_pool` | Mean pooling | 64 |
| `max_pool` | Max pooling | 64 |
| `cls_pool` | CLS token extraction | 64 |
| `matmul` | Matrix-vector multiply | 16×16 |
| `vector_add` | Vector addition | 256 |
| `vector_scale` | Vector scaling | 256 |

### Custom Shaders

```rust
use ruvector_onnx_embeddings::gpu::ShaderRegistry;

let mut registry = ShaderRegistry::new();

registry.register(ShaderModule {
    name: "custom_op".to_string(),
    source: r#"
        @group(0) @binding(0) var<storage, read_write> data: array<f32>;

        @compute @workgroup_size(256)
        fn custom_op(@builtin(global_invocation_id) gid: vec3<u32>) {
            let idx = gid.x;
            data[idx] = data[idx] * 2.0;
        }
    "#.to_string(),
    entry_point: "custom_op".to_string(),
    workgroup_size: [256, 1, 1],
});
```

## Troubleshooting

### GPU Not Detected

```rust
// Check availability
if !ruvector_onnx_embeddings::gpu::is_gpu_available().await {
    println!("GPU not available, using CPU fallback");
}
```

**Common causes:**
- Missing GPU drivers
- WebGPU not supported by browser (for WASM)
- GPU in use by another process

### Performance Issues

1. **Check batch size**: Use `min_batch_size` to avoid GPU overhead for small batches
2. **Check dimensions**: Use `min_dimension` for small vectors
3. **Enable profiling**: `config.with_profiling(true)` to identify bottlenecks
4. **Monitor memory**: Large batches may cause thrashing

### Error Handling

```rust
match gpu.batch_cosine_similarity(&query, &candidates) {
    Ok(results) => println!("Success: {:?}", results),
    Err(e) if e.is_gpu_error() => {
        println!("GPU error, using CPU fallback: {}", e);
        // Fallback to CPU
    }
    Err(e) => return Err(e.into()),
}
```

### Debug Mode

```bash
# Enable wgpu debugging
WGPU_BACKEND_TYPE=Vulkan cargo run --features gpu

# Enable trace logging
RUST_LOG=wgpu=debug cargo run --features gpu
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────┐   │
│  │              GpuAccelerator / HybridAccelerator      │   │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────────┐   │   │
│  │  │ GpuPooler │  │GpuSimilar │  │ GpuVectorOps  │   │   │
│  │  └───────────┘  └───────────┘  └───────────────┘   │   │
│  └─────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────┐   │
│  │              Backend Abstraction (GpuBackend)        │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐    │   │
│  │  │ WebGPU     │  │ CUDA-WASM  │  │ CPU        │    │   │
│  │  │ (wgpu)     │  │ (planned)  │  │ (fallback) │    │   │
│  │  └────────────┘  └────────────┘  └────────────┘    │   │
│  └─────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────┐   │
│  │              Shader Registry (WGSL)                  │   │
│  │  cosine_similarity │ mean_pool │ normalize │ ...    │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Integration with TernVector

```rust
use ruvector_onnx_embeddings::{
    Embedder, TernVectorBuilder, Distance,
    gpu::{GpuAccelerator, GpuConfig, HybridAccelerator},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create embedder
    let embedder = Embedder::default_model().await?;

    // Create GPU accelerator
    let gpu = GpuAccelerator::new(GpuConfig::auto()).await?;

    // Create TernVector index
    let index = TernVectorBuilder::new("gpu_search")
        .embedder(embedder)
        .distance(Distance::Cosine)
        .build()?;

    // Index documents
    let docs = vec![
        "GPU acceleration improves search performance",
        "WebGPU enables cross-platform GPU compute",
        "CUDA provides native NVIDIA GPU support",
    ];
    index.insert_batch(&docs)?;

    // Search with GPU-accelerated similarity
    let query = "GPU performance";
    let results = index.search(query, 3)?;

    for result in results {
        println!("{:.4}: {}", result.score, result.text);
    }

    Ok(())
}
```

## License

MIT / Apache-2.0
