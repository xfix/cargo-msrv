use crate::paths::crate_root_folder;
use crate::{Config, TResult};
use once_cell::unsync::OnceCell;
use std::path::{Path, PathBuf};

/// Context which provides a way to lazily initialize values.
/// Once initialized, the initialized properties can be re-used.
#[derive(Debug, Clone)]
pub struct ManifestContext<'c> {
    config: &'c Config<'c>,
    manifest_path: OnceCell<PathBuf>,
}

impl ManifestContext<'_> {
    pub fn from_config(config: &Config) -> Self {
        Self {
            config,
            manifest_path: OnceCell::default(),
        }
    }

    /// Get the manifest path from the crate root folder.
    pub fn manifest_path(&self) -> TResult<&Path> {
        let config = self.config;

        let path = self
            .manifest_path
            .get_or_try_init(|| crate_root_folder(config).map(|p| p.join("Cargo.toml")))?;

        Ok(path)
    }
}
