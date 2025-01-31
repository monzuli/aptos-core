// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

extern crate core;

mod backup;
mod backup_maintenance;
mod bootstrap;
mod replay_verify;
pub mod restore;
#[cfg(test)]
mod tests;
mod utils;

use anyhow::Result;
use aptos_db::db_debugger;
use aptos_logger::info;
use clap::Parser;

#[derive(Parser)]
#[clap(name = "Aptos db tool", author, disable_version_flag = true)]
pub enum DBTool {
    #[clap(subcommand)]
    Backup(backup::Command),

    #[clap(subcommand)]
    BackupMaintenance(backup_maintenance::Command),

    Bootstrap(bootstrap::Command),

    #[clap(subcommand)]
    Debug(db_debugger::Cmd),

    ReplayVerify(replay_verify::Opt),

    #[clap(subcommand)]
    Restore(restore::Command),
}

impl DBTool {
    pub async fn run(self) -> Result<()> {
        match self {
            DBTool::Backup(cmd) => cmd.run().await,
            DBTool::BackupMaintenance(cmd) => cmd.run().await,
            DBTool::Bootstrap(cmd) => cmd.run(),
            DBTool::Debug(cmd) => cmd.run(),
            DBTool::ReplayVerify(cmd) => {
                let ret = cmd.run().await;
                info!("Replay verify result: {:?}", ret);
                ret
            },
            DBTool::Restore(cmd) => cmd.run().await,
        }
    }
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    DBTool::command().debug_assert()
}
