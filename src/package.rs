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
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use tokio::fs;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Config(#[from] toml::de::Error),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum Protocol {
    MCP,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    protocol: Protocol,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    name: String,
    port: u16,
    routes: Vec<Route>,
}

#[derive(Debug)]
pub struct Package {
    path: PathBuf,
    config: Config,
}

impl Package {
    pub async fn try_from(path: PathBuf) -> Result<Self, Error> {
        let config_path = path.join("atm.toml");
        let config_content = fs::read_to_string(&config_path).await?;
        let config: Config = toml::de::from_str(&config_content)?;

        Ok(Self { path, config })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
