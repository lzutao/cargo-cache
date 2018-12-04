// Copyright 2017-2018 Matthias Krüger. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(crate) struct FileDesc {
    pub(crate) name: String,
    pub(crate) size: u64,
}

pub(crate) fn dir_exists(path: &PathBuf) -> bool {
    // check if a directory exists and print an warning message if not
    if path.exists() {
        true
    } else {
        eprintln!("Skipping '{}' because it doesn't exist.", path.display());
        false
    }
}

pub(crate) const TOP_CRATES_SPACING: usize = 3;
