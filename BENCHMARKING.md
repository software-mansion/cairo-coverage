# Benchmarking

This document provides instructions on how to set up and run benchmarks for the `cairo-coverage` project.

## Prerequisites

Before running benchmarks, ensure you have the following installed:

- Rust and Cargo: Follow the official Rust installation guide.
- `asdf` version manager: Follow the official `asdf` installation guide.
- `scarb` plugin for `asdf`:
  ```bash
  asdf plugin add scarb
  ```
- `starknet-foundry` plugin for `asdf`:
  ```bash
  asdf plugin add starknet-foundry
  ```

## Setting up Benchmark Projects

The benchmarks rely on trace data generated from specific Cairo projects. The `download_bench_projects.sh` script is
used to set up these projects and generate the necessary trace data.

1. Navigate to the `benches` directory:
   ```bash
   cd crates/cairo-coverage-core/benches
   ```

2. Run the `download_bench_projects.sh` script:
   ```bash
   ./download_bench_projects.sh
   ```
   This script contains the `create_traces` function, which is responsible for preparing a project for benchmarking. The
   `create_traces` function takes four arguments:
    - `<origin-link>`: The Git URL of the repository containing the Cairo project.
    - `<commit-hash>`: The specific commit hash of the repository to check out.
    - `<starknet-foundry-version>`: The `starknet-foundry` version to use for the project.
    - `<scarb-version>`: The `scarb` version to use for the project.

   For each call to `create_traces`, the script will:
    - Initialize and fetch the specified Git repository.
    - Check out the given commit.
    - Create a `.tool-versions` file within the cloned project directory to ensure consistent `starknet-foundry` and
      `scarb` versions using `asdf`.
    - Run `snforge test --save-trace-data` to execute the project's tests and generate execution trace data.
    - Copy the generated trace data into a `project-traces` directory, which the benchmarks will then use.

   **Adding New Projects for Benchmarking**:
   To add a new project to be benchmarked, you need to:
    - Add a new call to the `create_traces` function in `download_bench_projects.sh`.
    - Provide the `origin-link`, `commit-hash`, `starknet-foundry-version`, and `scarb-version` relevant to your new
      project.
    - After running `download_bench_projects.sh`, you will also need to add a corresponding benchmark function in
      `crates/cairo-coverage-core/benches/benchmarks/` (e.g., `my_new_project.rs`) and register it in
      `crates/cairo-coverage-core/benches/bench_main.rs`.

   **Example of adding a new project**:
   If you wanted to add a project from `https://github.com/example/my-cairo-project.git` at commit `abcdef123456`, using
   `starknet-foundry` version `0.55.0` and `scarb` version `2.13.0`, you would add the following line to
   `download_bench_projects.sh`:
   ```bash
   create_traces "https://github.com/example/my-cairo-project.git" "abcdef123456" 0.55.0 2.13.0
   ```

## Running Benchmarks

Once the benchmark projects are set up and trace data is generated, you can run the benchmarks using Cargo.

Run the benchmarks using `cargo bench`:

```bash
cargo bench --workspace --bench bench_main
```

This command will execute the benchmarks defined in `crates/cairo-coverage-core/benches/bench_main.rs` and its modules (
e.g., `starknet_staking.rs`).

Criterion will output the benchmark results to your console and also generate an HTML report which can be found in
`target/criterion/report/index.html`.