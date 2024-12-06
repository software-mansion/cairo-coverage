use crate::helpers::TestProject;

#[test]
fn coverage_ignore_dir() {
    TestProject::new("coverage_ignore")
        .create_cairo_coverage_ignore("*/simple/*")
        .run()
        .output_same_as_in_file("coverage_ignore_dir.lcov");
}

#[test]
fn coverage_ignore_files2() {
    TestProject::new("coverage_ignore")
        .create_cairo_coverage_ignore("simple_*.cairo")
        .run()
        .output_same_as_in_file("coverage_ignore_dir.lcov");
}

#[test]
fn coverage_ignore_files() {
    TestProject::new("coverage_ignore")
        .create_cairo_coverage_ignore("multiply.cairo\nsimple_add.cairo")
        .run()
        .output_same_as_in_file("coverage_ignore_file.lcov");
}

#[test]
fn coverage_ignore_file_does_not_exists() {
    TestProject::new("coverage_ignore")
        .create_cairo_coverage_ignore("multiply.cairo\nsimple_add.cairo\nnot_existing.cairo")
        .run()
        .output_same_as_in_file("coverage_ignore_file.lcov");
}
