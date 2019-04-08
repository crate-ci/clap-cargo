use std::collections;

/// Cargo flags for selecting crates in a workspace.
#[derive(Default, Clone, Debug, PartialEq, Eq, structopt::StructOpt)]
pub struct Workspace {
    #[structopt()]
    /// Package to process (see `cargo help pkgid`)
    pub package: Vec<String>,
    #[structopt()]
    /// Process all packages in the workspace
    pub all: bool,
    #[structopt()]
    /// Exclude packages from being processed
    pub exclude: Vec<String>,
}

#[cfg(feature="cargo_metadata")]
impl Workspace {
    /// Partition workspace members into those selected and those excluded.
    ///
    /// Notes:
    /// - Requires the features `cargo_metadata`.
    /// - Requires not calling `MetadataCommand::no_deps`
    pub fn partition_packages<'m>(&self, meta: &'m cargo_metadata::Metadata) -> (Vec<&'m cargo_metadata::Package>, Vec<&'m cargo_metadata::Package>) {
        let workspace_members: collections::HashSet<_> = meta.workspace_members.iter().collect();

        let resolve = meta.resolve.as_ref().expect("no-deps is unsupported");
        let all = self.all || resolve.root.is_none();
        let base_ids: collections::HashSet<_> = if all {
            workspace_members.clone()
        } else {
            let mut base_ids = collections::HashSet::new();
            base_ids.insert(resolve.root.as_ref().expect("`root` is always present outside of `--all`."));
            base_ids
        };

        // Probably unneeded optimization for when `all` and `package` are both set.
        let dummy = vec![];
        let packages = if all {
            &dummy
        } else {
            &self.package
        };

        meta.packages.iter()
            .filter(|p| workspace_members.contains(&p.id))
            .partition(|p| (base_ids.contains(&p.id) || packages.contains(&p.name)) && ! self.exclude.contains(&p.name))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature="cargo_metadata")]
    #[cfg(test)]
    mod partition_default {
        use super::*;

        #[test]
        fn single_crate() {
            let mut metadata = cargo_metadata::MetadataCommand::new();
            metadata.manifest_path("tests/fixtures/simple/Cargo.toml");
            let metadata = metadata.exec().unwrap();

            let workspace = Workspace {
                ..Default::default()
            };
            let (included, excluded) = workspace.partition_packages(&metadata);
            assert_eq!(included.len(), 1);
            assert_eq!(excluded.len(), 0);
        }

        #[test]
        fn mixed_ws_root() {
            let mut metadata = cargo_metadata::MetadataCommand::new();
            metadata.manifest_path("tests/fixtures/mixed_ws/Cargo.toml");
            let metadata = metadata.exec().unwrap();

            let workspace = Workspace {
                ..Default::default()
            };
            let (included, excluded) = workspace.partition_packages(&metadata);
            assert_eq!(included.len(), 1);
            assert_eq!(excluded.len(), 2);
        }

        #[test]
        fn mixed_ws_leaf() {
            let mut metadata = cargo_metadata::MetadataCommand::new();
            metadata.manifest_path("tests/fixtures/mixed_ws/c/Cargo.toml");
            let metadata = metadata.exec().unwrap();

            let workspace = Workspace {
                ..Default::default()
            };
            let (included, excluded) = workspace.partition_packages(&metadata);
            assert_eq!(included.len(), 1);
            assert_eq!(excluded.len(), 2);
        }

        #[test]
        fn pure_ws_root() {
            let mut metadata = cargo_metadata::MetadataCommand::new();
            metadata.manifest_path("tests/fixtures/pure_ws/Cargo.toml");
            let metadata = metadata.exec().unwrap();

            let workspace = Workspace {
                ..Default::default()
            };
            let (included, excluded) = workspace.partition_packages(&metadata);
            assert_eq!(included.len(), 3);
            assert_eq!(excluded.len(), 0);
        }

        #[test]
        fn pure_ws_leaf() {
            let mut metadata = cargo_metadata::MetadataCommand::new();
            metadata.manifest_path("tests/fixtures/pure_ws/c/Cargo.toml");
            let metadata = metadata.exec().unwrap();

            let workspace = Workspace {
                ..Default::default()
            };
            let (included, excluded) = workspace.partition_packages(&metadata);
            assert_eq!(included.len(), 1);
            assert_eq!(excluded.len(), 2);
        }
    }

    #[cfg(feature="cargo_metadata")]
    #[cfg(test)]
    mod partition_all {
        use super::*;

        #[test]
        fn single_crate() {
            let mut metadata = cargo_metadata::MetadataCommand::new();
            metadata.manifest_path("tests/fixtures/simple/Cargo.toml");
            let metadata = metadata.exec().unwrap();

            let workspace = Workspace {
                all: true,
                ..Default::default()
            };
            let (included, excluded) = workspace.partition_packages(&metadata);
            assert_eq!(included.len(), 1);
            assert_eq!(excluded.len(), 0);
        }

        #[test]
        fn mixed_ws_root() {
            let mut metadata = cargo_metadata::MetadataCommand::new();
            metadata.manifest_path("tests/fixtures/mixed_ws/Cargo.toml");
            let metadata = metadata.exec().unwrap();

            let workspace = Workspace {
                all: true,
                ..Default::default()
            };
            let (included, excluded) = workspace.partition_packages(&metadata);
            assert_eq!(included.len(), 3);
            assert_eq!(excluded.len(), 0);
        }

        #[test]
        fn mixed_ws_leaf() {
            let mut metadata = cargo_metadata::MetadataCommand::new();
            metadata.manifest_path("tests/fixtures/mixed_ws/c/Cargo.toml");
            let metadata = metadata.exec().unwrap();

            let workspace = Workspace {
                all: true,
                ..Default::default()
            };
            let (included, excluded) = workspace.partition_packages(&metadata);
            assert_eq!(included.len(), 3);
            assert_eq!(excluded.len(), 0);
        }

        #[test]
        fn pure_ws_root() {
            let mut metadata = cargo_metadata::MetadataCommand::new();
            metadata.manifest_path("tests/fixtures/pure_ws/Cargo.toml");
            let metadata = metadata.exec().unwrap();

            let workspace = Workspace {
                all: true,
                ..Default::default()
            };
            let (included, excluded) = workspace.partition_packages(&metadata);
            assert_eq!(included.len(), 3);
            assert_eq!(excluded.len(), 0);
        }

        #[test]
        fn pure_ws_leaf() {
            let mut metadata = cargo_metadata::MetadataCommand::new();
            metadata.manifest_path("tests/fixtures/pure_ws/c/Cargo.toml");
            let metadata = metadata.exec().unwrap();

            let workspace = Workspace {
                all: true,
                ..Default::default()
            };
            let (included, excluded) = workspace.partition_packages(&metadata);
            assert_eq!(included.len(), 3);
            assert_eq!(excluded.len(), 0);
        }
    }

    #[cfg(feature="cargo_metadata")]
    #[cfg(test)]
    mod partition_package {
        use super::*;

        #[test]
        fn single_crate() {
            let mut metadata = cargo_metadata::MetadataCommand::new();
            metadata.manifest_path("tests/fixtures/simple/Cargo.toml");
            let metadata = metadata.exec().unwrap();

            let workspace = Workspace {
                package: vec!["simple".to_owned()],
                ..Default::default()
            };
            let (included, excluded) = workspace.partition_packages(&metadata);
            assert_eq!(included.len(), 1);
            assert_eq!(excluded.len(), 0);
        }

        #[test]
        fn mixed_ws_root() {
            let mut metadata = cargo_metadata::MetadataCommand::new();
            metadata.manifest_path("tests/fixtures/mixed_ws/Cargo.toml");
            let metadata = metadata.exec().unwrap();

            let workspace = Workspace {
                package: vec!["a".to_owned()],
                ..Default::default()
            };
            let (included, excluded) = workspace.partition_packages(&metadata);
            assert_eq!(included.len(), 2);
            assert_eq!(excluded.len(), 1);
        }

        #[test]
        fn mixed_ws_leaf() {
            let mut metadata = cargo_metadata::MetadataCommand::new();
            metadata.manifest_path("tests/fixtures/mixed_ws/c/Cargo.toml");
            let metadata = metadata.exec().unwrap();

            let workspace = Workspace {
                package: vec!["a".to_owned()],
                ..Default::default()
            };
            let (included, excluded) = workspace.partition_packages(&metadata);
            assert_eq!(included.len(), 2);
            assert_eq!(excluded.len(), 1);
        }

        #[test]
        fn pure_ws_root() {
            let mut metadata = cargo_metadata::MetadataCommand::new();
            metadata.manifest_path("tests/fixtures/pure_ws/Cargo.toml");
            let metadata = metadata.exec().unwrap();

            let workspace = Workspace {
                package: vec!["a".to_owned()],
                ..Default::default()
            };
            let (included, excluded) = workspace.partition_packages(&metadata);
            assert_eq!(included.len(), 3);
            assert_eq!(excluded.len(), 0);
        }

        #[test]
        fn pure_ws_leaf() {
            let mut metadata = cargo_metadata::MetadataCommand::new();
            metadata.manifest_path("tests/fixtures/pure_ws/c/Cargo.toml");
            let metadata = metadata.exec().unwrap();

            let workspace = Workspace {
                package: vec!["a".to_owned()],
                ..Default::default()
            };
            let (included, excluded) = workspace.partition_packages(&metadata);
            assert_eq!(included.len(), 2);
            assert_eq!(excluded.len(), 1);
        }
    }
}
