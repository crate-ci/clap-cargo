# clap-cargo

> Re-usable CLI flags for `cargo` plugins

[![Build Status](https://dev.azure.com/crate-ci/crate-ci/_apis/build/status/clap-cargo?branchName=master)](https://dev.azure.com/crate-ci/crate-ci/_build/latest?definitionId=6&branchName=master)
[![codecov](https://codecov.io/gh/crate-ci/clap-cargo/branch/master/graph/badge.svg)](https://codecov.io/gh/crate-ci/clap-cargo)
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/clap-cargo.svg)
[![Crates Status](https://img.shields.io/crates/v/clap-cargo.svg)](https://crates.io/crates/clap-cargo)

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
clap-cargo = "0.1"
```

## Examples

```rust
// ...
#[derive(Debug, structopt::StructOpt)]
struct Cli {
    #[structopt(flatten)]
    manifest: clap_cargo::Manifest,
    #[structopt(flatten)]
    workspace: clap_cargo::Workspace,
    #[structopt(flatten)]
    features: clap_cargo::Features,
}
```

## Relevant crates

Other crates that might be useful for cargo plugins:
* [escargot][escargot] for wrapping `cargo-build`, `carg-run`, `cargo-test`, etc.
* [cargo_metadata][cargo_metadata] for getting crate information.
* [clap-verbosity][clap-verbosity] for adding logging to your CLI.

[escargot]: https://crates.io/crates/escargot
[cargo_metadata]: https://crates.io/crates/cargo_metadata
[clap-verbosity]: https://crates.io/crates/clap-verbosity-flag

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[Crates.io]: https://crates.io/crates/clap-cargo
[Documentation]: https://docs.rs/clap-cargo
