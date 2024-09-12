use crate::cli::IncludedComponent;
use crate::data_loader::LoadedData;
use crate::input::sierra_to_cairo_map::StatementOrigin;
use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use regex::Regex;
use std::collections::HashSet;
use std::iter::once;
use std::sync::LazyLock;

pub static VIRTUAL_FILE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[.*?]").unwrap());
const SNFORGE_TEST_EXECUTABLE: &str = "snforge_internal_test_executable";
const SNFORGE_SIERRA_DIR: &str = ".snfoundry_versioned_programs";

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
        source_sierra_path: &str,
        included_component: &[IncludedComponent],
        loaded_data: &LoadedData,
    ) -> Result<Self> {
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

        let user_project_path = find_user_project_path(source_sierra_path)?;
        Ok(Self {
            user_project_path,
            allowed_statement_categories,
            test_functions,
        })
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

fn find_user_project_path(source_sierra_path: &str) -> Result<String> {
    Utf8PathBuf::from(source_sierra_path)
        .parent()
        .filter(|parent| parent.file_name() == Some(SNFORGE_SIERRA_DIR))
        .and_then(|parent| parent.parent())
        .map(ToString::to_string)
        .context(format!(
            "Source sierra path should be in the format: <project_root>/{SNFORGE_SIERRA_DIR}/<file>.sierra.json, got: {source_sierra_path}"
        ))
}
