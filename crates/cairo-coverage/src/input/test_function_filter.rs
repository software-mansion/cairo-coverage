use crate::types::FunctionName;
use cairo_lang_sierra::ids::FunctionId;
use std::collections::HashSet;

pub struct TestFunctionFilter {
    include_test_functions: bool,
    test_functions: HashSet<String>,
}

impl TestFunctionFilter {
    pub fn new(
        functions_marked_with_test_attribute: &[FunctionId],
        include_test_functions: bool,
    ) -> Self {
        Self {
            include_test_functions,
            test_functions: functions_marked_with_test_attribute
                .iter()
                .map(ToString::to_string)
                .collect(),
        }
    }

    pub fn should_include(&self, function_name: &FunctionName) -> bool {
        self.include_test_functions || !self.test_functions.contains(function_name)
    }
}
