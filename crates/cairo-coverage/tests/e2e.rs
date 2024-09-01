mod helpers;

use crate::helpers::run_test_project;
use indoc::indoc;
use snapbox::cmd::{cargo_bin, Command as SnapboxCommand};
use std::env;

#[test]
fn simple() {
    let output = run_test_project("simple").unwrap();
    assert_eq!(
        output,
        indoc! {
        "
        TN:
        SF:tests/data/simple/src/lib.cairo
        FN:7,8,simple::increase_by_one
        FNDA:4,simple::increase_by_one
        FN:2,3,simple::increase_by_two
        FNDA:3,simple::increase_by_two
        FNF:2
        FNH:2
        DA:2,1
        DA:3,3
        DA:7,2
        DA:8,4
        LF:4
        LH:4
        end_of_record
        TN:
        SF:tests/data/simple/tests/test_call.cairo
        FN:2,2,simple_tests::test_call::my_test
        FNDA:4,simple_tests::test_call::my_test
        FNF:1
        FNH:1
        DA:2,4
        LF:1
        LH:1
        end_of_record
        "
        }
    );
}

#[test]
fn scarb_template() {
    let output = run_test_project("scarb_template").unwrap();
    assert_eq!(
        output,
        indoc! {
        "
        TN:
        SF:tests/data/scarb_template/src/lib.cairo
        FN:5,11,scarb_template::fib
        FNDA:34,scarb_template::fib
        FN:20,20,scarb_template::tests::it_works
        FNDA:3,scarb_template::tests::it_works
        FNF:2
        FNH:2
        DA:5,1
        DA:8,34
        DA:9,32
        DA:11,32
        DA:20,3
        LF:5
        LH:5
        end_of_record
        "
        }
    );
}

#[test]
#[ignore] // TODO: Fix in #26
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
