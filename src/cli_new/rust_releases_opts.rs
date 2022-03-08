use crate::{semver, ReleaseSource};
use clap::AppSettings;
use clap::Args;
use std::str::FromStr;

#[derive(Debug, Args)]
#[clap(next_help_heading = "RUST RELEASES OPTIONS", setting = AppSettings::DeriveDisplayOrder)]
pub struct RustReleasesOpts {
    /// Least recent version or edition to take into account
    ///
    /// Given version must match a valid Rust toolchain, and be semver compatible,
    /// or match a Rust edition alias. For example, the edition alias "2018" would match
    /// Rust version `1.31.0`, since that's the first version which added support for the Rust
    /// 2018 edition.
    #[clap(long, value_name = "VERSION_SPEC or EDITION", alias = "minimum")]
    pub min: Option<EditionOrVersion>, // TODO: currently is semver::Version, instead of BareVersion

    /// Most recent version to take into account
    ///
    /// Given version must match a valid Rust toolchain, and be semver compatible.
    #[clap(long, value_name = "VERSION_SPEC", alias = "maximum")]
    pub max: Option<semver::Version>, // TODO: currently is semver::Version, instead of BareVersion

    /// Include all patch releases, instead of only the last
    #[clap(long)]
    pub include_all_patch_releases: bool,

    #[clap(long, possible_values = ReleaseSource::variants(), default_value_t, value_name = "SOURCE")]
    pub release_source: ReleaseSource,
}

#[derive(Debug)]
pub enum EditionOrVersion {
    Edition(Edition),
    Version(semver::Version),
}

impl EditionOrVersion {
    pub fn as_version(&self) -> semver::Version {
        match self {
            Self::Edition(edition) => edition.as_version(),
            Self::Version(version) => version.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Edition {
    Edition2015,
    Edition2018,
    Edition2021,
}

impl FromStr for Edition {
    type Err = ParseEditionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "2015" => Ok(Self::Edition2015),
            "2018" => Ok(Self::Edition2018),
            "2021" => Ok(Self::Edition2021),
            unknown => Err(ParseEditionError::UnknownEdition(unknown.to_string())),
        }
    }
}

impl Edition {
    pub fn as_version(&self) -> semver::Version {
        match self {
            Self::Edition2015 => semver::Version::new(1, 0, 0),
            Self::Edition2018 => semver::Version::new(1, 31, 0),
            Self::Edition2021 => semver::Version::new(1, 56, 0),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseEditionError {
    #[error("Edition '{0}' is not supported")]
    UnknownEdition(String),
}

impl FromStr for EditionOrVersion {
    type Err = ParseEditionOrVersionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input
            .parse::<Edition>()
            .map(EditionOrVersion::Edition)
            .or_else(|edition_err| {
                semver::Version::parse(input)
                    .map(EditionOrVersion::Version)
                    .map_err(|semver_err| {
                        ParseEditionOrVersionError::EditionOrVersion(
                            input.to_string(),
                            edition_err,
                            semver_err,
                        )
                    })
            })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseEditionOrVersionError {
    #[error("Value '{0}' could not be parsed as a valid Rust version: {1} + {2}")]
    EditionOrVersion(String, ParseEditionError, semver::Error),
}
