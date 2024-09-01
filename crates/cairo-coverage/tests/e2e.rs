mod helpers;

use crate::helpers::run_test_project;
use indoc::indoc;
use snapbox::cmd::{cargo_bin, Command as SnapboxCommand};
use std::env;

#[test]
#[ignore] // Fixed in nexts PRs
fn simple() {
    let output = run_test_project("simple").unwrap();
    assert_eq!(
        output,
        indoc! {
        "
        "
        }
    );
}

#[test]
#[ignore] // Fixed in nexts PRs
fn scarb_template() {
    let output = run_test_project("scarb_template").unwrap();
    assert_eq!(
        output,
        indoc! {
        "
        "
        }
    );
}

#[test]
#[ignore] // Fixed in nexts PRs
fn complex_calculator() {
    let output = run_test_project("complex_calculator").unwrap();
    assert_eq!(
        output,
        indoc! {
        "
        "
        }
    );
}

#[test]
fn no_trace_files_provided() {
    SnapboxCommand::new(cargo_bin!("cairo-coverage"))
        .assert()
        .failure()
        .stderr_eq(indoc! {
            "error: the following required arguments were not provided:
              <TRACE_FILES>...

            Usage: cairo-coverage <TRACE_FILES>...

            For more information, try '--help'.
            "
        });
}
