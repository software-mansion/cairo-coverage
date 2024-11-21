use crate::helpers::TestProject;
use assert_fs::fixture::PathChild;

#[test]
fn simple() {
    TestProject::new("simple")
        .run()
        .output_same_as_in_file("simple.lcov");
}

#[test]
fn simple_with_tests() {
    TestProject::new("simple")
        .coverage_args(&["--include", "test-functions"])
        .run()
        .output_same_as_in_file("simple_with_tests.lcov");
}

#[test]
fn simple_with_output_path() {
    assert!(TestProject::new("simple")
        .coverage_args(&["--output-path", "custom_output.lcov"])
        .run()
        .dir()
        .child("custom_output.lcov")
        .exists());
}

#[test]
fn scarb_template() {
    TestProject::new("scarb_template")
        .run()
        .output_same_as_in_file("scarb_template.lcov");
}

#[test]
fn complex_calculator() {
    TestProject::new("complex_calculator")
        .run()
        .output_same_as_in_file("complex_calculator.lcov");
}

#[test]
fn readme_example() {
    // If you ever find yourself in a situation where you need to change the expected output,
    // please update the lcov.md files as well.
    TestProject::new("readme_example")
        .run()
        .output_same_as_in_file("readme_example.lcov");
}

#[test]
fn macros() {
    TestProject::new("macros")
        .coverage_args(&["--include", "macros"])
        .run()
        .output_same_as_in_file("macros.lcov");
}

#[test]
fn macros_not_included() {
    TestProject::new("macros")
        .run_without_genhtml()
        .assert_empty_output();
}

#[test]
fn snforge_template() {
    TestProject::new("snforge_template")
        .run()
        .output_same_as_in_file("snforge_template.lcov");
}
