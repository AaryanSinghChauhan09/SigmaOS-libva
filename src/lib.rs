//! SigmaOS-libva - SigmaOS Component
//!
//! Graphics, audio, and storage component for SigmaOS.

use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub enabled: bool,
    pub hardware_acceleration: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { enabled: true, hardware_acceleration: true }
    }
}

pub struct libva {
    config: Config,
}

impl libva {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn initialize(&mut self) -> Result<()> {
        info!("Initializing SigmaOS-libva");
        if self.config.hardware_acceleration {
            info!("Hardware acceleration enabled");
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        info!("Running SigmaOS-libva");
        Ok(())
    }
}
