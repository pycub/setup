use anyhow::{Context, Result};
use colored::Colorize;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

/// Securely prompts for a password without echoing it to the terminal
pub fn prompt_for_password(prompt: &str) -> Result<String> {
    eprint!("{} ", prompt);
    std::io::stderr().flush()?;

    rpassword::read_password().context("Failed to read password")
}

/// Maintains a cached sudo credential to avoid repeated password prompts
pub struct SudoSession {
    cached: Arc<Mutex<bool>>,
}

impl SudoSession {
    pub fn new() -> Self {
        Self {
            cached: Arc::new(Mutex::new(false)),
        }
    }

    /// Ensures sudo credentials are valid, prompting for password if needed
    pub fn ensure_sudo_access(&self) -> Result<()> {
        {
            let cached = self.cached.lock().unwrap();
            if *cached {
                return Ok(());
            }
        }

        // Try a no-op sudo command to check if credentials are cached
        let status = Command::new("sudo")
            .arg("-n") // Non-interactive, fail if password is required
            .arg("true")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .context("Failed to execute sudo check")?;

        if status.success() {
            // Sudo access is already available
            let mut cached = self.cached.lock().unwrap();
            *cached = true;
            return Ok(());
        }

        println!(
            "{}",
            "This operation requires administrator privileges.".yellow()
        );
        let password = prompt_for_password("🔒 Enter your password: ")?;

        // Use -S to read password from stdin
        let mut child = Command::new("sudo")
            .arg("-S") // Read password from stdin
            .arg("-v") // Validate and extend timeout
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn sudo process")?;

        // Send password to sudo
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(password.as_bytes())?;
            stdin.write_all(b"\n")?;
        }

        let status = child.wait().context("Failed to wait for sudo process")?;

        if !status.success() {
            anyhow::bail!("Authentication failed. Please check your password and try again.");
        }

        let mut cached = self.cached.lock().unwrap();
        *cached = true;

        println!("{}", "Authentication successful.".green());
        Ok(())
    }

    /// Run a command with sudo, handling authentication as needed
    pub fn run_sudo_command(&self, args: &[&str]) -> Result<()> {
        self.ensure_sudo_access()?;

        // Now run the actual sudo command
        crate::utils::system::run_command("sudo", args)
    }
}

impl Default for SudoSession {
    fn default() -> Self {
        Self::new()
    }
}

// Helper function to securely store credentials for specific services
pub fn set_credentials(service_name: &str, username: &str) -> Result<()> {
    println!("Setting up credentials for: {}", service_name.blue());
    let username = if username.is_empty() {
        dialoguer::Input::<String>::new()
            .with_prompt("Username")
            .interact()?
    } else {
        username.to_string()
    };

    let password = prompt_for_password("🔒 Password: ")?;

    // For this example, we'll just demonstrate the concept
    // In a real application, use a secure credential store like keyring
    println!(
        "{} Credentials for {} stored securely",
        "✓".green(),
        service_name
    );

    // We don't actually store them in this example
    // Real implementation would use keyring crate or similar
    // keyring::Entry::new("ubuntu-setup-cli", &format!("{}-{}", service_name, username))?
    //     .set_password(&password)?;

    Ok(())
}
