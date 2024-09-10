use crate::coverage_data::{
    FileCoverageData, FileCoverageDataOps, FilesCoverageData, FunctionCoverageData,
    FunctionCoverageDataOps,
};
use crate::types::{FileLocation, FunctionName, HitCount, LineNumber};
use derived_deref::Deref;
use std::fmt;
use std::fmt::Display;

#[derive(Deref)]
pub struct LcovFormat(Vec<(FileLocation, LcovData)>);

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
        let mut list: Vec<_> = files_coverage_data
            .iter()
            .map(|(file_location, file_coverage_data)| {
                (file_location.to_owned(), file_coverage_data.into())
            })
            .collect();
        list.sort_by_key(|(file_location, _)| file_location.to_owned());
        Self(list)
    }
}

impl From<&FileCoverageData> for LcovData {
    fn from(file_coverage_data: &FileCoverageData) -> Self {
        let mut lines: Vec<_> = file_coverage_data.lines().into_iter().collect();
        lines.sort_unstable();

        let mut functions: Vec<_> = file_coverage_data.iter().map(LcovDetails::from).collect();
        functions.sort();

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
        for (file_location, functions) in self.iter() {
            writeln!(f, "TN:")?;
            writeln!(f, "SF:{file_location}")?;
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
