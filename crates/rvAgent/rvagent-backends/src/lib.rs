// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! rvAgent backends — filesystem, shell, composite, state, store, and sandbox protocols.
//!
//! This crate provides all backend implementations for rvAgent, following
//! ADR-094 (Backend Protocol & Trait System) and ADR-103 (Review Amendments).
//!
//! # Backend implementations
//!
//! - [`StateBackend`](state::StateBackend) — Ephemeral in-memory file store
//! - [`FilesystemBackend`](filesystem::FilesystemBackend) — Local disk with security hardening
//! - [`LocalShellBackend`](local_shell::LocalShellBackend) — Filesystem + shell execution
//! - [`CompositeBackend`](composite::CompositeBackend) — Path-prefix routing to sub-backends
//! - [`StoreBackend`](store::StoreBackend) — Persistent key-value storage
//!
//! # Security features (ADR-103)
//!
//! - Path traversal protection with atomic resolve+open (SEC-001)
//! - Environment sanitization for shell execution (SEC-005)
//! - Unicode security detection and stripping (SEC-016)
//! - Composite path re-validation after prefix stripping (SEC-003)
//! - Literal grep mode to prevent ReDoS (SEC-021)

pub mod protocol;
pub mod security;
pub mod utils;
pub mod unicode_security;
pub mod state;
pub mod filesystem;
pub mod local_shell;
pub mod composite;
pub mod sandbox;
pub mod store;
pub mod rvf_store;
pub mod anthropic;
pub mod gemini;

// Re-export core types for convenience.
pub use protocol::{
    Backend, SandboxBackend, FileOperationError, FileInfo, FileData,
    FileDownloadResponse, FileUploadResponse, GrepMatch,
    WriteResult, EditResult, ExecuteResponse,
};
pub use state::StateBackend;
pub use filesystem::FilesystemBackend;
pub use local_shell::{LocalShellBackend, LocalShellConfig, CommandAllowlist};
pub use composite::{CompositeBackend, BackendRef};
pub use sandbox::{BaseSandbox, SandboxConfig, SandboxError, LocalSandbox};
pub use store::StoreBackend;
pub use rvf_store::MountedToolInfo;
pub use anthropic::AnthropicClient;
