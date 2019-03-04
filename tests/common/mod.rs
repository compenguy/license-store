// Copyright 2019 Will Page <compenguy@gmail.com> and contributors
// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::fs::File;
use std::path::Path;

use license_store::Store;

pub const TEST_CACHE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test-cache.bin.gz");

pub fn load_store() -> Store {
    if Path::new(TEST_CACHE).exists() {
        return Store::from_cache(&File::open(TEST_CACHE).unwrap()).unwrap();
    }

    let mut store = Store::new();
    store
        .load_spdx(true)
        .expect("Couldn't create a store from SPDX data (needed for tests).");
    let mut cache = File::create(TEST_CACHE).unwrap();
    store.to_cache(&mut cache).unwrap();

    store
}
