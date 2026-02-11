use crate::helpers::{TestProject, scarb_version};
use assert_fs::fixture::PathChild;
use semver::Version;

#[test]
fn simple() {
    TestProject::new("simple")
        .run()
        .output_same_as_in_file("simple.lcov");
}

#[test]
fn simple_with_output_path() {
    assert!(
        TestProject::new("simple")
            .coverage_args(&["--output-path", "custom_output.lcov"])
            .run()
            .dir()
            .child("custom_output.lcov")
            .exists()
    );
}

#[test]
fn scarb_template() {
    TestProject::new("scarb_template")
        .run()
        .output_same_as_in_file("scarb_template.lcov");
}

#[test]
#[ignore = "instability (rework test)"]
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
        .run()
        .output_same_as_in_file("macros.lcov");
}

#[test]
#[cfg(feature = "allows-excluding-macros")]
fn macros_not_included() {
    TestProject::new("macros")
        .coverage_args(&["--unstable", "--include"])
        .run_without_genhtml()
        .assert_empty_output();
}

#[test]
fn snforge_template() {
    let file = if scarb_version() >= Version::new(2, 15, 0) {
        // In cairo 2.15.0 `#[starknet::contract]` attribute generates different code.
        // Hence, we have different expected output for scarb 2.15.0 and above.
        "snforge_template-scarb-2.15.lcov"
    } else {
        "snforge_template.lcov"
    };
    TestProject::new("snforge_template")
        .run()
        .output_same_as_in_file(file);
}

#[test]
#[cfg(feature = "allows-excluding-macros")]
fn snforge_template_macros_not_included() {
    TestProject::new("snforge_template")
        .coverage_args(&["--unstable", "--include"])
        .run()
        .output_same_as_in_file("snforge_template_macros_not_included.lcov");
}
