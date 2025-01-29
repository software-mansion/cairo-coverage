use crate::coverage::function::{ExecutionCount, FunctionCoverage, FunctionCoverageOperations};
use cairo_annotations::annotations::profiler::FunctionName;
use itertools::Itertools;
use std::collections::HashMap;

/// A mapping of function names to the [`FunctionCoverage`]
/// This is used to represent the coverage of a single file.
pub type FileCoverage = HashMap<FunctionName, FunctionCoverage>;

/// Operations that can be performed on a [`FileCoverage`].
pub trait FileCoverageOperations {
    /// Returns the number of functions that were executed.
    fn executed_functions(&self) -> ExecutionCount;
    /// Flattens the coverage data to a single [`FunctionCoverage`].
    /// This is useful for calculating the total coverage of a file.
    fn flatten(&self) -> FunctionCoverage;
    /// Returns the number of lines that were executed.
    fn executed_lines(&self) -> ExecutionCount;
}

impl FileCoverageOperations for FileCoverage {
    fn executed_functions(&self) -> ExecutionCount {
        self.values()
            .map(FunctionCoverage::was_executed)
            .filter(|was_executed| *was_executed)
            .count()
    }

    fn flatten(&self) -> FunctionCoverage {
        self.values()
            .flat_map(|details| {
                details
                    .iter()
                    .map(|(&line, &execution_count)| (line, execution_count))
            })
            .into_grouping_map()
            .fold(0, |execution_count1, _, execution_count2| {
                execution_count1 + execution_count2
            })
    }

    fn executed_lines(&self) -> ExecutionCount {
        self.flatten()
            .values()
            .filter(|execution_count| **execution_count > 0)
            .count()
    }
}
