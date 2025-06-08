# cairo-coverage

`cairo-coverage` is a utility designed to generate coverage reports for code written in the Cairo programming language.

> ⚠️ **IMPORTANT**:
> Please note that this repository is actively being developed and is currently in an alpha release stage.
> If you encounter any issues, please report them to us via the issues
> tab on our GitHub [repository](https://github.com/software-mansion/cairo-coverage).
>
> We currently don't support:
> - Branch coverage
>
> Things that might not work as expected:
> - Counters for how many times line was executed

## Installation

You can install `cairo-coverage` using [asdf](https://asdf-vm.com/guide/getting-started.html)
or the installation script.

### asdf (recommended):

```shell
asdf plugin add cairo-coverage
asdf install cairo-coverage latest  # Replace 'latest' with a specific version number if needed
```

Please remember to set global/project version to use:

```shell
asdf global cairo-coverage latest
# or
asdf local cairo-coverage latest
```

### script:

To install the latest stable version of `cairo-coverage`, run:

```shell
curl -L https://raw.githubusercontent.com/software-mansion/cairo-coverage/main/scripts/install.sh | sh
```

If you want to install a specific version, run the following command with the requested version:

```shell
curl -L https://raw.githubusercontent.com/software-mansion/cairo-coverage/main/scripts/install.sh | sh -s -- v0.1.0
```

### Installation on Windows

We do not provide a Windows binary for `cairo-coverage`. 
We recommend using the Windows Subsystem for Linux (WSL) to run `cairo-coverage` on Windows.

## Integrated tools

- [x] [Starknet Foundry](https://github.com/foundry-rs/starknet-foundry) - check how to use it
  with `cairo-coverage` [here](https://foundry-rs.github.io/starknet-foundry/testing/coverage.html)
- [ ] Cairo Test

## Usage

### Help

To see the available commands and options, run:

```shell
cairo-coverage --help
```

Using the `--help` flag with any command will display additional information about that specific command.

```shell
cairo-coverage clean  --help
```

### Coverage Across Different Scarb Versions

`cairo-coverage` relies heavily on `scarb` and the internal workings of the `cairo` compiler, which can lead to variations in behavior depending on the `scarb` version used.

To ensure consistency in coverage reports across different versions of `scarb`, we have categorized features into **stable** and **unstable**:

- **Stable features** provide consistent results across all versions of `scarb`.
- **Unstable features** may produce different results depending on the `scarb` version.

A feature is considered **stable** if it produces the same results across all minor versions of `scarb` from `2.8.*` onward, using the latest patch version.

To enable unstable features, use the `--unstable` flag.

### `.cairo-coverage-ignore` File

You can create a `.cairo-coverage-ignore` file in the root of your project to specify the files or directories that you
want to exclude from the coverage report. The file should contain a list of paths to be ignored, with one path per line,
similar to a `.gitignore` file.

In addition to specific file or directory names, **you can also use regular expressions ** in this file to define
more flexible or dynamic patterns for ignored paths. This can be especially useful for excluding files with certain
extensions, names, or those in specific structural patterns.

For example:

```gitignore
# Ignore test_contract.cairo
test_contract.cairo

# Ignore all test files
test_*.cairo

# Ignore everything in the utils directory
*/utils/*
```

### Generate Coverage Report

To generate a coverage report, run the `cairo-coverage run` command with one or more `<PATH_TO_TRACE_DATA>` arguments. These
arguments specify the paths to the JSON files containing the trace data to be used for generating the coverage report.

```shell
cairo-coverage run path/to/trace/1.json path/to/trace/2.json path/to/trace/3.json
```

The generated output file is in the `lcov` format. For your convenience, you can find an explanation along with a simple
example of the `lcov` format [here](./lcov.md).

#### Using `snforge`:

Please refer to
the [Starknet Foundry documentation](https://foundry-rs.github.io/starknet-foundry/testing/coverage.html) for additional
information on using `cairo-coverage` with `snforge`.

### Viewing Report

Before you can view the coverage report as an HTML file, **the report must first be generated**. Please refer to
the [Generate Coverage Report](#generate-coverage-report) section above for detailed instructions.

Once you have generated the `coverage.lcov` file, a summary report with aggregated data can be produced by one of the
many tools that accept the `lcov` format.

In this example, we will use the `genhtml` tool from
the [lcov package](https://github.com/linux-test-project/lcov/tree/master) to generate an HTML report. If you don’t
already have `genhtml` installed, you can find installation instructions [here](https://command-not-found.com/genhtml).

```shell
genhtml -o coverage_report coverage.lcov
```

You can now open the `index.html` file in the `coverage_report` directory to see the generated coverage report.

## Usage in GitHub actions

A variety of GitHub actions are available for analyzing coverage data for continuous integration purposes, which can
accept input in the form of an lcov file.
Examples of such actions include [CodeCov](https://github.com/codecov/codecov-action)
and [Coveralls](https://github.com/coverallsapp/github-action).

The example workflow below illustrates how to use coverage report generated by `snforge` in conjunction with `CodeCov`:

```yaml
name: Example cairo coverage workflow
on:
  pull_request:
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: foundry-rs/setup-snfoundry@v3

      - name: Install cairo-coverage
        run: curl -L https://raw.githubusercontent.com/software-mansion/cairo-coverage/main/scripts/install.sh | sh

      - name: Run tests and generate report
        run: snforge test --coverage

      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v4
        with:
          file: ./coverage.lcov
          token: ${{ secrets.CODECOV_TOKEN }}
```

## External tools integration

`cairo-coverage` is tool-agnostic which means that it accepts input from any tool. However, these tools need to generate
trace data in a specific expected format -
the same format which is accepted by the [cairo-profiler](https://github.com/software-mansion/cairo-profiler/tree/main).
For the exact code implementation of this format, please refer
to [this page](https://github.com/software-mansion/cairo-profiler/blob/main/crates/trace-data/src/lib.rs).

## Getting Help

Join the [Telegram](https://t.me/cairo_coverage) group to get help

Found a bug? Open an [issue](https://github.com/software-mansion/cairo-coverage/issues/new).