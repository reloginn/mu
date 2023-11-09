use self::error::Error;

use crate::template::DEFAULT_TEMPLATE;
use colored::Colorize;
use std::{
    env,
    fs::{self, File},
};
use std::{io::Write, path::PathBuf, process::Command};
use tracing::{error, info};
use tracing_subscriber::fmt;

mod error;
mod template;

#[cfg(unix)]
fn program_is_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for path in path.split(':') {
            if fs::metadata(format!("{}/{}", path, program)).is_ok() {
                return true;
            }
        }
    }
    false
}

#[cfg(unix)]
fn main() -> Result<(), Error> {
    fmt().init();
    if program_is_in_path("tarantool") {
        let current_dir = env::current_dir()?;
        let rocks = PathBuf::from(format!("{}/.rocks", current_dir.to_string_lossy()));
        if !rocks.exists() {
            let tarantoolctl = Command::new("tarantoolctl")
                .args(["rocks", "install", "http"])
                .current_dir(&current_dir)
                .spawn()?
                .wait()?
                .success();
            if !tarantoolctl {
                error!(
                    "{}",
                    "The `tarantoolctl` child process terminated with an error"
                        .red()
                        .bold()
                );
                return Err(Error::ChildProcessTerminated);
            }
        }
        File::create(current_dir.join("template.lua"))?.write_all(DEFAULT_TEMPLATE.as_bytes())?;
        let tarantool = Command::new("tarantool").arg("template.lua").spawn()?;
        info!(
            "{}",
            format!("Tarantool successfully launched (PID: {})", tarantool.id())
                .green()
                .bold()
        );
        info!("{}", "Default server port is 5432".green().bold());
        Ok(())
    } else {
        error!("{}", "Tarantool is not found on your system".red().bold());
        Err(Error::TarantoolNotFound)
    }
}

#[cfg(windows)]
fn main() {
    fmt().init();
    error!(
        "{}",
        format!("Windows is not supported because Tarantool is not available on Windows")
            .red()
            .bold()
    )
}
