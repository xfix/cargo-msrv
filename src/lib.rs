#![deny(clippy::all)]
#![allow(clippy::upper_case_acronyms, clippy::unnecessary_wraps)]

#[macro_use]
extern crate tracing;

#[cfg(feature = "rust-releases-dist-source")]
use rust_releases::RustDist;
use rust_releases::{semver, Channel, FetchResources, ReleaseIndex, RustChangelog, Source};
use std::io::Write;

use crate::config::{Config, ModeIntent, ReleaseSource};
use crate::errors::{CargoMSRVError, TResult};
use crate::reporter::{Output, ProgressAction};

use crate::reporter_new::events::FetchingIndex;
use crate::reporter_new::{events, Report, ReportType, Reporter};
use crate::subcommands::list::run_list_msrv;
use crate::subcommands::set::run_set_msrv;
use crate::subcommands::show::run_show_msrv;
pub use crate::{
    result::MinimalCompatibility, subcommands::find::find_msrv,
    subcommands::find::run_find_msrv_action, subcommands::verify::run_verify_msrv_action,
};

pub mod check;
pub mod cli_new;
pub(crate) mod command;
pub mod config;
pub(crate) mod dependencies;
pub(crate) mod download;
pub mod errors;
pub mod exit_code;
pub(crate) mod fetch;
pub(crate) mod formatter;
pub(crate) mod lockfile;
pub(crate) mod log_level;
pub(crate) mod manifest;
pub(crate) mod outcome;
pub(crate) mod paths;
pub(crate) mod releases;
pub mod reporter;
pub mod reporter_new;
pub(crate) mod result;
pub(crate) mod search_methods;
pub(crate) mod subcommands;
pub(crate) mod toolchain;
pub(crate) mod toolchain_file;

pub fn run_app<R: ReportType, W: Write>(
    config: &Config,
    reporter: &mut Reporter<R, W>,
) -> TResult<()>
where
    FetchingIndex: events::Event<R>,
{
    reporter.report_event(FetchingIndex);
    let index = fetch_index(config)?;

    run_action(config, &index, reporter)
}

fn fetch_index(config: &Config) -> TResult<ReleaseIndex> {
    let source = config.release_source();

    info!(
        source = Into::<&'static str>::into(source),
        "fetching index"
    );

    let index = match config.release_source() {
        ReleaseSource::RustChangelog => {
            RustChangelog::fetch_channel(Channel::Stable)?.build_index()?
        }
        #[cfg(feature = "rust-releases-dist-source")]
        ReleaseSource::RustDist => RustDist::fetch_channel(Channel::Stable)?.build_index()?,
    };

    Ok(index)
}

fn run_action<R: ReportType, W: Write>(
    config: &Config,
    index: &ReleaseIndex,
    reporter: &mut Reporter<R, W>,
) -> TResult<()> {
    let action = config.action_intent();

    info!(
        action = Into::<&'static str>::into(action),
        "running action"
    );

    match action {
        // ModeIntent::Find => run_find_msrv_action(config, reporter, index),
        // ModeIntent::Verify => run_verify_msrv_action(config, reporter, index),
        // ModeIntent::List => run_list_msrv(config, reporter),
        // ModeIntent::Set => run_set_msrv(config, reporter),
        ModeIntent::Show => run_show_msrv(config, reporter),
    };

    Ok(())
}
