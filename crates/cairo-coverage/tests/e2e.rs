use assert_fs::fixture::PathCopy;
use assert_fs::TempDir;
use indoc::indoc;
use snapbox::cmd::{cargo_bin, Command as SnapboxCommand};
use std::{env, fs};

#[test]
fn generating_visualisations() {
    let temp_dir = TempDir::new().unwrap();
    temp_dir
        .copy_from("./tests/data/test_project/", &["*.json", "*.cairo"])
        .unwrap();

    SnapboxCommand::new(cargo_bin!("cairo-coverage"))
        .current_dir(&temp_dir)
        .arg("snfoundry_trace/tests::test_call::my_test.json")
        .assert()
        .success();

    let output_path = temp_dir.join("./coverage.lcov");
    assert!(output_path.exists());

    SnapboxCommand::new("genhtml")
        .arg(output_path)
        .arg("--output-directory")
        .arg(temp_dir.path())
        .assert()
        .success();
}

#[test]
fn proper_output() {
    let temp_dir = TempDir::new().unwrap();
    temp_dir
        .copy_from("./tests/data/test_project/", &["*.json", "*.cairo"])
        .unwrap();

    let output_path = temp_dir.join("./coverage.lcov");

    SnapboxCommand::new(cargo_bin!("cairo-coverage"))
        .current_dir(&temp_dir)
        .arg("snfoundry_trace/tests::test_call::my_test.json")
        .assert()
        .success();

    let output = fs::read_to_string(output_path).unwrap();
    assert_eq!(
        output,
        indoc! {
        "
        TN:
        SF:tests/data/test_project/src/lib.cairo
        FN:7,8,test_project::increase_by_one
        FNDA:2,test_project::increase_by_one
        FN:2,3,test_project::increase_by_two
        FNDA:1,test_project::increase_by_two
        FNF:2
        FNH:2
        DA:2,1
        DA:3,1
        DA:7,2
        DA:8,1
        LF:4
        LH:4
        end_of_record
        TN:
        SF:tests/data/test_project/tests/test_call.cairo
        FN:5,6,tests::test_call::my_test
        FNDA:2,tests::test_call::my_test
        FNF:1
        FNH:1
        DA:5,2
        DA:6,1
        LF:2
        LH:2
        end_of_record
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
