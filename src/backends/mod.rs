mod docker;

use std::error::Error;
use crate::backends::docker::DockerBackend;
use crate::types::Package;
pub use docker::DockerConfig;
use serde::{Deserialize, Serialize};

pub trait Backend: Send + Sync {
    fn install(&self, package: &Package) -> Result<(), Box<dyn Error>>;
    fn uninstall(&self, package: &Package) -> Result<(), Box<dyn Error>>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum BackendConfig {
    Local,
    Docker(DockerConfig),
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self::Docker(DockerConfig::default())
    }
}

#[derive(Default)]
struct DummyBackend;

impl Backend for DummyBackend {
    fn install(&self, _package: &Package) -> Result<(), Box<dyn Error>> { Ok(()) }
    fn uninstall(&self, _package: &Package) -> Result<(), Box<dyn Error>> { Ok(()) }
}

impl BackendConfig {
    pub fn to_backend(&self) -> Result<Box<dyn Backend>, Box<dyn Error>> {
        match self {
            BackendConfig::Local => Ok(Box::new(DummyBackend::default())),
            BackendConfig::Docker(_) => Ok(Box::new(DockerBackend::new()?)),
        }
    }
}
