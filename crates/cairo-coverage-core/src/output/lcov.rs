use crate::coverage::file::{FileCoverage, FileCoverageOperations};
use crate::coverage::function::FunctionCoverageOperations;
use crate::coverage::project::ProjectCoverage;
use crate::hashmap_utils::stable_iter::{IntoStableIter, StableIter};
use cairo_annotations::annotations::coverage::SourceFileFullPath;
use std::fmt;

/// Formats coverage data in the LCOV format as a string.
pub fn fmt_string(project_coverage: &ProjectCoverage) -> String {
    let mut buf = String::new();
    LcovFormatter::new(&mut buf)
        .fmt(project_coverage)
        .unwrap_or_else(|_| unreachable!("formatting to a string should never fail"));
    buf
}

/// Formats coverage data in the LCOV format to a writer.
struct LcovFormatter<T: fmt::Write> {
    writer: T,
}

impl<T> LcovFormatter<T>
where
    T: fmt::Write,
{
    /// Creates a new [`LcovFormatter`] that writes to the given writer.
    fn new(writer: T) -> Self {
        Self { writer }
    }

    /// Formats the coverage data in the LCOV format.
    fn fmt(&mut self, project_coverage: &ProjectCoverage) -> fmt::Result {
        for (source_file_full_path, coverage_by_function) in project_coverage.stable_iter() {
            self.general_information(source_file_full_path)?;
            self.function_details(coverage_by_function)?;
            self.function_summary(coverage_by_function)?;
            self.line_execution(coverage_by_function)?;
            self.end_of_record()?;
        }
        Ok(())
    }

    /// Writes the general:
    /// - TN(Test Name): accepted to be empty in the LCOV format
    /// - SF(Source File): source file
    fn general_information(&mut self, source_file_full_path: &SourceFileFullPath) -> fmt::Result {
        writeln!(self.writer, "TN:")?;
        writeln!(self.writer, "SF:{source_file_full_path}")
    }

    /// Writes the function details:
    /// - FN(Function Name): line at which function start, function name
    /// - FNDA(Function Data): how many times function was executed, function name
    fn function_details(&mut self, file_coverage: &FileCoverage) -> fmt::Result {
        for (name, by_line) in file_coverage.stable_iter() {
            writeln!(self.writer, "FN:{},{}", by_line.starts_at(), name)?;
            writeln!(
                self.writer,
                "FNDA:{},{}",
                by_line.max_execution_count(),
                name
            )?;
        }

        Ok(())
    }

    /// Writes the function summary:
    /// - FNF(Functions Found): number of functions found
    /// - FNH(Functions Hit): number of functions hit (executed)
    fn function_summary(&mut self, file_coverage: &FileCoverage) -> fmt::Result {
        writeln!(self.writer, "FNF:{}", file_coverage.len())?;
        writeln!(self.writer, "FNH:{}", file_coverage.executed_functions())
    }

    /// Writes the line execution:
    /// - DA(Line Data): line number, execution count
    /// - LF (Lines Found): number of lines found
    /// - LH(Lines Hit): number of lines hit
    fn line_execution(&mut self, coverage_by_function: &FileCoverage) -> fmt::Result {
        let lines = coverage_by_function
            .flatten()
            .into_stable_iter()
            .collect::<Vec<_>>();

        for (line_number, execution_count) in &lines {
            writeln!(self.writer, "DA:{line_number},{execution_count}")?;
        }

        writeln!(self.writer, "LF:{}", lines.len())?;
        writeln!(self.writer, "LH:{}", coverage_by_function.executed_lines())
    }

    /// Writes the end of record marker.
    pub fn end_of_record(&mut self) -> fmt::Result {
        writeln!(self.writer, "end_of_record")
    }
}
