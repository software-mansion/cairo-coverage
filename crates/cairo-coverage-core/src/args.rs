/// Options accepted by `cairo_coverage_core` `run` function.
#[derive(Default, Clone)]
pub struct RunOptions {
    /// Include additional components in the coverage report.
    pub include: Vec<IncludedComponent>,

    /// If set, the hit count of the lines will not be truncated to 1.
    pub no_truncation: bool,
}

/// Additional components that can be included in the coverage report.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum IncludedComponent {
    /// Run coverage on functions marked with `#[test]` attribute
    TestFunctions,
    /// Run coverage on macros and generated code by them. This includes inline macros, attribute macros, and derive macros.
    Macros,
}
