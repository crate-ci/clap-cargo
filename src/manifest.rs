//! Cargo flag for selecting the relevant crate.

use std::path;

#[derive(Default, Clone, Debug, PartialEq, Eq, clap::Args)]
#[non_exhaustive]
pub struct Manifest {
    #[clap(long, name = "PATH", parse(from_os_str))]
    /// Path to Cargo.toml
    pub manifest_path: Option<path::PathBuf>,
}

#[cfg(feature = "cargo_metadata")]
impl Manifest {
    /// Create a `cargo_metadata::MetadataCommand`
    ///
    /// Note: Requires the features `cargo_metadata`.
    pub fn metadata(&self) -> cargo_metadata::MetadataCommand {
        let mut c = cargo_metadata::MetadataCommand::new();
        if let Some(ref manifest_path) = self.manifest_path {
            c.manifest_path(manifest_path);
        }
        c
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_app() {
        #[derive(Debug, clap::StructOpt)]
        struct Cli {
            #[clap(flatten)]
            manifest: Manifest,
        }

        use clap::CommandFactory;
        Cli::command().debug_assert()
    }

    #[cfg(feature = "cargo_metadata")]
    #[test]
    fn metadata_with_path() {
        let manifest = Manifest {
            manifest_path: Some(path::PathBuf::from("tests/fixtures/simple/Cargo.toml")),
        };
        let metadata = manifest.metadata();
        metadata.exec().unwrap();
        // TODO verify we forwarded correctly.
    }

    #[cfg(feature = "cargo_metadata")]
    #[test]
    fn metadata_without_path() {
        let cwd = path::PathBuf::from("tests/fixtures/simple");
        let manifest = Manifest {
            manifest_path: None,
        };
        let mut metadata = manifest.metadata();
        metadata.current_dir(cwd).exec().unwrap();
        // TODO verify we forwarded correctly.
    }
}
