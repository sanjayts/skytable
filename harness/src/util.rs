/*
 * Created on Thu Mar 17 2022
 *
 * This file is a part of Skytable
 * Skytable (formerly known as TerrabaseDB or Skybase) is a free and open-source
 * NoSQL database written by Sayan Nandan ("the Author") with the
 * vision to provide flexibility in data modelling without compromising
 * on performance, queryability or scalability.
 *
 * Copyright (c) 2022, Sayan Nandan <ohsayan@outlook.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 *
*/

use crate::process::ExitStatus;
use crate::{HarnessError, HarnessResult};
use std::env;
use std::io::Result as IoResult;
use std::process::Child;
use std::process::Command;
pub type ExitCode = Option<i32>;

pub const VAR_TARGET: &str = "TARGET";
pub const VAR_ARTIFACT: &str = "ARTIFACT";

pub fn get_var(var: &str) -> Option<String> {
    env::var_os(var).map(|v| v.to_string_lossy().to_string())
}

pub fn handle_exitstatus(desc: &'static str, status: IoResult<ExitStatus>) -> HarnessResult<()> {
    match status {
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(HarnessError::ChildError(desc, status.code()))
            }
        }
        Err(e) => Err(HarnessError::Other(format!(
            "Failed to get exitcode while running `{desc}`. this error happened: {e}"
        ))),
    }
}

pub fn get_child(desc: impl ToString, mut input: Command) -> HarnessResult<Child> {
    let desc = desc.to_string();
    match input.spawn() {
        Ok(child) => Ok(child),
        Err(e) => Err(HarnessError::Other(format!(
            "Failed to spawn process for `{desc}` with error: {e}"
        ))),
    }
}

pub fn handle_child(desc: &'static str, input: Command) -> HarnessResult<()> {
    self::handle_exitstatus(desc, self::get_child(desc, input)?.wait())
}

pub fn sleep_sec(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs))
}

#[macro_export]
macro_rules! cmd {
    ($base:expr, $($cmd:expr),*) => {{
        let mut cmd = ::std::process::Command::new($base);
        $(
            cmd.arg($cmd);
        )*
        cmd
    }};
}
