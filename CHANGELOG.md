# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [0.18.3] - 2025-10-22

### Features

- Expose `styles::CONTEXT` from `cargo info`

## [0.18.2] - 2025-10-20

### Fixes

- Add `testing_colors` feature for forcing colors to be the same on all platforms for test purposes

## [0.18.1] - 2025-10-16

### Fixes

- Update to Cargo 1.91 colors

## [0.18.0] - 2025-10-01

### Breaking Changes

- Upgraded `cargo_metadata` to 0.23

## [0.17.1] - 2025-09-08

### Documentation

- Show how to use terminal styling

## [0.17.0] - 2025-08-27

### Breaking Changes

- Upgraded `cargo_metadata` to 0.22

## [0.16.0] - 2025-07-12

### Breaking Changes

- Upgraded `cargo_metadata` to 0.21

### Compatibility

- Bump MSRV to 1.86

### Fixes

- Don't force long help on for `--help`

## [0.15.2] - 2025-01-14

## [0.15.1] - 2025-01-03

### Fix

- Reduce `Cargo.lock` content

## [0.15.0] - 2025-01-02

### Breaking Changes

- Upgraded `cargo_metadata` to 0.19

## [0.14.1] - 2024-07-25

### Compatibility

- MSRV bumped to 1.74

## [0.14.0] - 2024-02-06

### Compatibility

- Packages selected by default may change
- MSRV bumped to 1.72

### Fixes

- Use cargo's definition of `default-members` when partitioning packages

## [0.13.0] - 2023-10-02

### Breaking Changes

- Upgraded `cargo_metadata` to 0.18

## [0.12.0] - 2023-09-11

### Breaking Changes

- Move clap support behind `clap` feature flag

### Compatibility

- Update MSRV to 1.70.0

### Features

- Expose cargo's nightly styling

## [0.11.0] - 2023-08-01

### Breaking Changes

- Upgraded `cargo_metadata`

### Compatibility

- Raised MSRV to 1.66.0

## [0.10.0] - 2022-09-28

### Breaking CHanges

- Upgraded to clap v4

### Compatibility

- Raised MSRV to 1.60.0

## [0.9.1] - 2022-07-11

### Features

- `-F` short flag for `--features` to match cargo 1.62.0

## [0.9.0] - 2022-07-01

### Breaking Changes

- Upgrade `cargo_metadata`

## [0.8.0] - 2021-12-31

### Breaking Changes

- Upgraded to clap3

## [0.7.0] - 2021-11-15

### Breaking Changes

- If you already have a `-p` short flag, this will break

### Fixes

- Add `-p` short flag
- Use `SPEC` as `--package` / `--exclude` value name

## [0.6.1] - 2021-08-12

### Fixes

- `--exclude` was broken by the previous change

## [0.6.0] - 2021-08-12

### Fixes

- `--package` is now an opt-in to match `cargo`s behavior

## [0.5.0] - 2021-07-31

### Breaking Changes

- Updated to cargo_metadata 0.14

## [0.4.1] - 2021-07-31

### Fixes

- With `Workspace::package`, `Workspace::exclude`, and `Features::feature` we
  were accidentally accepting multiple values (`--feature foo bar`) rather than
  multiple occurrences (`--feature foo --feature bar`) or delimited values
  (`--feature "foo bar"`).

## [0.4.0] - 2021-02-01

### Breaking Changes

* cargo_metadata 0.12 is now required

## [0.3.1] - 2020-05-11

### Fixes

* Workaround structopt bug causing documentation to override help strings

## 0.3.0 (2019-12022)

### Breaking Changes

* cargo_metadata 0.9 is now required


## 0.2.0 (2019-09-12)

### Features

* `--workspace` has been added with `--all` acting as an alias.

### Breaking Changes

* structopt 0.3 is now required
* cargo_metadata 0.8 is now required
* Structs are now non-exhaustive


## 0.1.4 (2019-05-23)


### Bug Fixes

*   Argument names were missing ([c444c5cc](https://github.com/crate-ci/clap-cargo/commit/c444c5cc019f08c6f2e619e166344f548531b8f6))


<!-- next-url -->
[Unreleased]: https://github.com/crate-ci/clap-cargo/compare/v0.18.3...HEAD
[0.18.3]: https://github.com/crate-ci/clap-cargo/compare/v0.18.2...v0.18.3
[0.18.2]: https://github.com/crate-ci/clap-cargo/compare/v0.18.1...v0.18.2
[0.18.1]: https://github.com/crate-ci/clap-cargo/compare/v0.18.0...v0.18.1
[0.18.0]: https://github.com/crate-ci/clap-cargo/compare/v0.17.1...v0.18.0
[0.17.1]: https://github.com/crate-ci/clap-cargo/compare/v0.17.0...v0.17.1
[0.17.0]: https://github.com/crate-ci/clap-cargo/compare/v0.16.0...v0.17.0
[0.16.0]: https://github.com/crate-ci/clap-cargo/compare/v0.15.2...v0.16.0
[0.15.2]: https://github.com/crate-ci/clap-cargo/compare/v0.15.1...v0.15.2
[0.15.1]: https://github.com/crate-ci/clap-cargo/compare/v0.15.0...v0.15.1
[0.15.0]: https://github.com/crate-ci/clap-cargo/compare/v0.14.1...v0.15.0
[0.14.1]: https://github.com/crate-ci/clap-cargo/compare/v0.14.0...v0.14.1
[0.14.0]: https://github.com/crate-ci/clap-cargo/compare/v0.13.0...v0.14.0
[0.13.0]: https://github.com/crate-ci/clap-cargo/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/crate-ci/clap-cargo/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/crate-ci/clap-cargo/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/crate-ci/clap-cargo/compare/v0.9.1...v0.10.0
[0.9.1]: https://github.com/crate-ci/clap-cargo/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/crate-ci/clap-cargo/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/crate-ci/clap-cargo/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/crate-ci/clap-cargo/compare/v0.6.1...v0.7.0
[0.6.1]: https://github.com/crate-ci/clap-cargo/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/crate-ci/clap-cargo/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/crate-ci/clap-cargo/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/crate-ci/clap-cargo/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/crate-ci/clap-cargo/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/crate-ci/clap-cargo/compare/v0.3.0...v0.3.1
