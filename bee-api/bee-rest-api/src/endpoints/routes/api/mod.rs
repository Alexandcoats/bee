// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod core;
pub mod plugins;

use axum::Router;

use crate::endpoints::storage::StorageBackend;

pub(crate) fn filter<B: StorageBackend>() -> Router {
    Router::new().nest("/api", plugins::filter::<B>().merge(core::filter::<B>()))
}
