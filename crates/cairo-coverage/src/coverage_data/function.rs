use crate::input::{InputData, LineRange, SierraToCairoMap};
use crate::types::{FileLocation, FunctionName, HitCount, LineNumber};
use std::collections::HashMap;

pub type FilesCoverageData = HashMap<FileLocation, FileCoverageData>;

#[allow(dead_code)]
pub fn create_files_coverage_data_with_hits(input_data: &InputData) -> FilesCoverageData {
    input_data
        .sierra_to_cairo_map
        .iter()
        .filter(|(id, _)| input_data.unique_executed_sierra_ids.contains_key(id))
        .fold(
            create_files_coverage_data(&input_data.sierra_to_cairo_map),
            |mut files_coverage_data, (id, statement_origin)| {
                if let Some(function_details) = files_coverage_data
                    .get_mut(&statement_origin.file_location)
                    .and_then(|functions| functions.get_mut(&statement_origin.function_name))
                {
                    function_details.register_hit(
                        &statement_origin.line_range,
                        input_data.unique_executed_sierra_ids[id],
                    );
                }
                files_coverage_data
            },
        )
}

fn create_files_coverage_data(sierra_to_cairo_map: &SierraToCairoMap) -> FilesCoverageData {
    sierra_to_cairo_map
        .values()
        .cloned()
        .fold(HashMap::new(), |mut acc, origin| {
            acc.entry(origin.file_location)
                .or_default()
                .entry(origin.function_name)
                .and_modify(|function_details: &mut FunctionCoverageData| {
                    function_details.register_line(&origin.line_range);
                })
                .or_insert(origin.line_range.into());
            acc
        })
}

pub type FileCoverageData = HashMap<FunctionName, FunctionCoverageData>;

pub type FunctionCoverageData = HashMap<LineNumber, HitCount>;

impl From<LineRange> for FunctionCoverageData {
    fn from(line_range: LineRange) -> Self {
        let mut function_coverage_data = FunctionCoverageData::new();
        function_coverage_data.register_line(&line_range);
        function_coverage_data
    }
}

trait Register {
    fn register_line<T: IntoIterator<Item = LineNumber>>(&mut self, lines: T);
    fn register_hit<T: IntoIterator<Item = LineNumber>>(&mut self, lines: T, times: HitCount);
}

impl Register for FunctionCoverageData {
    fn register_line<T: IntoIterator<Item = LineNumber>>(&mut self, lines: T) {
        for line in lines {
            self.entry(line).or_default();
        }
    }

    fn register_hit<T: IntoIterator<Item = LineNumber>>(&mut self, lines: T, times: HitCount) {
        for line in lines {
            *self.entry(line).or_default() += times;
        }
    }
}
