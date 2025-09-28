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

use crate::backends::Backend;
use crate::types::Package;
use bollard::Docker;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DockerConfig {
    #[serde(default = "DockerConfig::default_dockerfile")]
    dockerfile: String,

    #[serde(default = "DockerConfig::default_scoped")]
    scoped: bool,

    #[serde(default = "DockerConfig::default_max_replica")]
    max_replica: usize,
}

impl DockerConfig {
    fn default_dockerfile() -> String {
        "./Dockerfile".to_string()
    }

    fn default_scoped() -> bool {
        false
    }

    fn default_max_replica() -> usize {
        1
    }
}

impl Default for DockerConfig {
    fn default() -> Self {
        Self {
            dockerfile: Self::default_dockerfile(),
            scoped: Self::default_scoped(),
            max_replica: Self::default_max_replica(),
        }
    }
}

pub struct DockerBackend {
    docker: Arc<Docker>,
}

impl DockerBackend {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let docker = Docker::connect_with_local_defaults()?;

        Ok(Self {
            docker: Arc::new(docker),
        })
    }
}

impl Backend for DockerBackend {
    fn install(&self, package: &Package) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn uninstall(&self, package: &Package) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
