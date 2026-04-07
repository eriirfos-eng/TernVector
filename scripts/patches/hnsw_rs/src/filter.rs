// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! defines a trait for filtering requests.  
//! See examples in tests/filtertest.rs

use crate::prelude::DataId;

/// Only queries returning true are taken into account along the search
pub trait FilterT {
    fn hnsw_filter(&self, id: &DataId) -> bool;
}

impl FilterT for Vec<usize> {
    fn hnsw_filter(&self, id: &DataId) -> bool {
        self.binary_search(id).is_ok()
    }
}

impl<F> FilterT for F
where
    F: Fn(&DataId) -> bool,
{
    fn hnsw_filter(&self, id: &DataId) -> bool {
        self(id)
    }
}
