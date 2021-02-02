//! **clap-cargo**: Re-usable CLI flags for `cargo` plugins
//!
//! ## Install
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! clap-cargo = "0.1"
//! ```
//!
//! ## Examples
//!
//! ```rust
//! // ...
//! #[derive(Debug, structopt::StructOpt)]
//! struct Cli {
//!     #[structopt(flatten)]
//!     manifest: clap_cargo::Manifest,
//!     #[structopt(flatten)]
//!     workspace: clap_cargo::Workspace,
//!     #[structopt(flatten)]
//!     features: clap_cargo::Features,
//! }
//! ```
//!
//! ## Relevant crates
//!
//! Other crates that might be useful for cargo plugins:
//! * [escargot][escargot] for wrapping `cargo-build`, `carg-run`, `cargo-test`, etc.
//! * [cargo_metadata][cargo_metadata] for getting crate information.
//! * [clap-verbosity][clap-verbosity] for adding logging to your CLI.
//!
//! [escargot]: https://crates.io/crates/escargot
//! [cargo_metadata]: https://crates.io/crates/cargo_metadata
//! [clap-verbosity]: https://crates.io/crates/clap-verbosity-flag

#![warn(missing_debug_implementations)]
#![warn(unused_extern_crates)]

mod features;
mod manifest;
mod workspace;

pub use features::*;
pub use manifest::*;
pub use workspace::*;

#[macro_use]
extern crate doc_comment;
doc_comment! {
    include_str!("../README.md")
}
