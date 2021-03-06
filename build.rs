// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at
// https://mozilla.org/MPL/2.0/.
//
// Author: https://github.com/Prof-Bloodstone/
// Commit: "Some more work on tests, parsing json etc" https://github.com/Prof-Bloodstone/botstone/commit/7c8b6672919c67256a3735869159ef052b8faeb2

// I license all of my modifications and additions to this file also under the Mozilla Public License, v. 2.0,
// Obtain a copy at https://mozilla.org/MPL/2.0/

#[cfg(not(feature = "deterministic"))]
#[path = "src/version_data.rs"]
mod version_data;

use std::{
    env::{self},
    path::{Path, PathBuf},
};

#[cfg(not(feature = "deterministic"))]
use std::process::Command;

fn main() -> std::io::Result<()> {
    println!("Version");
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir).join("embed");
    let _ = std::fs::create_dir(&out_dir);
    let version_file_dest_path = out_dir.join("version.json");

    copy_resource_reactions(&out_dir);
    println!("?");

    let _ = std::fs::remove_file(&version_file_dest_path);

    #[cfg(not(feature = "deterministic"))]
    {
        use chrono::Utc;
        use std::{
            env::consts::{ARCH, OS},
            fs::File,
            io::Write,
        };
        use version_data::VersionData;

        let mut file = File::create(version_file_dest_path)?;
        let is_git_info_available = git_cmd()
            .arg("status")
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        let data = VersionData {
            build: env::var("PROFILE").unwrap(),
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            branch: if is_git_info_available {
                get_branch_name()
            } else {
                None
            }
            .unwrap_or_else(|| "NONE".to_string()),
            commit: if is_git_info_available {
                get_commit_hash()
            } else {
                None
            }
            .unwrap_or_else(|| "NONE".to_string()),
            clean_worktree: is_git_info_available && is_working_tree_clean(),
            os: OS.to_string(),
            arch: ARCH.to_string(),
            timestamp: Utc::now().format("%Y-%m-%d %H:%M").to_string(),
        };

        let version_path = env::var("OUT_DIR").unwrap();
        println!("{}", version_path);
        let serialized_data = serde_json::to_string::<VersionData>(&data).unwrap();
        println!("Data: {:?}", data);
        let _ = file.write(serialized_data.as_bytes()).unwrap();
        file.flush()?;
    }

    println!("cargo:rustc-env=RESOURCE_PATH={}", out_dir.display());

    Ok(())
}

fn copy_resource_reactions(out_dir: &PathBuf) {
    let cargo = env::var("CARGO_MANIFEST_DIR").expect("fuc");
    let _ = std::fs::copy(
        Path::new(&cargo).join("res/reactions/skull.png"),
        out_dir.join("skull.png"),
    );
    for i in 1..7 {
        let _ = std::fs::copy(
            Path::new(&cargo).join(format!("res/reactions/{}.png", i)),
            out_dir.join(format!("{}.png", i)),
        );
    }
}

#[cfg(not(feature = "deterministic"))]
fn git_cmd() -> Command {
    let mut cmd = Command::new("git".to_string());
    cmd.current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd
}

#[cfg(not(feature = "deterministic"))]
fn get_commit_hash() -> Option<String> {
    let output = git_cmd()
        .arg("log")
        .arg("-1")
        .arg("--pretty=format:%h")
        .output()
        .unwrap();

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        None
    }
}

#[cfg(not(feature = "deterministic"))]
fn get_branch_name() -> Option<String> {
    let output = git_cmd()
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .unwrap();

    if output.status.success() {
        Some(
            String::from_utf8_lossy(&output.stdout)
                .trim_end()
                .to_string(),
        )
    } else {
        None
    }
}

#[cfg(not(feature = "deterministic"))]
fn is_working_tree_clean() -> bool {
    let status = git_cmd()
        .arg("diff")
        .arg("--quiet")
        .arg("--exit-code")
        .arg("HEAD")
        .status()
        .unwrap();

    status.success()
}
