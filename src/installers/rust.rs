use super::Installer;
use crate::utils::security::SudoSession;
use crate::utils::system::run_command;
use anyhow::Result;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

pub struct RustInstaller {
    sudo_session: Arc<SudoSession>,
}

impl RustInstaller {
    pub fn new(sudo_session: Arc<SudoSession>) -> Self {
        Self { sudo_session }
    }

    fn get_cargo_home(&self) -> PathBuf {
        env::var("CARGO_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = dirs::home_dir().expect("Could not find home directory");
                home.join(".cargo")
            })
    }
}

impl Installer for RustInstaller {
    fn name(&self) -> &str {
        "Rust"
    }

    fn description(&self) -> &str {
        "Installs Rust programming language"
    }

    fn is_installed(&self) -> Result<bool> {
        let cargo_path = self.get_cargo_home().join("bin").join("cargo");
        Ok(cargo_path.exists())
    }

    fn pre_install(&self) -> Result<()> {
        // Install build dependencies
        self.sudo_session.run_sudo_command(&[
            "apt-get",
            "install",
            "-y",
            "build-essential",
            "curl",
        ])?;
        Ok(())
    }

    fn install(&self) -> Result<()> {
        // Download and run rustup installer
        let status = Command::new("sh")
            .arg("-c")
            .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y")
            .status()?;

        if !status.success() {
            anyhow::bail!("Failed to install Rust");
        }

        Ok(())
    }

    fn post_install(&self) -> Result<()> {
        // Add cargo to PATH for the current session
        println!("Rust installed successfully. Restart your shell or run:");
        println!("source \"$HOME/.cargo/env\"");
        Ok(())
    }

    fn dependencies(&self) -> Vec<&str> {
        vec!["APT Update & Upgrade"]
    }

    fn get_reinstall_msg(&self) -> String {
        format!("Are you sure reinstalling {}?", self.name())
    }

    fn sudo_session(&self) -> Arc<SudoSession> {
        self.sudo_session.clone()
    }
}
