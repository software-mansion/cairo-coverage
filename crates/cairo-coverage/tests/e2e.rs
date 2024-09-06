mod helpers;

use crate::helpers::{run_test_project, run_test_project_with_args};
use indoc::{formatdoc, indoc};
use snapbox::cmd::{cargo_bin, Command as SnapboxCommand};
use std::env;

#[test]
fn simple() {
    let (dir, output) = run_test_project("simple").unwrap();
    assert_eq!(
        output,
        formatdoc! {
        "
        TN:
        SF:{dir}/src/lib.cairo
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
        ",
        dir = dir
        }
    );
}

#[test]
fn simple_with_tests() {
    let (dir, output) =
        run_test_project_with_args("simple", &["--include-test-functions"]).unwrap();
    assert_eq!(
        output,
        formatdoc! {
        "
        TN:
        SF:{dir}/src/lib.cairo
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
        SF:{dir}/tests/test_call.cairo
        FN:2,2,simple_tests::test_call::my_test
        FNDA:4,simple_tests::test_call::my_test
        FNF:1
        FNH:1
        DA:2,4
        LF:1
        LH:1
        end_of_record
        ",
        dir = dir
        }
    );
}

#[test]
fn scarb_template() {
    let (dir, output) = run_test_project("scarb_template").unwrap();
    assert_eq!(
        output,
        formatdoc! {
        "
        TN:
        SF:{dir}/src/lib.cairo
        FN:5,11,scarb_template::fib
        FNDA:34,scarb_template::fib
        FNF:1
        FNH:1
        DA:5,1
        DA:8,34
        DA:9,32
        DA:11,32
        LF:4
        LH:4
        end_of_record
        ",
        dir = dir
        }
    );
}

#[test]
fn complex_calculator() {
    let (dir, output) = run_test_project("complex_calculator").unwrap();
    assert_eq!(
        output,
        formatdoc! {
        "
        TN:
        SF:{dir}/src/lib.cairo
        FN:2,2,complex_calculator::add
        FNDA:2,complex_calculator::add
        FN:17,21,complex_calculator::divide
        FNDA:1,complex_calculator::divide
        FN:25,30,complex_calculator::factorial
        FNDA:12,complex_calculator::factorial
        FN:45,49,complex_calculator::fibonacci
        FNDA:2,complex_calculator::fibonacci
        FN:53,63,complex_calculator::is_prime
        FNDA:84,complex_calculator::is_prime
        FN:10,10,complex_calculator::multiply
        FNDA:2,complex_calculator::multiply
        FN:35,40,complex_calculator::power
        FNDA:10,complex_calculator::power
        FN:6,6,complex_calculator::subtract
        FNDA:2,complex_calculator::subtract
        FN:14,14,complex_calculator::unsafe_divide
        FNDA:0,complex_calculator::unsafe_divide
        FNF:9
        FNH:8
        DA:2,2
        DA:6,2
        DA:10,2
        DA:14,0
        DA:17,1
        DA:18,1
        DA:19,1
        DA:21,0
        DA:25,1
        DA:28,12
        DA:29,10
        DA:30,10
        DA:35,2
        DA:38,10
        DA:39,6
        DA:40,6
        DA:45,2
        DA:46,1
        DA:47,0
        DA:48,1
        DA:49,0
        DA:53,2
        DA:54,4
        DA:55,0
        DA:58,0
        DA:59,84
        DA:60,80
        DA:61,0
        DA:63,80
        LF:29
        LH:22
        end_of_record
        ",
        dir = dir
        }
    );
}

#[test]
fn readme_example() {
    let (dir, output) = run_test_project("readme_example").unwrap();

    // If you ever find yourself in a situation where you need to change the expected output,
    // please update the lcov.md files as well.
    assert_eq!(
        output,
        formatdoc! {
        "
        TN:
        SF:{dir}/src/lib.cairo
        FN:8,8,readme_example::add
        FNDA:4,readme_example::add
        FN:16,18,readme_example::calculator
        FNDA:4,readme_example::calculator
        FN:12,12,readme_example::multiply
        FNDA:0,readme_example::multiply
        FNF:3
        FNH:2
        DA:8,4
        DA:12,0
        DA:16,2
        DA:17,4
        DA:18,0
        LF:5
        LH:3
        end_of_record
        ",
        dir = dir
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
