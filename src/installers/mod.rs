use anyhow::Result;
use std::sync::Arc;

mod alacritty;
mod apt;
// mod nvm;
// mod oh_my_zsh;
// mod poetry;
// mod pyenv;
mod rust;
// mod tmux;
// mod zed;
// mod zsh;

pub use alacritty::AlacrittyInstaller;
pub use apt::AptInstaller;
// pub use nvm::NvmInstaller;
// pub use oh_my_zsh::OhMyZshInstaller;
// pub use poetry::PoetryInstaller;
// pub use pyenv::PyenvInstaller;
pub use rust::RustInstaller;

use crate::utils::security::SudoSession;
// pub use tmux::TmuxInstaller;
// pub use zed::ZedInstaller;
// pub use zsh::ZshInstaller;

pub trait Installer {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn is_installed(&self) -> Result<bool>;
    fn pre_install(&self) -> Result<()>;
    fn install(&self) -> Result<()>;
    fn post_install(&self) -> Result<()>;
    fn dependencies(&self) -> Vec<&str> {
        Vec::new()
    }
    fn get_reinstall_msg(&self) -> String;
    fn sudo_session(&self) -> Arc<SudoSession>;
}

pub struct InstallerRegistry {
    installers: Vec<Box<dyn Installer>>,
    sudo_session: Arc<SudoSession>,
}

impl InstallerRegistry {
    pub fn new(sudo_session: Arc<SudoSession>) -> Self {
        Self {
            installers: Vec::new(),
            sudo_session,
        }
    }

    pub fn register<I: Installer + 'static>(&mut self, installer: I) {
        self.installers.push(Box::new(installer));
    }

    pub fn register_all(&mut self) {
        self.register(AptInstaller::new(self.sudo_session.clone()));
        // self.register(ZshInstaller::new());
        // self.register(OhMyZshInstaller::new());
        // self.register(NvmInstaller::new());
        self.register(RustInstaller::new(self.sudo_session.clone()));
        self.register(AlacrittyInstaller::new(self.sudo_session.clone()));
        // self.register(PyenvInstaller::new());
        // self.register(PoetryInstaller::new());
        // self.register(ZedInstaller::new());
        // self.register(TmuxInstaller::new());
    }

    pub fn get_installers(&self) -> Vec<&Box<dyn Installer>> {
        self.installers.iter().collect()
    }
}
