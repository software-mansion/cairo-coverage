use camino::Utf8PathBuf;

/// Options accepted by `cairo_coverage_core` `run` function.
#[derive(Default, Clone)]
pub struct RunOptions {
    /// Include additional components in the coverage report.
    pub include: Vec<IncludedComponent>,

    /// Path to the project directory. If not provided, the project directory is inferred from the trace.
    pub project_path: Option<Utf8PathBuf>,
}

/// Additional components that can be included in the coverage report.
#[derive(Clone, Copy)]
pub enum IncludedComponent {
    /// Run coverage on functions marked with `#[test]` attribute
    TestFunctions,
    /// Run coverage on macros and generated code by them. This includes inline macros, attribute macros, and derive macros.
    Macros,
}
