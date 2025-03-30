// src/installers/apt.rs - updated with SudoSession
use super::Installer;
use crate::utils::security::SudoSession;
use crate::utils::system::run_command;
use anyhow::Result;
use std::process::Command;
use std::sync::Arc;

pub struct AptInstaller {
    updated: bool,
    sudo_session: Arc<SudoSession>,
}

impl AptInstaller {
    pub fn new(sudo_session: Arc<SudoSession>) -> Self {
        Self {
            updated: false,
            sudo_session,
        }
    }
}

impl Installer for AptInstaller {
    fn name(&self) -> &str {
        "APT Update & Upgrade"
    }

    fn description(&self) -> &str {
        "Updates and upgrades Ubuntu packages"
    }

    fn is_installed(&self) -> Result<bool> {
        // APT is always installed, but we'll check if it's been updated recently
        let output = Command::new("bash")
            .arg("-c")
            .arg("apt-get -s upgrade | grep -q '^0 upgraded'")
            .output()?;

        Ok(output.status.success())
    }

    fn pre_install(&self) -> Result<()> {
        // Ensure we have sudo access before proceeding
        self.sudo_session.ensure_sudo_access()?;
        Ok(())
    }

    fn install(&self) -> Result<()> {
        // Update apt (using sudo_session)
        self.sudo_session
            .run_sudo_command(&["apt-get", "update", "-y"])?;

        // Upgrade apt (using sudo_session)
        self.sudo_session
            .run_sudo_command(&["apt-get", "upgrade", "-y"])?;

        Ok(())
    }

    fn post_install(&self) -> Result<()> {
        // Run autoremove to clean up (using sudo_session)
        self.sudo_session
            .run_sudo_command(&["apt-get", "autoremove", "-y"])?;
        Ok(())
    }

    fn dependencies(&self) -> Vec<&str> {
        Vec::new() // APT has no dependencies
    }

    fn get_reinstall_msg(&self) -> String {
        format!("Hi")
    }

    fn sudo_session(&self) -> Arc<SudoSession> {
        self.sudo_session.clone()
    }
}
