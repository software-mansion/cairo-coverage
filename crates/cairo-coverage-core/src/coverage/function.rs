use cairo_annotations::annotations::coverage::LineNumber;
use std::collections::HashMap;

/// A mapping of line numbers to the number of times they were executed
/// This is used to represent the coverage of a single function.
pub type FunctionCoverage = HashMap<LineNumber, ExecutionCount>;

pub type ExecutionCount = usize;

/// Operations that can be performed on a [`FunctionCoverage`].
pub trait FunctionCoverageOperations {
    /// Returns `true` if function was executed.
    fn was_executed(&self) -> bool;
    /// Returns the maximum number of times a line was executed in this function.
    fn max_execution_count(&self) -> ExecutionCount;
    /// Returns the line number where the function starts.
    fn starts_at(&self) -> LineNumber;
}

impl FunctionCoverageOperations for FunctionCoverage {
    fn was_executed(&self) -> bool {
        self.values().any(|execution_count| *execution_count > 0)
    }
    fn max_execution_count(&self) -> ExecutionCount {
        self.values().max().copied().unwrap_or_default()
    }
    fn starts_at(&self) -> LineNumber {
        self.keys().min().copied().unwrap_or_default()
    }
}
