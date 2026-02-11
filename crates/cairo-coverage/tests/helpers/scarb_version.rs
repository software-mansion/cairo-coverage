use semver::Version;
use snapbox::cmd::Command as SnapboxCommand;
use std::process::Output;

pub fn scarb_version() -> Version {
    let output = run_command();
    let stdout = get_stdout(&output);
    parse_version(&stdout)
}

fn run_command() -> Output {
    SnapboxCommand::new("scarb")
        .arg("-V")
        .output()
        .expect("failed to execute 'scarb -V'")
}

fn get_stdout(output: &Output) -> String {
    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("unknown error");
        panic!("scarb -V failed: {stderr}");
    }

    str::from_utf8(&output.stdout)
        .expect("scarb -V output was not valid UTF-8")
        .trim()
        .to_string()
}

fn parse_version(stdout: &str) -> Version {
    let version_str = stdout
        .split_whitespace()
        .nth(1)
        .expect("Unexpected output format from 'scarb -V', expected 'scarb x.y.z (hash..., date)'");

    Version::parse(version_str)
        .unwrap_or_else(|_| panic!("failed to parse '{version_str}' into semver"))
}
