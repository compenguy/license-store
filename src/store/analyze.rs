// Copyright 2019 Will Page <compenguy@gmail.com> and contributors
// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::{cmp::Ordering, fmt};
use rayon::prelude::*;

use crate::{license::LicenseType, license::TextData, store::base::Store};

/// Information about text that was compared against licenses in the store.
///
/// This only contains information about the overall match; to uncover more
/// data you can run methods like `optimize_bounds` on `TextData`.
///
/// Its lifetime is tied to the lifetime of the `Store` it was generated from.
#[derive(Clone)]
pub struct Match<'a> {
    /// Confidence score of the match, ranging from 0 to 1.
    pub score: f32,
    /// The name of the closest matching license in the `Store`. This will
    /// always be something that exists in the store, regardless of the score.
    pub name: String,
    /// The type of the license that matched. Useful to know if the match was
    /// the complete text, a header, or something else.
    pub license_type: LicenseType,
    /// A reference to the license data that matched inside the `Store`. May be
    /// useful for diagnostic purposes or to further optimize the result.
    pub data: &'a TextData,
}

/// A lighter version of Match to be used during analysis.
/// Reduces the need for cloning a bunch of fields.
struct PartialMatch<'a> {
    pub name: &'a str,
    pub score: f32,
    pub license_type: LicenseType,
    pub data: &'a TextData,
}

impl<'a> PartialOrd for PartialMatch<'a> {
    fn partial_cmp(&self, other: &PartialMatch<'_>) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl<'a> PartialEq for PartialMatch<'a> {
    fn eq(&self, other: &PartialMatch<'_>) -> bool {
        self.score.eq(&other.score)
            && self.name == other.name
            && self.license_type == other.license_type
    }
}

impl<'a> fmt::Debug for Match<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Match {{ score: {}, name: {}, license_type: {:?} }}",
            self.score, self.name, self.license_type
        )
    }
}

// this could probably be a stand-alone closure, but I was hitting lifetime
// hell, so a macro it is. feel free to attempt it yourself.
macro_rules! analyze_fold_closure {
    ($text:ident) => {
        |mut acc: Vec<PartialMatch<'_>>, (name, data)| {
            acc.push(PartialMatch {
                score: data.original.match_score($text),
                name,
                license_type: LicenseType::Original,
                data: &data.original,
            });
            data.alternates.iter().for_each(|alt| {
                acc.push(PartialMatch {
                    score: alt.match_score($text),
                    name,
                    license_type: LicenseType::Alternate,
                    data: alt,
                })
            });
            data.headers.iter().for_each(|head| {
                acc.push(PartialMatch {
                    score: head.match_score($text),
                    name,
                    license_type: LicenseType::Header,
                    data: head,
                })
            });
            acc
        }
    };
}

impl Store {
    /// Compare the given `TextData` against all licenses in the `Store`.
    ///
    /// This parallelizes the search as much as it can to find the best match.
    /// Once a match is obtained, it can be optimized further; see methods on
    /// `TextData` for more information.
    pub fn analyze(&self, text: &TextData) -> Match<'_> {
        let mut res: Vec<PartialMatch<'_>>;

        // parallel analysis
        res = self
            .licenses
            .par_iter()
            .fold(Vec::new, analyze_fold_closure!(text))
            .reduce(
                Vec::new,
                |mut a: Vec<PartialMatch<'_>>, b: Vec<PartialMatch<'_>>| {
                    a.extend(b);
                    a
                },
            );
        res.par_sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

        let m = &res[0];
        Match {
            score: m.score,
            name: m.name.to_string(),
            license_type: m.license_type,
            data: m.data,
        }
    }
}
