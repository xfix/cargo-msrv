use crate::Config;
use std::convert::TryFrom;
use std::path::PathBuf;

/// The minimal environment required to run the Set sub-command
pub struct SetContext {
    /// Path to the Cargo manifest
    cargo_manifest_path: PathBuf,
}

impl TryFrom<Config<'_>> for SetContext {
    type Error = ();

    fn try_from(config: Config<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            cargo_manifest_path: todo!(),
        })
    }
}

trait Override<T, V: Into<Option<T>>> {
    fn override_with(value: V) -> T {
        if let Some(t) = value.into() {
            t
        } else {
            todo!()
        }
    }
}

//
// SetContextBuilder::new()
//   .override_path<T: Override<PathBuf, _>><(config.manifest.path)
//   .build() -> SetContext
