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
use crate::repository::Error::{
    Git, HeadNotFound, InvalidName, InvalidRemote, NameNotFound, RemoteNotFound,
};
use git2::Oid;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;
use url::{ParseError, Url};

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "Name is not explicitly specified, although name cannot be inferred from given url or path"
    )]
    NameNotFound,
    #[error(
        "Name is not suitable for single segment of path -or- contains invalid unicode sequence"
    )]
    InvalidName,
    #[error("Remote is not specified")]
    RemoteNotFound,
    #[error("malformed url has specified for remote: {0}")]
    MalformedRemoteUrl(#[from] ParseError),
    #[error("Remote name contains invalid unicode sequence")]
    InvalidRemote,
    #[error("HEAD not found in repository")]
    HeadNotFound,
    #[error("git operation error: {0}")]
    Git(#[from] git2::Error),
    #[error(transparent)]
    Tokio(#[from] tokio::task::JoinError),
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Version(Oid);

impl From<Oid> for Version {
    fn from(value: Oid) -> Self {
        Self(value)
    }
}

pub struct Repository {
    name: String,
    remote: Url,
    path: Arc<PathBuf>,
    inner: git2::Repository,
}

impl Repository {
    fn get_name_from_url(url: &Url) -> Option<String> {
        url.path_segments()
            .into_iter()
            .last()
            .map(|name| name.collect())
    }

    fn get_repo_path(name: impl AsRef<Path>) -> PathBuf {
        #[cfg(unix)]
        return PathBuf::from("/opt/atm").join(name);
        #[cfg(windows)]
        return PathBuf::from("C:/Program Files/ATM").join(name);
    }

    pub async fn clone(remote: Url, name: Option<String>) -> Result<Self, Error> {
        let Some(name) = Self::get_name_from_url(&remote).or(name) else {
            return Err(NameNotFound);
        };

        // validate given name
        let name = PathBuf::from(&name);
        let segments: Vec<&OsStr> = name.into_iter().collect();
        if segments.len() != 1 {
            return Err(InvalidName);
        }

        let name = segments[0];
        let path = Arc::new(Self::get_repo_path(name));
        // safe unwrap:  1) name is passed by &str  2) PathBuf doesn't change inner value
        let name = name.to_str().unwrap().to_string();

        // cloning is intended : cloning repository may be much expensive then small reallocation
        let remote_clone = remote.to_string();
        let path_clone = path.clone();
        let repo = tokio::task::spawn_blocking(move || {
            git2::Repository::clone_recurse(&remote_clone, path_clone.as_ref())
        })
        .await??;

        Ok(Self {
            name,
            remote,
            path,
            inner: repo,
        })
    }

    pub async fn open(path: PathBuf) -> Result<Self, Error> {
        tokio::task::spawn_blocking(move || {
            let repo = git2::Repository::open(&path).map_err(Git)?;

            let remotes = repo.remotes()?;
            let remote = remotes
                .into_iter()
                .take(1)
                .last()
                .flatten()
                .ok_or_else(|| RemoteNotFound)?;
            let remote = Url::parse(remote)?;

            let Some(name) = path.iter().last() else {
                return Err(NameNotFound);
            };

            let Some(name) = name.to_str() else {
                return Err(InvalidName);
            };

            Ok(Self {
                name: name.to_string(),
                remote,
                path: Arc::new(path),
                inner: repo,
            })
        })
        .await?
    }

    pub async fn pull(&self) -> Result<(), Error> {
        let path = self.path.clone();
        tokio::task::spawn_blocking(move || {
            // cannot use cached repository : it's not thread-safe
            let repo = git2::Repository::open(path.as_ref())?;

            let remote_buf = repo.branch_upstream_remote("HEAD")?;
            let Some(remote) = remote_buf.as_str() else {
                return Err(InvalidRemote);
            };

            let mut remote = repo.find_remote(remote)?;
            remote.fetch(&["HEAD"], None, None)?;
            Ok(())
        })
        .await?
    }

    pub fn version(&self) -> Result<Version, Error> {
        Ok(self.inner.refname_to_id("HEAD")?.into())
    }

    pub async fn remote_version(&self) -> Result<Version, Error> {
        let path = self.path.clone();
        let oid = tokio::task::spawn_blocking(move || {
            // cannot use cached repository : it's not thread-safe
            let repo = git2::Repository::open(path.as_ref())?;

            let remote_name_buf = repo.branch_upstream_remote("HEAD")?;
            let Some(remote) = remote_name_buf.as_str() else {
                return Err(InvalidRemote);
            };

            // TODO : retrieving resources from external source - make it asynchronous
            let mut remote = repo.find_remote(remote)?;
            remote.connect(git2::Direction::Fetch)?;

            // retrieve id of HEAD
            let mut oid: Option<Oid> = None;
            for head in remote.list()? {
                if head.name() == "HEAD" {
                    oid = Some(head.oid());
                }
            }
            let Some(oid) = oid else {
                return Err(HeadNotFound);
            };

            remote.disconnect()?;

            Ok(oid)
        })
        .await??;

        Ok(oid.into())
    }
}
