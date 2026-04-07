// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Node.js-compatible type definitions

use exo_core::{Metadata, MetadataValue, Pattern, PatternId, SearchResult, SubstrateTime};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

/// Pattern for Node.js
#[napi(object)]
#[derive(Clone)]
pub struct JsPattern {
    /// Vector embedding as Float32Array
    pub embedding: Float32Array,
    /// Metadata as JSON string
    pub metadata: Option<String>,
    /// Causal antecedents (pattern IDs as strings)
    pub antecedents: Option<Vec<String>>,
    /// Salience score (importance, default 1.0)
    pub salience: Option<f64>,
}

impl TryFrom<JsPattern> for Pattern {
    type Error = Error;

    fn try_from(pattern: JsPattern) -> Result<Self> {
        let metadata = if let Some(meta_str) = pattern.metadata {
            let fields: HashMap<String, serde_json::Value> = serde_json::from_str(&meta_str)
                .map_err(|e| Error::from_reason(format!("Invalid metadata JSON: {}", e)))?;

            let mut meta = Metadata::default();
            for (key, value) in fields {
                let meta_value = match value {
                    serde_json::Value::String(s) => MetadataValue::String(s),
                    serde_json::Value::Number(n) => {
                        MetadataValue::Number(n.as_f64().unwrap_or(0.0))
                    }
                    serde_json::Value::Bool(b) => MetadataValue::Boolean(b),
                    _ => continue,
                };
                meta.fields.insert(key, meta_value);
            }
            meta
        } else {
            Metadata::default()
        };

        // Parse antecedents from UUID strings
        let antecedents = pattern
            .antecedents
            .unwrap_or_default()
            .into_iter()
            .filter_map(|s| uuid::Uuid::parse_str(&s).ok().map(|uuid| PatternId(uuid)))
            .collect();

        Ok(Pattern {
            id: PatternId::new(),
            embedding: pattern.embedding.to_vec(),
            metadata,
            timestamp: SubstrateTime::now(),
            antecedents,
            salience: pattern.salience.unwrap_or(1.0) as f32,
        })
    }
}

/// Search result for Node.js
#[napi(object)]
#[derive(Debug, Clone)]
pub struct JsSearchResult {
    /// Pattern ID as string
    pub id: String,
    /// Similarity score (lower is better for distance metrics)
    pub score: f64,
    /// Distance value
    pub distance: f64,
}

impl From<SearchResult> for JsSearchResult {
    fn from(result: SearchResult) -> Self {
        JsSearchResult {
            id: result.pattern.id.to_string(),
            score: f64::from(result.score),
            distance: f64::from(result.distance),
        }
    }
}
