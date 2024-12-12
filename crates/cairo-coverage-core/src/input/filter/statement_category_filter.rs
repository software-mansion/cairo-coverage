use crate::data_loader::LoadedData;
use crate::input::filter::ignore::CairoCoverageIgnoreMatcher;
use crate::input::sierra_to_cairo_map::{SimpleLibfuncName, StatementOrigin};
use cairo_annotations::annotations::coverage::SourceFileFullPath;
use cairo_annotations::annotations::profiler::FunctionName;
use cairo_coverage_args::IncludedComponent;
use camino::Utf8PathBuf;
use regex::Regex;
use std::collections::HashSet;
use std::iter::once;
use std::sync::LazyLock;

pub static VIRTUAL_FILE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[.*?]").unwrap());

/// This is not the best way to do this, and I'm not proud of it.
/// However, it is definitely the easiest way to achieve this.
/// Some functions like `store_temp` are used in many places.
/// Removing it would eliminate a lot of true positives.
/// However, users would likely be more frustrated by false negatives.
static NOT_RELIABLE_LIBFUNCS: LazyLock<HashSet<SimpleLibfuncName>> = LazyLock::new(|| {
    [
        "drop",
        "enable_ap_tracking",
        "disable_ap_tracking",
        "struct_deconstruct",
        "dup",
        "enum_init",
        "struct_construct",
        "store_temp",
        "return",
        "rename",
        "snapshot_take",
        "struct_snapshot_deconstruct",
        "const_as_immediate",
        "contract_address_const",
    ]
    .iter()
    .map(ToString::to_string)
    .map(SimpleLibfuncName)
    .collect()
});

const SNFORGE_TEST_EXECUTABLE: &str = "snforge_internal_test_executable";

#[derive(Eq, PartialEq, Hash)]
enum StatementCategory {
    TestFunction,
    UserFunction,
    NonUserFunction,
    Macro,
    NotReliableLibfunc,
    Ignored,
}

impl From<IncludedComponent> for StatementCategory {
    fn from(included_component: IncludedComponent) -> Self {
        match included_component {
            IncludedComponent::TestFunctions => StatementCategory::TestFunction,
            IncludedComponent::Macros => StatementCategory::Macro,
        }
    }
}

pub struct StatementCategoryFilter {
    user_project_path: String,
    allowed_statement_categories: HashSet<StatementCategory>,
    test_functions: HashSet<String>,
    ignore_matcher: CairoCoverageIgnoreMatcher,
}

impl StatementCategoryFilter {
    pub fn new(
        user_project_path: &Utf8PathBuf,
        included_component: &[IncludedComponent],
        loaded_data: &LoadedData,
    ) -> Self {
        let test_functions = loaded_data
            .debug_info
            .executables
            .get(SNFORGE_TEST_EXECUTABLE)
            .unwrap_or(&Vec::new())
            .iter()
            .map(ToString::to_string)
            .collect();

        let allowed_statement_categories = included_component
            .iter()
            .cloned()
            .map(StatementCategory::from)
            .chain(once(StatementCategory::UserFunction))
            .collect();

        let ignore_matcher = CairoCoverageIgnoreMatcher::new(user_project_path)
            .expect("Failed to create ignore matcher");
        let user_project_path = user_project_path.to_string();

        Self {
            user_project_path,
            allowed_statement_categories,
            test_functions,
            ignore_matcher,
        }
    }

    pub fn should_include(&self, statement_origin: &StatementOrigin) -> bool {
        self.get_categories_for_statement(statement_origin)
            .is_subset(&self.allowed_statement_categories)
    }

    fn get_categories_for_statement(
        &self,
        StatementOrigin {
            function_name: FunctionName(function_name),
            source_file_full_path: SourceFileFullPath(source_file_full_path),
            simple_libfunc_name,
            ..
        }: &StatementOrigin,
    ) -> HashSet<StatementCategory> {
        let mut labels = HashSet::new();

        if self.test_functions.contains(function_name) {
            labels.insert(StatementCategory::TestFunction);
        } else if VIRTUAL_FILE_REGEX.is_match(source_file_full_path) {
            labels.insert(StatementCategory::Macro);
        }

        if source_file_full_path.contains(&self.user_project_path) {
            labels.insert(StatementCategory::UserFunction);
        } else {
            labels.insert(StatementCategory::NonUserFunction);
        }

        if NOT_RELIABLE_LIBFUNCS.contains(&remove_prefix(simple_libfunc_name)) {
            labels.insert(StatementCategory::NotReliableLibfunc);
        }

        if self.ignore_matcher.is_ignored(source_file_full_path) {
            labels.insert(StatementCategory::Ignored);
        }

        labels
    }
}

fn remove_prefix(input: &SimpleLibfuncName) -> SimpleLibfuncName {
    let without_generics = truncate_at_char(&input.0, '<');
    SimpleLibfuncName(truncate_at_char(without_generics, '(').into())
}
fn truncate_at_char(input: &str, delimiter: char) -> &str {
    match input.find(delimiter) {
        Some(index) => &input[..index],
        None => input,
    }
}
