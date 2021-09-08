// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rand::Rng;

/// Generates a random [`bool`].
pub fn rand_bool() -> bool {
    rand::thread_rng().gen::<bool>()
}
