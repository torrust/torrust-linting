use std::env;
use std::process::Command;
use std::time::Instant;

use anyhow::Result;
use tracing::{error, info};

use crate::utils::is_command_available;

/// Install lychee using Cargo.
///
/// # Errors
///
/// Returns an error if Cargo is not available or if the installation fails.
fn install_lychee() -> Result<()> {
    info!(target: "lychee", "Installing lychee...");

    if !is_command_available("cargo") {
        error!(target: "lychee", "Cargo is required to install lychee");
        return Err(anyhow::anyhow!("Cargo is not available"));
    }

    let output = Command::new("cargo").args(["install", "lychee"]).output()?;

    if output.status.success() {
        info!(target: "lychee", "lychee installed successfully");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!(target: "lychee", "Failed to install lychee: {stderr}");
        Err(anyhow::anyhow!("Failed to install lychee"))
    }
}

/// Run lychee to check local Markdown links and fragments.
///
/// Lychee automatically discovers a root `lychee.toml` from the current working directory. This
/// runner always uses offline mode to keep the standard linting workflow deterministic.
///
/// # Errors
///
/// Returns an error if lychee is not available, cannot be installed, or reports broken links.
pub fn run_lychee_linter() -> Result<()> {
    if !is_command_available("lychee") {
        install_lychee()?;
    }

    let repo_root = env::current_dir()?;
    let t = Instant::now();
    info!(target: "lychee", "Checking local Markdown links and fragments...");

    let mut command = Command::new("lychee");
    command.current_dir(&repo_root).args([
        "--offline",
        "--no-progress",
        "--exclude-path",
        "(^|/)target/",
        "--exclude-path",
        "(^|/)\\.terraform/",
        "**/*.md",
    ]);

    if repo_root.join(".github").is_dir() {
        command.arg(".github/**/*.md");
    }

    let output = command.output()?;

    if output.status.success() {
        info!(target: "lychee", "All local Markdown links passed checking! ({:.3}s)", t.elapsed().as_secs_f64());
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        if !stdout.is_empty() {
            println!("{stdout}");
        }
        if !stderr.is_empty() {
            eprintln!("{stderr}");
        }

        println!();
        error!(target: "lychee", "Local Markdown link checking failed. Please fix the issues above. ({:.3}s)", t.elapsed().as_secs_f64());
        Err(anyhow::anyhow!("Local Markdown link checking failed"))
    }
}
