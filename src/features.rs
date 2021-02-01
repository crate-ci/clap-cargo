//! Cargo Feature Flags.

#[derive(Default, Clone, Debug, PartialEq, Eq, structopt::StructOpt)]
#[non_exhaustive]
pub struct Features {
    #[structopt(long)]
    /// Activate all available features
    pub all_features: bool,
    #[structopt(long)]
    /// Do not activate the `default` feature
    pub no_default_features: bool,
    #[structopt(long)]
    /// Space-separated list of features to activate
    pub features: Vec<String>,
}

#[cfg(feature = "cargo_metadata")]
impl Features {
    /// Forward these flags to the `cargo_metadata` crate.
    ///
    /// Note: Requires the features `cargo_metadata`.
    pub fn forward_metadata<'m>(
        &self,
        meta: &'m mut cargo_metadata::MetadataCommand,
    ) -> &'m mut cargo_metadata::MetadataCommand {
        if self.all_features {
            meta.features(cargo_metadata::CargoOpt::AllFeatures);
        }
        if self.no_default_features {
            meta.features(cargo_metadata::CargoOpt::NoDefaultFeatures);
        }
        if !self.features.is_empty() {
            meta.features(cargo_metadata::CargoOpt::SomeFeatures(
                self.features.clone(),
            ));
        }
        meta
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "cargo_metadata")]
    #[test]
    fn features_all() {
        let mut metadata = cargo_metadata::MetadataCommand::new();
        metadata.manifest_path("tests/fixtures/simple/Cargo.toml");

        let features = Features {
            all_features: true,
            ..Default::default()
        };
        features.forward_metadata(&mut metadata);
        metadata.exec().unwrap();
        // TODO verify we forwarded correctly.
    }

    #[cfg(feature = "cargo_metadata")]
    #[test]
    fn features_none() {
        let mut metadata = cargo_metadata::MetadataCommand::new();
        metadata.manifest_path("tests/fixtures/simple/Cargo.toml");

        let features = Features {
            no_default_features: true,
            ..Default::default()
        };
        features.forward_metadata(&mut metadata);
        metadata.exec().unwrap();
        // TODO verify we forwarded correctly.
    }
}
