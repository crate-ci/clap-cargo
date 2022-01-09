# clap-cargo

> Re-usable CLI flags for `cargo` plugins

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
#[derive(Debug, clap::Parser)]
struct Cli {
    #[clap(flatten)]
    manifest: clap_cargo::Manifest,
    #[clap(flatten)]
    workspace: clap_cargo::Workspace,
    #[clap(flatten)]
    features: clap_cargo::Features,
}
```

## Limitations

* [Argument methods][argument_methods] support with required clap(flatten) - see clap [derive reference][derive_reference]

[argument_methods]: https://docs.rs/clap/latest/clap/struct.Arg.html#implementations
[derive_reference]: https://github.com/clap-rs/clap/blob/master/examples/derive_ref/README.md

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
