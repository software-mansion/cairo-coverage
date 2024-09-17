use crate::cli::IncludedComponent;
use crate::data_loader::LoadedData;
use crate::input::sierra_to_cairo_map::StatementOrigin;
use camino::Utf8PathBuf;
use regex::Regex;
use std::collections::HashSet;
use std::iter::once;
use std::sync::LazyLock;

pub static VIRTUAL_FILE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[.*?]").unwrap());
const SNFORGE_TEST_EXECUTABLE: &str = "snforge_internal_test_executable";

#[derive(Eq, PartialEq, Hash)]
enum StatementCategory {
    TestFunction,
    UserFunction,
    NonUserFunction,
    Macro,
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

        let user_project_path = user_project_path.to_string();

        Self {
            user_project_path,
            allowed_statement_categories,
            test_functions,
        }
    }

    pub fn should_include(&self, statement_origin: &StatementOrigin) -> bool {
        self.get_categories_for_statement(statement_origin)
            .is_subset(&self.allowed_statement_categories)
    }

    fn get_categories_for_statement(
        &self,
        StatementOrigin {
            function_name,
            file_location,
            ..
        }: &StatementOrigin,
    ) -> HashSet<StatementCategory> {
        let mut labels = HashSet::new();

        if self.test_functions.contains(function_name) {
            labels.insert(StatementCategory::TestFunction);
        } else if VIRTUAL_FILE_REGEX.is_match(file_location) {
            labels.insert(StatementCategory::Macro);
        }

        if file_location.contains(&self.user_project_path) {
            labels.insert(StatementCategory::UserFunction);
        } else {
            labels.insert(StatementCategory::NonUserFunction);
        }

        labels
    }
}
