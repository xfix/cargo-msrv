use crate::config::{Config, ModeIntent};
use crate::errors::{CargoMSRVError, IoErrorSource, TResult};
use crate::manifest::bare_version::BareVersion;
use crate::manifest::{CargoManifest, CargoManifestParser, TomlParser};
use crate::paths::crate_root_folder;
use crate::{ReportType, Reporter};
use std::convert::TryFrom;
use std::io::Write;
use toml_edit::Document;

pub fn run_show_msrv<R: ReportType, W: Write>(
    config: &Config,
    reporter: &mut Reporter<R, W>,
) -> TResult<()> {
    // reporter.mode(ModeIntent::Show);

    let crate_folder = crate_root_folder(config)?;
    let cargo_toml = crate_folder.join("Cargo.toml");

    let contents = std::fs::read_to_string(&cargo_toml).map_err(|error| CargoMSRVError::Io {
        error,
        source: IoErrorSource::ReadFile(cargo_toml),
    })?;

    let manifest = CargoManifestParser::default().parse::<Document>(&contents)?;
    let manifest = CargoManifest::try_from(manifest)?;

    let msrv = manifest.minimum_rust_version();
    if msrv.is_some() {
        // reporter.finish_success(
        //     ModeIntent::Show,
        //     msrv.map(BareVersion::to_semver_version).as_ref(),
        // );
    } else {
        // reporter.finish_failure(ModeIntent::Show, None);
    }

    Ok(())
}
