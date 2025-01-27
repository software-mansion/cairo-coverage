use crate::args::IncludedComponent;
use crate::build::filter::ignore_matcher::CairoCoverageIgnoreMatcher;
use crate::build::filter::libfuncs;
use crate::build::filter::libfuncs::NOT_RELIABLE_LIBFUNCS;
use crate::loading::enriched_program::EnrichedProgram;
use cairo_annotations::annotations::coverage::SourceFileFullPath;
use cairo_annotations::annotations::profiler::FunctionName;
use cairo_lang_sierra::program::StatementIdx;
use camino::Utf8PathBuf;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

/// Regex to match virtual files like `/path/to/project/lib.cairo[array_inline_macro][assert_macro]`
/// where `array_inline_macro` and `assert_macro` is a virtual file.
pub static VIRTUAL_FILE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[.*?]").unwrap());

/// Statement category filter that is used to filter out statements that should not be included in the coverage report.
/// `included_components` and `ignore_matcher` are references to reduce the amount of data that needs to be copied.
pub struct StatementCategoryFilter<'a> {
    user_project_path: String,
    included_components: &'a [IncludedComponent],
    test_functions: HashSet<FunctionName>,
    ignore_matcher: &'a CairoCoverageIgnoreMatcher,
    libfunc_names_by_idx: HashMap<StatementIdx, String>,
}

/// Build a new instance of the [`StatementCategoryFilter`] based on the given parameters.
pub fn build<'a>(
    user_project_path: &Utf8PathBuf,
    included_components: &'a [IncludedComponent],
    ignore_matcher: &'a CairoCoverageIgnoreMatcher,
    enriched_program: &EnrichedProgram,
) -> StatementCategoryFilter<'a> {
    let test_functions = enriched_program
        .test_executables
        .iter()
        .map(ToString::to_string)
        .map(FunctionName)
        .collect();

    let user_project_path = user_project_path.to_string();
    let libfunc_names_by_idx = libfuncs::build_names_map(&enriched_program.program);

    StatementCategoryFilter {
        user_project_path,
        included_components,
        test_functions,
        ignore_matcher,
        libfunc_names_by_idx,
    }
}

impl StatementCategoryFilter<'_> {
    /// Check if statement with the given index should be included in the coverage report.
    pub fn should_include(
        &self,
        idx: StatementIdx,
        function_name: &FunctionName,
        source_file_full_path: &SourceFileFullPath,
    ) -> bool {
        self.is_allowed_macro(function_name, source_file_full_path)
            && self.is_user_function(source_file_full_path)
            && self.is_reliable_libfunc(idx)
            && self.is_not_ignored(source_file_full_path)
    }

    fn is_allowed_macro(
        &self,
        function_name: &FunctionName,
        source_file_full_path: &SourceFileFullPath,
    ) -> bool {
        if self.test_functions.contains(function_name) {
            self.included_components
                .contains(&IncludedComponent::TestFunctions)
        } else if VIRTUAL_FILE_REGEX.is_match(&source_file_full_path.0) {
            self.included_components
                .contains(&IncludedComponent::Macros)
        } else {
            true
        }
    }

    fn is_user_function(&self, source_file_full_path: &SourceFileFullPath) -> bool {
        source_file_full_path.0.contains(&self.user_project_path)
    }

    fn is_reliable_libfunc(&self, idx: StatementIdx) -> bool {
        !self
            .libfunc_names_by_idx
            .get(&idx)
            .is_some_and(|libfunc_name| NOT_RELIABLE_LIBFUNCS.contains(libfunc_name))
    }

    fn is_not_ignored(&self, source_file_full_path: &SourceFileFullPath) -> bool {
        !self.ignore_matcher.is_ignored(&source_file_full_path.0)
    }
}
