# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2025-01-03

#### Added

- `cairo-coverage clean` command to remove all generated files. This is useful as by default, if a coverage report
  already exists, the new report is appended to it. So if you want to start fresh, you can use this command.

#### Changed

- `cairo-coverage` is now up to 2x faster. This is achieved by using multiple threads
- `cairo-coverage` must be called within a scarb-managed workspace as now it uses `scarb metadata` to infer the project
  root directory
- `.cairo-coverage-ignore` file now is only accepted in the project root directory

## [0.3.0] - 2024-12-09

#### Added

- `.cairo-coverage-ignore` file to exclude files or directories from the coverage report

## [0.3.0-rc0] - 2024-11-21

#### Fixed

- Fixed project inference to work with upcoming `snforge` `0.34.0`
- A lot of false negatives in the coverage report (your code has executed at this line but it is not marked as
  executed). `Note:` That this might remove some true positives/negatives (mark them as not executable). Please report
  any issues you find.

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
