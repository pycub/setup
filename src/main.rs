use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use dialoguer::{Confirm, MultiSelect, theme::ColorfulTheme};
use std::path::PathBuf;
use std::sync::Arc;

// mod config;
mod installers;
mod utils;
//
use installers::{Installer, InstallerRegistry};
use utils::security::SudoSession;
//

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Path to configuration file
    #[clap(short, long, value_parser)]
    config: Option<PathBuf>,

    /// Skip confirmation for all installations
    #[clap(short, long)]
    yes: bool,

    /// Only show what would be installed
    #[clap(short, long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    println!("{}", "Ubuntu Dev Environment Setup".green().bold());
    println!("{}", "===========================".green());

    // Create and share the sudo session
    let sudo_session: Arc<SudoSession> = Arc::new(SudoSession::new());

    // Initialize installer registry
    let mut registry = InstallerRegistry::new(sudo_session.clone());
    registry.register_all();

    // Get available installers
    let installers: Vec<&Box<dyn Installer>> = registry.get_installers();
    let installer_names: Vec<&str> = installers.iter().map(|i| i.name()).collect();

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select components to install (use space to select, enter to confirm)")
        .defaults(&vec![true; installer_names.len()])
        .items(&installer_names)
        .interact()?;

    // Filter installers based on selection
    let selected_installers: Vec<&Box<dyn Installer>> =
        selections.iter().map(|&i| installers[i]).collect();

    run_installations(&selected_installers, cli.dry_run)?;

    println!("\n{}", "Setup completed successfully!".green().bold());
    Ok(())
}

fn run_installations(installers: &[&Box<dyn Installer>], dry_run: bool) -> Result<()> {
    for installer in installers {
        let name = installer.name();

        // Check if already installed
        if installer.is_installed()? {
            println!(
                "{} {} {}",
                "✓".green(),
                name,
                "is already installed".bright_black()
            );

            // Ask if user wants to reinstall
            if !dry_run
                && Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(installer.get_reinstall_msg())
                    .default(false)
                    .interact()?
            {
                perform_installation(installer, dry_run)?;
            }
        } else {
            // Ask if user wants to install
            if !dry_run
                && Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(&format!("Do you want to install {}?", name))
                    .default(true)
                    .interact()?
            {
                perform_installation(installer, dry_run)?;
            } else if dry_run {
                println!("{} Would install {}", "→".blue(), name);
            }
        }
    }

    Ok(())
}

fn perform_installation(installer: &Box<dyn Installer>, dry_run: bool) -> Result<()> {
    let name = installer.name();

    if dry_run {
        println!("{} Would install {}", "→".blue(), name);
        return Ok(());
    }

    println!("{} Installing {}", "→".blue(), name);

    // Run pre-installation checks
    installer
        .pre_install()
        .with_context(|| format!("Pre-installation check failed for {}", name))?;

    // Perform installation
    installer
        .install()
        .with_context(|| format!("Installation failed for {}", name))?;

    // Run post-installation setup
    installer
        .post_install()
        .with_context(|| format!("Post-installation setup failed for {}", name))?;

    println!("{} {} installed successfully", "✓".green(), name);
    Ok(())
}
