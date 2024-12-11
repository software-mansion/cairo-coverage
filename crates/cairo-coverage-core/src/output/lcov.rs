use crate::coverage_data::{
    FileCoverageData, FileCoverageDataOps, FilesCoverageData, FunctionCoverageData,
    FunctionCoverageDataOps,
};
use crate::types::HitCount;
use cairo_annotations::annotations::coverage::{LineNumber, SourceFileFullPath};
use cairo_annotations::annotations::profiler::FunctionName;
use derived_deref::Deref;
use itertools::Itertools;
use std::fmt;
use std::fmt::Display;

#[derive(Deref)]
pub struct LcovFormat(Vec<(SourceFileFullPath, LcovData)>);

pub struct LcovData {
    lines: Vec<(LineNumber, HitCount)>,
    file_hit_count: HitCount,
    unique_file_hit_count: HitCount,
    functions: Vec<LcovDetails>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct LcovDetails {
    name: FunctionName,
    starts_at: LineNumber,
    hit_count: HitCount,
}

impl From<FilesCoverageData> for LcovFormat {
    fn from(files_coverage_data: FilesCoverageData) -> Self {
        Self(
            files_coverage_data
                .iter()
                .map(|(source_file_full_path, file_coverage_data)| {
                    (source_file_full_path.to_owned(), file_coverage_data.into())
                })
                .sorted_by(
                    |(source_file_full_path, _), (other_source_file_full_path, _)| {
                        source_file_full_path.cmp(other_source_file_full_path)
                    },
                )
                .collect(),
        )
    }
}

impl From<&FileCoverageData> for LcovData {
    fn from(file_coverage_data: &FileCoverageData) -> Self {
        let lines = file_coverage_data.lines().into_iter().sorted().collect();

        let functions = file_coverage_data
            .iter()
            .map(LcovDetails::from)
            .sorted()
            .collect();

        let file_hit_count = file_coverage_data.file_hit_count();
        let unique_file_hit_count = file_coverage_data.unique_file_hit_count();

        Self {
            lines,
            file_hit_count,
            unique_file_hit_count,
            functions,
        }
    }
}

impl From<(&FunctionName, &FunctionCoverageData)> for LcovDetails {
    fn from((name, function_coverage_data): (&FunctionName, &FunctionCoverageData)) -> Self {
        Self {
            name: name.to_owned(),
            starts_at: function_coverage_data.starts_at(),
            hit_count: function_coverage_data.hit_count(),
        }
    }
}

impl Display for LcovFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (source_file_full_path, functions) in self.iter() {
            writeln!(f, "TN:")?;
            writeln!(f, "SF:{source_file_full_path}")?;
            write!(f, "{functions}")?;
        }
        Ok(())
    }
}

impl Display for LcovData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for function in &self.functions {
            writeln!(f, "FN:{},{}", function.starts_at, function.name)?;
            writeln!(f, "FNDA:{},{}", function.hit_count, function.name)?;
        }

        writeln!(f, "FNF:{}", self.functions.len())?;
        writeln!(f, "FNH:{}", self.file_hit_count)?;

        for (line_number, hit_count) in &self.lines {
            writeln!(f, "DA:{line_number},{hit_count}")?;
        }

        writeln!(f, "LF:{}", self.lines.len())?;
        writeln!(f, "LH:{}", self.unique_file_hit_count)?;
        writeln!(f, "end_of_record")?;
        Ok(())
    }
}
