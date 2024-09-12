# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

#### Added

- Option to not include macros in coverage report. To get the same behavior as before use `--include macros`.

#### Fixed

- Bug where hit count was not correctly calculated for functions declared at same line
- Functions from imported projects are no longer included in report when `SCARB_CACHE` is set

#### Changed

- `--include-test-functions` was remove in favor of `--include`. To get same behavior as before use `--include tests-functions`.