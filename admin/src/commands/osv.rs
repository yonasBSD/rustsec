//! `rustsec-admin osv` subcommand
//!
//! Exports all advisories to the OSV format defined at
//! <https://github.com/google/osv>

use std::{
    path::{Path, PathBuf},
    process::exit,
};

use abscissa_core::{Command, Runnable, status_err};
use clap::Parser;

use crate::osv_export::OsvExporter;

#[derive(Command, Debug, Default, Parser)]
pub struct OsvCmd {
    /// Path to the advisory database
    #[arg(
        long = "db",
        help = "filesystem path to the RustSec advisory DB git repo"
    )]
    repo_path: Option<PathBuf>,
    /// Path to the output directory
    #[arg(help = "filesystem directory where OSV JSON files will be written")]
    path: Option<PathBuf>,
}

impl Runnable for OsvCmd {
    fn run(&self) {
        let out_path = match &self.path {
            None => Path::new("."),
            Some(path) => path,
        };

        let repo_path = self.repo_path.as_deref();
        let exporter = OsvExporter::new(repo_path).unwrap_or_else(|e| {
            status_err!("Failed to fetch the advisory database: {}", e);
            exit(1);
        });
        exporter.export_all(out_path).unwrap_or_else(|e| {
            status_err!("failed not export to '{}': {}", out_path.display(), e);
            exit(1);
        });
    }
}
