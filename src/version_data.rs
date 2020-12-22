// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at
// https://mozilla.org/MPL/2.0/.
//
// Author: https://github.com/Prof-Bloodstone/
// Commit: "Add latency to ping command" https://github.com/Prof-Bloodstone/botstone/commit/2fbae857febdde487c8f661e5f4541cd759ff3d9

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VersionData {
    pub build: String,
    pub name: String,
    pub version: String,
    pub branch: String,
    pub commit: String,
    pub clean_worktree: bool,
    pub os: String,
    pub arch: String,
    pub timestamp: String,
}
