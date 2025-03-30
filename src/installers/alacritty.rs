use super::Installer;
use crate::utils::security::SudoSession;
use crate::utils::system::run_command;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
pub struct AlacrittyInstaller {
    sudo_session: Arc<SudoSession>,
}

impl AlacrittyInstaller {
    pub fn new(sudo_session: Arc<SudoSession>) -> Self {
        Self { sudo_session }
    }

    fn get_config_path(&self) -> PathBuf {
        let home = dirs::home_dir().expect("Could not find home directory");
        home.join(".config").join("alacritty")
    }
}

impl Installer for AlacrittyInstaller {
    fn name(&self) -> &str {
        "Alacritty"
    }

    fn description(&self) -> &str {
        "Fast GPU-accelerated terminal emulator"
    }

    fn is_installed(&self) -> Result<bool> {
        let cargo_path = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".cargo")
            .join("bin")
            .join("alacritty");

        Ok(cargo_path.exists())
    }

    fn pre_install(&self) -> Result<()> {
        // Install Alacritty dependencies
        self.sudo_session.run_sudo_command(
            &[
                "apt-get",
                "install",
                "-y",
                "cmake",
                "pkg-config",
                "libfreetype6-dev",
                "libfontconfig1-dev",
                "libxcb-xfixes0-dev",
                "libxkbcommon-dev",
                "python3",
            ],
        )?;
        Ok(())
    }

    fn install(&self) -> Result<()> {
        // Install Alacritty using cargo
        let cargo_path = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".cargo")
            .join("bin")
            .join("cargo");

        let cargo_path_str = cargo_path.to_string_lossy();
        run_command(&cargo_path_str, &["install", "alacritty"])?;
        Ok(())
    }

    fn post_install(&self) -> Result<()> {
        // Create configuration directory if it doesn't exist
        let config_dir = self.get_config_path();
        fs::create_dir_all(&config_dir)?;

        // Write default configuration
        let config_path = config_dir.join("alacritty.yml");
        fs::write(&config_path, DEFAULT_ALACRITTY_CONFIG)?;

        println!("Alacritty configuration written to {:?}", config_path);
        Ok(())
    }

    fn dependencies(&self) -> Vec<&str> {
        vec!["Rust", "APT Update & Upgrade"]
    }

    fn get_reinstall_msg(&self) -> String {
        format!("Are you sure reinstalling {}?", self.name())
    }

    fn sudo_session(&self) -> Arc<SudoSession> {
        self.sudo_session.clone()
    }
}

const DEFAULT_ALACRITTY_CONFIG: &str = r#"window:
  padding:
    x: 10
    y: 10
  dynamic_padding: true
  decorations: full
  opacity: 0.95

font:
  normal:
    family: "JetBrains Mono"
    style: Regular
  bold:
    family: "JetBrains Mono"
    style: Bold
  italic:
    family: "JetBrains Mono"
    style: Italic
  size: 12.0

colors:
  primary:
    background: '#282c34'
    foreground: '#abb2bf'
  normal:
    black:   '#282c34'
    red:     '#e06c75'
    green:   '#98c379'
    yellow:  '#e5c07b'
    blue:    '#61afef'
    magenta: '#c678dd'
    cyan:    '#56b6c2'
    white:   '#abb2bf'
"#;
