use crate::types::{FileLocation, FunctionName, LineNumber};
use cairo_lang_sierra::program::StatementIdx;
use serde::Deserialize;
use std::collections::HashMap;
use std::ops::RangeInclusive;

pub type CodeLocation = (FileLocation, LineRange);

#[derive(Deserialize, Clone, Eq, PartialEq)]
pub struct LineRange {
    pub start: Position,
    pub end: Position,
}

impl IntoIterator for &LineRange {
    type Item = LineNumber;
    type IntoIter = RangeInclusive<LineNumber>;

    fn into_iter(self) -> Self::IntoIter {
        self.start.line..=self.end.line
    }
}

#[derive(Deserialize, Clone, Eq, PartialEq)]
pub struct Position {
    pub line: LineNumber,
}

#[derive(Deserialize)]
pub struct CoverageAnnotations {
    pub statements_code_locations: HashMap<StatementIdx, Vec<CodeLocation>>,
}

#[derive(Deserialize)]
pub struct ProfilerAnnotations {
    pub statements_functions: HashMap<StatementIdx, Vec<FunctionName>>,
}
