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

use crate::backends::BackendConfig;
use crate::ATM_PACKAGES_FILE;
use git2::{Direction, Oid, Remote, Repository};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::KeyValueMap;
use std::collections::HashMap;
use std::fs::{File, OpenOptions, TryLockError};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::{env, fs};
use thiserror::Error;
use tracing::{error, info_span, warn};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Git operation failed: {0}")]
    Git(#[from] git2::Error),

    #[error("Invalid encoding of branch name")]
    InvalidBranchNameEncoding,

    #[error("HEAD of branch not found")]
    NoBranchHead,

    #[error("{0}: {1}")]
    IOError(PathBuf, std::io::Error),

    #[error("{0}: locked by another handle/process")]
    LockError(PathBuf),

    #[error("{0}: {1}")]
    DeserializationError(PathBuf, toml::de::Error),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Protocol {
    MCP,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndpointConfig {
    path: PathBuf,
    protocol: Protocol,
}

impl EndpointConfig {
    pub fn new(path: PathBuf, protocol: Protocol) -> Self {
        Self { path, protocol }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageConfig {
    backend: BackendConfig,
    endpoint: EndpointConfig,
}

impl PackageConfig {
    pub fn new(backend: BackendConfig, endpoint: EndpointConfig) -> Self {
        Self { backend, endpoint }
    }

    pub fn backend(&self) -> &BackendConfig {
        &self.backend
    }

    pub fn endpoint(&self) -> &EndpointConfig {
        &self.endpoint
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    #[serde(rename = "$key$")]
    name: String,
    url: Url,
    commit: String,
    config: PackageConfig,
}

impl Package {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn commit(&self) -> Oid {
        Oid::from_str(&self.commit).unwrap()
    }

    pub fn config(&self) -> &PackageConfig {
        &self.config
    }
}

fn remote(url: &'_ str) -> Result<Remote<'_>, Error> {
    let mut remote = Remote::create_detached(url)?;
    remote.connect(Direction::Fetch)?;
    Ok(remote)
}

fn latest_commit(url: &Url) -> Result<String, Error> {
    let remote = remote(url.as_str())?;

    let branch = remote.default_branch()?;
    let branch = branch.as_str().ok_or(Error::InvalidBranchNameEncoding)?;

    let mut oid: Option<Oid> = None;
    for head in remote.list()? {
        if head.name() == branch {
            oid = Some(head.oid());
        }
    }

    if oid.is_none() {
        return Err(Error::NoBranchHead);
    }

    Ok(oid.unwrap().to_string())
}

pub struct PackageRoot {
    dir: PathBuf,
}

impl PackageRoot {
    fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    pub fn dir(&self) -> &PathBuf {
        &self.dir
    }
}

impl Drop for PackageRoot {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.dir) {
            warn!("{}", e)
        }
    }
}

impl Package {
    pub fn fetch(name: String, url: Url) -> Result<(Self, PackageRoot), Error> {
        let commit = latest_commit(&url)?;

        let mut s = Self {
            name,
            url,
            commit,
            config: PackageConfig::new(
                BackendConfig::default(),
                EndpointConfig::new(PathBuf::new(), Protocol::MCP),
            ),
        };

        let mut dir = env::temp_dir();
        dir.push("atm");
        dir.push(Uuid::now_v7().as_hyphenated().to_string());

        s.clone_to(&dir)?;

        let config_path = dir.join("atm.toml");
        let config_string =
            fs::read_to_string(&config_path).map_err(|e| Error::IOError(config_path.clone(), e))?;

        let config: PackageConfig = toml::from_str(&config_string)
            .map_err(|e| Error::DeserializationError(config_path.clone(), e))?;

        s.config = config;

        Ok((s, PackageRoot::new(dir)))
    }

    pub fn latest_commit(&self) -> Result<String, Error> {
        latest_commit(self.url())
    }

    pub fn clone_to(&self, dir: &PathBuf) -> Result<(), Error> {
        let repo;
        {
            let span = info_span!("Cloning...");
            let entered = span.enter();

            repo = Repository::clone_recurse(self.url().as_str(), dir)?;

            drop(entered);
            drop(span);
        }

        let commit = repo.find_commit(self.commit())?;
        let tree = commit.tree()?.into_object();

        repo.checkout_tree(&tree, None)?;

        Ok(())
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct PackageList(#[serde_as(as = "KeyValueMap<_>")] Vec<Package>);

pub struct PackageMap {
    file: File,
    map: HashMap<String, Package>,
}

impl PackageMap {
    pub fn from_file(path: &PathBuf) -> Result<Self, Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(path)
            .map_err(|e| Error::IOError(path.clone(), e))?;

        match file.try_lock() {
            Ok(_) => {}
            Err(TryLockError::Error(e)) => return Err(Error::IOError(path.clone(), e)),
            Err(TryLockError::WouldBlock) => return Err(Error::LockError(path.clone())),
        }

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .map_err(|e| Error::IOError(path.clone(), e))?;

        let list = toml::from_str::<PackageList>(&buffer)
            .map_err(|e| Error::DeserializationError(path.clone(), e))?;

        let mut map: HashMap<String, Package> = HashMap::new();
        map.reserve(list.0.len());
        for x in list.0 {
            map.insert(x.name().to_string(), x);
        }

        Ok(Self { file, map })
    }

    pub fn from_global() -> Result<Self, Error> {
        let path = PathBuf::from(ATM_PACKAGES_FILE);
        Self::from_file(&path)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }

    pub fn add(&mut self, package: Package) -> Option<Package> {
        self.map.insert(package.name().to_string(), package)
    }

    pub fn remove(&mut self, name: &str) -> Option<Package> {
        self.map.remove(name)
    }
}

impl Drop for PackageMap {
    fn drop(&mut self) {
        let v: Vec<Package> = self.map.values().cloned().collect();

        match toml::to_string_pretty(&PackageList(v)) {
            Ok(v) => {
                if let Err(e) = self.file.set_len(0) {
                    error!("Error setting length to file: {}", e);
                } else if let Err(e) = self.file.seek(SeekFrom::Start(0)) {
                    error!("Error seeking to file: {}", e);
                } else if let Err(e) = self.file.write_all(v.as_bytes()) {
                    error!("Error writing to file: {}", e);
                }
            }
            Err(e) => {
                error!("failed to serialize package list: {}", e);
            }
        };

        if let Err(e) = self.file.unlock() {
            warn!("failed to unlock package list file: {}", e)
        }
    }
}
