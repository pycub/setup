// src/config/mod.rs
mod defaults;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub use defaults::*;

/// Configuration for the application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Enable verbose output
    #[serde(default = "default_verbose")]
    pub verbose: bool,

    /// Default answer for installation prompts if not specified
    #[serde(default = "default_auto_yes")]
    pub auto_yes: bool,

    /// Skip already installed packages
    #[serde(default = "default_skip_installed")]
    pub skip_installed: bool,

    /// Installer-specific configurations
    #[serde(default)]
    pub installers: InstallerConfigs,
}

/// Configuration for individual installers
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InstallerConfigs {
    /// ZSH configuration
    #[serde(default)]
    pub zsh: ZshConfig,

    /// Alacritty configuration
    #[serde(default)]
    pub alacritty: AlacrittyConfig,

    /// Tmux configuration
    #[serde(default)]
    pub tmux: TmuxConfig,

    /// Zed configuration
    #[serde(default)]
    pub zed: ZedConfig,
}

/// ZSH configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZshConfig {
    /// Make ZSH the default shell
    #[serde(default = "default_true")]
    pub set_as_default: bool,

    /// Plugins to install
    #[serde(default = "default_zsh_plugins")]
    pub plugins: Vec<String>,
}

impl Default for ZshConfig {
    fn default() -> Self {
        Self {
            set_as_default: default_true(),
            plugins: default_zsh_plugins(),
        }
    }
}

/// Alacritty configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlacrittyConfig {
    /// Theme to use
    #[serde(default = "default_alacritty_theme")]
    pub theme: String,

    /// Font size
    #[serde(default = "default_font_size")]
    pub font_size: f32,

    /// Font family
    #[serde(default = "default_font_family")]
    pub font_family: String,

    /// Window opacity (0.0 - 1.0)
    #[serde(default = "default_opacity")]
    pub opacity: f32,
}

impl Default for AlacrittyConfig {
    fn default() -> Self {
        Self {
            theme: default_alacritty_theme(),
            font_size: default_font_size(),
            font_family: default_font_family(),
            opacity: default_opacity(),
        }
    }
}

/// Tmux configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TmuxConfig {
    /// Use custom Tmux config
    #[serde(default = "default_true")]
    pub use_custom_config: bool,

    /// Install TPM (Tmux Plugin Manager)
    #[serde(default = "default_true")]
    pub install_tpm: bool,

    /// Plugins to install
    #[serde(default = "default_tmux_plugins")]
    pub plugins: Vec<String>,
}

impl Default for TmuxConfig {
    fn default() -> Self {
        Self {
            use_custom_config: default_true(),
            install_tpm: default_true(),
            plugins: default_tmux_plugins(),
        }
    }
}

/// Zed configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZedConfig {
    /// Theme to use
    #[serde(default = "default_zed_theme")]
    pub theme: String,

    /// Extensions to install
    #[serde(default = "default_zed_extensions")]
    pub extensions: Vec<String>,
}

impl Default for ZedConfig {
    fn default() -> Self {
        Self {
            theme: default_zed_theme(),
            extensions: default_zed_extensions(),
        }
    }
}

impl Config {
    /// Load configuration from file or create default if file doesn't exist
    pub fn load(path: Option<&Path>) -> Result<Self> {
        if let Some(path) = path {
            if path.exists() {
                let content = fs::read_to_string(path)
                    .with_context(|| format!("Failed to read config file: {:?}", path))?;

                toml::from_str(&content)
                    .with_context(|| format!("Failed to parse config file: {:?}", path))
            } else {
                Ok(Self::default())
            }
        } else {
            Ok(Self::default())
        }
    }

    /// Save configuration to file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
        }

        fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {:?}", path))?;

        Ok(())
    }

    /// Get the default config path
    pub fn default_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ubuntu-setup-cli")
            .join("config.toml")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            verbose: default_verbose(),
            auto_yes: default_auto_yes(),
            skip_installed: default_skip_installed(),
            installers: InstallerConfigs::default(),
        }
    }
}

// Default function helpers
fn default_verbose() -> bool {
    false
}

fn default_auto_yes() -> bool {
    false
}

fn default_skip_installed() -> bool {
    true
}

fn default_true() -> bool {
    true
}

fn default_font_size() -> f32 {
    12.0
}

fn default_font_family() -> String {
    "JetBrains Mono".to_string()
}

fn default_opacity() -> f32 {
    0.95
}

fn default_alacritty_theme() -> String {
    "one_dark".to_string()
}

fn default_zed_theme() -> String {
    "one_dark".to_string()
}

fn default_zsh_plugins() -> Vec<String> {
    vec![
        "git".to_string(),
        "zsh-autosuggestions".to_string(),
        "zsh-syntax-highlighting".to_string(),
    ]
}

fn default_tmux_plugins() -> Vec<String> {
    vec![
        "tmux-plugins/tpm".to_string(),
        "tmux-plugins/tmux-sensible".to_string(),
        "tmux-plugins/tmux-resurrect".to_string(),
        "tmux-plugins/tmux-continuum".to_string(),
    ]
}

fn default_zed_extensions() -> Vec<String> {
    vec![
        "rust-analyzer".to_string(),
        "prettier".to_string(),
        "python-lsp".to_string(),
    ]
}
