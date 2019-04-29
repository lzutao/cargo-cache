// Copyright 2019 Matthias Krüger. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[path = "../src/test_helpers.rs"]
mod test_helpers;

use std::path::PathBuf;
use std::process::Command;

use walkdir::WalkDir;

#[allow(non_snake_case)]
#[test]
fn CARGO_HOME_subdirs_are_known() {
    // this tests makes cargo create a new CARGO_HOME and makes sure that the paths that are found
    // are known by cargo cache
    let cargo_home = "target/cargo_home_subdirs_known_CARGO_HOME/";

    // in the fake CARGO_HOME, install cargo-cache via git
    let command = Command::new("cargo")
        .arg("install")
        .arg("--git")
        .arg("https://github.com/matthiaskrgr/cargo-cache")
        .arg("--debug")
        .arg("--force")
        //        .current_dir(&crate_path)
        .env(
            "CARGO_TARGET_DIR",
            "target/cargo_home_dirs_are_known_target_dir/",
        )
        .env("CARGO_HOME", "target/cargo_home_subdirs_known_CARGO_HOME/")
        .output();
    // note: it does not matter if the build succeeds or not, we only need
    // cargo to inizialize the CARGO_HOME

    let status = command.unwrap();

    let stderr = String::from_utf8_lossy(&status.stderr).to_string();
    let stdout = String::from_utf8_lossy(&status.stdout).to_string();

    println!("ERR {:?}", stderr);
    println!("OUT {:?}", stdout);

    assert!(
        PathBuf::from(&cargo_home).is_dir(),
        "fake cargo home was not created!"
    );

    /*WalkDir::new(cargo_home)
        .max_depth(3)
        .into_iter()
        .for_each(|x| println!("{:?}", x));
    */
    /*
    Ok(DirEntry("target/cargo_home_subdirs_known_CARGO_HOME"))
    Ok(DirEntry("target/cargo_home_subdirs_known_CARGO_HOME/git"))
    Ok(DirEntry("target/cargo_home_subdirs_known_CARGO_HOME/git/.cargo-lock-git"))
    Ok(DirEntry("target/cargo_home_subdirs_known_CARGO_HOME/git/db"))
    Ok(DirEntry("target/cargo_home_subdirs_known_CARGO_HOME/git/db/cargo-cache-16826c8e13331adc"))
    Ok(DirEntry("target/cargo_home_subdirs_known_CARGO_HOME/git/checkouts"))
    Ok(DirEntry("target/cargo_home_subdirs_known_CARGO_HOME/git/checkouts/cargo-cache-16826c8e13331adc"))
    Ok(DirEntry("target/cargo_home_subdirs_known_CARGO_HOME/registry"))
    Ok(DirEntry("target/cargo_home_subdirs_known_CARGO_HOME/registry/index"))
    Ok(DirEntry("target/cargo_home_subdirs_known_CARGO_HOME/registry/index/github.com-1ecc6299db9ec823"))
    */

    let walkdir = WalkDir::new(cargo_home).max_depth(3);
    let mut x = walkdir
        .into_iter()
        .map(|x| x.unwrap().path().display().to_string())
        .collect::<Vec<_>>();

    x.sort();
    x.iter().for_each(|x| println!("{:?}", x));
    let mut x = x.into_iter();

    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/"));

    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/.crates.toml"));

    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/bin"));

    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/bin/cargo-cache"));

    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/git"));
    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/git/.cargo-lock-git"));

    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/git/checkouts"));
    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/git/checkouts/cargo-cache-"));

    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/git/db"));
    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/git/db/cargo-cache-"));

    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/registry"));

    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/registry/cache"));

    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/registry/cache/github.com-"));

    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/registry/index"));
    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/registry/index/github.com"));

    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/registry/src"));

    assert!(x
        .next()
        .unwrap()
        .starts_with("target/cargo_home_subdirs_known_CARGO_HOME/registry/src/github.com"));

    assert!(x.next().is_none()); // end reached
}