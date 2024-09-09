use crate::helpers::TestProject;

#[test]
fn simple() {
    TestProject::run("simple").output_same_as_in_file("simple.lcov");
}

#[test]
fn simple_with_tests() {
    TestProject::run_with_args("simple", &["--include-test-functions"])
        .output_same_as_in_file("simple_with_tests.lcov");
}

#[test]
fn scarb_template() {
    TestProject::run("scarb_template").output_same_as_in_file("scarb_template.lcov");
}

#[test]
fn complex_calculator() {
    TestProject::run("complex_calculator").output_same_as_in_file("complex_calculator.lcov");
}

#[test]
fn readme_example() {
    // If you ever find yourself in a situation where you need to change the expected output,
    // please update the lcov.md files as well.
    TestProject::run("readme_example").output_same_as_in_file("readme_example.lcov");
}
