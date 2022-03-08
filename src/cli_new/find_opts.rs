use crate::cli_new::custom_check_opts::CustomCheckOpts;
use crate::cli_new::rust_releases_opts::RustReleasesOpts;
use crate::cli_new::toolchain_opts::ToolchainOpts;
use clap::AppSettings;
use clap::Args;

// Cli Options for top-level cargo-msrv (find) command
#[derive(Debug, Args)]
#[clap(next_help_heading = "FIND MSRV OPTIONS", setting = AppSettings::DeriveDisplayOrder)]
pub struct FindOpts {
    /// Use a binary search to find the MSRV (default)
    ///
    /// When the search space is sufficiently large, which is common, this is much
    /// faster than a linear search. A binary search will approximately halve the search
    /// space for each Rust version checked for compatibility.
    #[clap(long, conflicts_with = "linear")]
    pub bisect: bool,

    /// Use a linear search to find the MSRV
    ///
    /// This method checks toolchain from the most recent release to the earliest.
    #[clap(long, conflicts_with = "bisect")]
    pub linear: bool,

    /// Output a rust-toolchain file with the MSRV as toolchain
    ///
    /// The toolchain file will pin the Rust version for this crate.
    /// See https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file for more.
    #[clap(long)]
    pub toolchain_file: bool,

    /// Temporarily remove the lockfile, so it will not interfere with the building process
    ///
    /// This is important when testing against older Rust versions such as Cargo versions prior to
    /// Rust 1.38.0, for which Cargo does not recognize the newer lockfile formats.
    #[clap(long)]
    pub ignore_lockfile: bool,

    /// Don't read the `edition` of the crate and do not use its value to reduce the search space
    #[clap(long)]
    pub no_read_min_edition: bool,

    /// Don't print the result of compatibility checks
    ///
    /// The feedback of a compatibility check can be useful to determine why a certain Rust
    /// version is not compatible. Rust usually prints very detailed error messages.
    /// While most often very useful, in some cases they may be too noisy or lengthy.
    /// If this flag is given, the result messages will not be printed.
    #[clap(long)]
    pub no_check_feedback: bool,

    #[clap(flatten)]
    pub rust_releases_opts: RustReleasesOpts,

    #[clap(flatten)]
    pub toolchain_opts: ToolchainOpts,

    #[clap(flatten)]
    pub custom_check_opts: CustomCheckOpts,
}
