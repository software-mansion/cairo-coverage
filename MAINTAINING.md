# `cairo-coverage` Maintenance

## Maintenance Procedure

To maintain `cairo-coverage` effectively, ensure compatibility with the latest versions of **Scarb** and **Starknet
Foundry**.

### Compatibility Checks

Historically, breakages often occur due to updates in Scarb, as Foundry includes its own tests for Scarb and trace
generation. When a new version of Scarb is released, verify that `cairo-coverage` remains compatible by updating the CI
configuration. You can
reference [this PR](https://github.com/software-mansion/cairo-coverage/pull/217/changes) for an
example of how to implement these checks.

### Troubleshooting

If the tests for a new scarb version fail, the issue is usually related to CASM compilation. To resolve this:

Upgrade the `cairo-lang` dependencies to the latest version. If this fixes the issue, release a new version of
`cairo-coverage` with support for the latest Cairo compiler.

## Release Procedure

To release a new version of `cairo-coverage`, follow these steps:

> Note: It is recommended to run benchmarks before releasing to ensure that the new version does not introduce any
> performance regressions. You can refer to the [benchmarking documentation](./BENCHMARKING.md) for instructions on how to
> run benchmarks

1. **Prepare a Pull Request (PR)**:
    - Ensure the correct version is updated in the following files:
        - `Cargo.toml`
        - `Cargo.lock`
        - `CHANGELOG.md`
    - As a reference, you can check this [PR](https://github.com/software-mansion/cairo-coverage/pull/116/files).

2. **Run the Release Action**:
    - Trigger the [release workflow](https://github.com/software-mansion/cairo-coverage/actions/workflows/release.yml)
      from the branch you want to release.
    - It is recommended to run this from the `main` branch.
    - Ensure that the commit you are releasing from has passed CI checks, as this is not automatically verified.
    - The workflow will create a tag, generate a new GitHub release with built binaries, and include the contents of
      `CHANGELOG.md`.

3. **Announce the Release**:
    - Notify the community about the update on **Twitter, Telegram, and Discord**.