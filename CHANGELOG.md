# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

#### Fixed

- Fixed project inference to work with upcoming `snforge` `0.34.0`

## [0.2.0] - 2024-09-17

#### Added

- Support for contracts
- Option to not include macros in coverage report. To get the same behavior as before use `--include macros`
- `--project-path` flag to specify the path to the project root directory. This useful when inference fails

#### Fixed

- Bug where hit count was not correctly calculated for functions declared at same line
- Functions from imported projects are no longer included in report when `SCARB_CACHE` is set

#### Changed

- `--include-test-functions` was remove in favor of `--include`. To get same behavior as before
  use `--include tests-functions`
- Only the start of the function is now included in the report. The end of the function is optional in the lcov format
  and would produce a warning in tools like `genhtml` when two or more functions are declared on the same line
