// Copyright 2019 Will Page <compenguy@gmail.com> and contributors
// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod common;

use std::fs::File;
use std::io::prelude::*;

use license_store::TextData;

#[test]
fn store_loads() {
    let store = common::load_store();
    assert!(store.len() > 0, "store should have licenses");
}

#[test]
fn self_licenses() {
    let store = common::load_store();
    for license in spdx_dataset::spdx_text::SPDX_LICENSES.values() {
        let text_data: TextData = license.to_owned().into();
        let matched = store.analyze(&text_data);

        assert_eq!(license, &matched.name);
        assert_eq!(
            matched.score, 1f32,
            "license {} must have confidence 1 against itself, it was {}",
            license, matched.score
        );
    }
}

// this is primarily checking that we don't panic on empty text
#[test]
fn empty_match() {
    let store = common::load_store();
    let text = TextData::from("");
    let matched = store.analyze(&text);

    assert_eq!(0f32, matched.score);
}
