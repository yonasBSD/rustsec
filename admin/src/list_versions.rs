//! Backend for the `list-affected-versions` subcommand.

use std::path::PathBuf;

use rustsec::{Advisory, Database};
use tame_index::index::RemoteGitIndex;

use crate::{error::Error, lock::acquire_cargo_package_lock, prelude::*};

/// Lists all versions for a crate and prints info on which ones are affected
pub struct AffectedVersionLister {
    /// Loaded crates.io index
    crates_index: RemoteGitIndex,

    /// Loaded Advisory DB
    advisory_db: Database,
}

impl AffectedVersionLister {
    /// Load the database at the given path
    pub fn new(repo_path: impl Into<PathBuf>) -> Result<Self, Error> {
        let repo_path = repo_path.into();
        let lock = acquire_cargo_package_lock()?;
        let mut crates_index = RemoteGitIndex::new(
            tame_index::GitIndex::new(tame_index::IndexLocation::new(
                tame_index::IndexUrl::CratesIoGit,
            ))?,
            &lock,
        )?;
        crates_index.fetch(&lock)?;
        let advisory_db = Database::open(&repo_path)?;
        Ok(Self {
            crates_index,
            advisory_db,
        })
    }

    /// Borrow the loaded advisory database
    pub fn advisory_db(&self) -> &Database {
        &self.advisory_db
    }

    /// List affected and unaffected crate versions for a given advisory
    pub fn process_one_advisory(&self, advisory: &Advisory) {
        status_ok!(
            "Loaded",
            "{} for '{}'",
            advisory.id(),
            advisory.metadata.package
        );
        let crate_name = advisory.metadata.package.as_str();
        let crate_info = self
            .crates_index
            .krate(
                crate_name.try_into().unwrap(),
                true,
                &acquire_cargo_package_lock().unwrap(),
            )
            .unwrap()
            .unwrap_or_else(|| panic!("expected crate {crate_name} to exist"));
        for version in crate_info.versions {
            let parsed_version = rustsec::Version::parse(&version.version).unwrap();
            if advisory.versions.is_vulnerable(&parsed_version) {
                println!("{} vulnerable", version.version)
            } else {
                println!("{} OK", version.version)
            }
        }
    }

    /// List affected and unaffected crate versions for all advisories
    pub fn process_all_advisories(&self) -> Result<(), Error> {
        for advisory in self.advisory_db.iter() {
            // We currently only support crate versions, not advisories against Rust versions
            if advisory.metadata.collection.unwrap() != rustsec::Collection::Crates {
                continue;
            }
            self.process_one_advisory(advisory);
        }
        Ok(())
    }
}
