# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

#### Breaking Changes

- Upgrade `cargo_metadata`

## [0.8.0] - 2021-12-31

#### Breaking Changes

- Upgraded to clap3

## [0.7.0] - 2021-11-15

#### Breaking Changes

- If you already have a `-p` short flag, this will break

#### Fixes

- Add `-p` short flag
- Use `SPEC` as `--package` / `--exclude` value name

## [0.6.1] - 2021-08-12

#### Fixes

- `--exclude` was broken by the previous change

## [0.6.0] - 2021-08-12

#### Fixes

- `--package` is now an opt-in to match `cargo`s behavior

## [0.5.0] - 2021-07-31

#### Breaking Changes

- Updated to cargo_metadata 0.14

## [0.4.1] - 2021-07-31

#### Fixes

- With `Workspace::package`, `Workspace::exclude`, and `Features::feature` we
  were accidentally accepting multiple values (`--feature foo bar`) rather than
  multiple occurrences (`--feature foo --feature bar`) or delimited values
  (`--feature "foo bar"`).

## [0.4.0] - 2021-02-01

#### Breaking Changes

* cargo_metadata 0.12 is now required

## [0.3.1] - 2020-05-11

#### Fixes

* Workaround structopt bug causing documentation to override help strings

## 0.3.0 (2019-12022)

#### Breaking Changes

* cargo_metadata 0.9 is now required


## 0.2.0 (2019-09-12)

#### Features

* `--workspace` has been added with `--all` acting as an alias.

#### Breaking Changes

* structopt 0.3 is now required
* cargo_metadata 0.8 is now required
* Structs are now non-exhaustive


## 0.1.4 (2019-05-23)


#### Bug Fixes

*   Argument names were missing ([c444c5cc](https://github.com/crate-ci/clap-cargo/commit/c444c5cc019f08c6f2e619e166344f548531b8f6))


<!-- next-url -->
[Unreleased]: https://github.com/crate-ci/clap-cargo/compare/v0.8.0...HEAD
[0.8.0]: https://github.com/crate-ci/clap-cargo/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/crate-ci/clap-cargo/compare/v0.6.1...v0.7.0
[0.6.1]: https://github.com/crate-ci/clap-cargo/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/crate-ci/clap-cargo/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/crate-ci/clap-cargo/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/crate-ci/clap-cargo/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/crate-ci/clap-cargo/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/crate-ci/clap-cargo/compare/v0.3.0...v0.3.1
