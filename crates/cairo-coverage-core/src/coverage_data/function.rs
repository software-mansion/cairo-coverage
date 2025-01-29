use crate::build::coverage_input::CoverageInput;
use crate::build::statement_information::StatementInformationMap;
use crate::types::HitCount;
use cairo_annotations::annotations::coverage::{LineNumber, SourceFileFullPath};
use cairo_annotations::annotations::profiler::FunctionName;
use itertools::Itertools;
use std::collections::HashMap;

pub type FilesCoverageData = HashMap<SourceFileFullPath, FileCoverageData>;

pub fn create_files_coverage_data_with_hits(coverage_input: &CoverageInput) -> FilesCoverageData {
    coverage_input
        .statement_information_map
        .iter()
        .filter(|(id, _)| coverage_input.executed_statement_count.contains_key(id))
        .fold(
            create_files_coverage_data(&coverage_input.statement_information_map),
            |mut files_coverage_data, (id, statement_origin)| {
                if let Some(function_details) = files_coverage_data
                    .get_mut(&statement_origin.source_file_full_path)
                    .and_then(|functions| functions.get_mut(&statement_origin.function_name))
                {
                    function_details.register_hit(
                        &statement_origin.line_range,
                        coverage_input.executed_statement_count[id],
                    );
                }
                files_coverage_data
            },
        )
}

fn create_files_coverage_data(
    statement_information_map: &StatementInformationMap,
) -> FilesCoverageData {
    statement_information_map
        .values()
        .cloned()
        .fold(HashMap::new(), |mut acc, origin| {
            acc.entry(origin.source_file_full_path)
                .or_default()
                .entry(origin.function_name)
                .or_default()
                .register_line(&origin.line_range);
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
            .into_grouping_map()
            .fold(0, |hits1, _, hits2| hits1 + hits2)
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
}

trait Register {
    fn register_line(&mut self, lines: impl IntoIterator<Item = LineNumber>);
    fn register_hit(&mut self, lines: impl IntoIterator<Item = LineNumber>, times: HitCount);
}

impl Register for FunctionCoverageData {
    fn register_line(&mut self, lines: impl IntoIterator<Item = LineNumber>) {
        for line in lines {
            self.entry(line).or_default();
        }
    }

    fn register_hit(&mut self, lines: impl IntoIterator<Item = LineNumber>, times: HitCount) {
        for line in lines {
            *self.entry(line).or_default() += times;
        }
    }
}
