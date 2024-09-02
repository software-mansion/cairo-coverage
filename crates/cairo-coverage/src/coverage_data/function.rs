use crate::data_loader::LineRange;
use crate::input::{InputData, SierraToCairoMap};
use crate::types::{FileLocation, FunctionName, HitCount, LineNumber};
use std::collections::HashMap;

pub type FilesCoverageData = HashMap<FileLocation, FileCoverageData>;

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

pub trait FileCoverageDataOps {
    fn file_hit_count(&self) -> HitCount;
    fn lines(&self) -> HashMap<LineNumber, HitCount>;

    fn unique_file_hit_count(&self) -> HitCount;
}

impl FileCoverageDataOps for FileCoverageData {
    fn file_hit_count(&self) -> HitCount {
        self.values()
            .map(FunctionCoverageData::hit)
            .filter(|hit| *hit)
            .count()
    }

    fn lines(&self) -> HashMap<LineNumber, HitCount> {
        self.values()
            .flat_map(|details| details.iter().map(|(&line, &hits)| (line, hits)))
            .collect()
    }

    fn unique_file_hit_count(&self) -> HitCount {
        self.lines().values().filter(|hit| **hit > 0).count()
    }
}

pub type FunctionCoverageData = HashMap<LineNumber, HitCount>;

pub trait FunctionCoverageDataOps {
    fn hit(&self) -> bool;
    fn hit_count(&self) -> HitCount;
    fn starts_at(&self) -> LineNumber;
    fn ends_at(&self) -> LineNumber;
}

impl FunctionCoverageDataOps for FunctionCoverageData {
    fn hit(&self) -> bool {
        self.values().any(|hit| *hit > 0)
    }

    fn hit_count(&self) -> HitCount {
        self.values().max().copied().unwrap_or_default()
    }

    fn starts_at(&self) -> LineNumber {
        self.keys().min().copied().unwrap_or_default()
    }

    fn ends_at(&self) -> LineNumber {
        self.keys().max().copied().unwrap_or_default()
    }
}

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
