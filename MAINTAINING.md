# `cairo-coverage` Maintenance

## Release Procedure

To release a new version of `cairo-coverage`, follow these steps:

1. **Prepare a Pull Request (PR)**:
    - Ensure the correct version is updated in the following files:
        - `Cargo.toml`
        - `Cargo.lock`
        - `CHANGELOG.md`
    - As a reference, you can check this [PR](https://github.com/software-mansion/cairo-coverage/pull/116/files).

2. **Run the Release Action**:
    - Trigger the [release workflow](https://github.com/software-mansion/cairo-coverage/actions/workflows/release.yml) from the branch you want to release.
    - It is recommended to run this from the `main` branch.
    - Ensure that the commit you are releasing from has passed CI checks, as this is not automatically verified.
    - The workflow will create a tag, generate a new GitHub release with built binaries, and include the contents of `CHANGELOG.md`.

3. **Announce the Release**:
    - Notify the community about the update on **Twitter, Telegram, and Discord**.