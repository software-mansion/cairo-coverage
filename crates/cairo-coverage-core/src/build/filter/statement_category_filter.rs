use crate::args::IncludedComponent;
use crate::build::filter::ignore_matcher::CairoCoverageIgnoreMatcher;
use cairo_annotations::annotations::coverage::SourceFileFullPath;
use cairo_annotations::annotations::profiler::FunctionName;
use cairo_lang_sierra::ids::FunctionId;
use cairo_lang_sierra::program::StatementIdx;
use cairo_lang_sierra_to_casm::compiler::CairoProgramDebugInfo;
use camino::Utf8Path;
use std::collections::HashSet;

/// Statement category filter that is used to filter out statements that should not be included in the coverage report.
pub struct StatementCategoryFilter<'a> {
    user_project_path: &'a str,
    included_components: &'a [IncludedComponent],
    test_functions: HashSet<FunctionName>,
    ignore_matcher: &'a CairoCoverageIgnoreMatcher,
    casm_debug_info: &'a CairoProgramDebugInfo,
}

/// Build a new instance of the [`StatementCategoryFilter`] based on the given parameters.
pub fn build<'a>(
    user_project_path: &'a Utf8Path,
    included_components: &'a [IncludedComponent],
    ignore_matcher: &'a CairoCoverageIgnoreMatcher,
    test_executables: &[FunctionId],
    casm_debug_info: &'a CairoProgramDebugInfo,
) -> StatementCategoryFilter<'a> {
    let test_functions = test_executables
        .iter()
        .map(ToString::to_string)
        .map(FunctionName)
        .collect();

    StatementCategoryFilter {
        user_project_path: user_project_path.as_str(),
        included_components,
        test_functions,
        ignore_matcher,
        casm_debug_info,
    }
}

impl StatementCategoryFilter<'_> {
    /// Check if statement with the given index should be included in the coverage report.
    pub fn should_include(
        &self,
        idx: StatementIdx,
        function_name: &FunctionName,
        source_file_full_path: &SourceFileFullPath,
        is_macro: bool,
    ) -> bool {
        self.is_allowed_macro(function_name, is_macro)
            && self.is_user_function(source_file_full_path)
            && self.does_compile_to_any_casm(idx)
            && self.is_not_ignored(source_file_full_path)
    }

    fn is_allowed_macro(&self, function_name: &FunctionName, is_macro: bool) -> bool {
        if self.test_functions.contains(function_name) {
            self.included_components
                .contains(&IncludedComponent::TestFunctions)
        } else if is_macro {
            self.included_components
                .contains(&IncludedComponent::Macros)
        } else {
            true
        }
    }

    fn is_user_function(&self, source_file_full_path: &SourceFileFullPath) -> bool {
        source_file_full_path.0.contains(self.user_project_path)
    }

    fn does_compile_to_any_casm(&self, idx: StatementIdx) -> bool {
        let debug_info = &self.casm_debug_info.sierra_statement_info[idx.0];
        debug_info.start_offset != debug_info.end_offset
    }

    fn is_not_ignored(&self, source_file_full_path: &SourceFileFullPath) -> bool {
        !self.ignore_matcher.is_ignored(&source_file_full_path.0)
    }
}
