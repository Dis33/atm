/*
 * Copyright (C) 2025  Yeong-won Seo
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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
