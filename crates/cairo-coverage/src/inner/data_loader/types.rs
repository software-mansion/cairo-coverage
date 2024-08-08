use crate::inner::data_loader::deserialize;
use anyhow::{ensure, Result};
use cairo_lang_sierra::debug_info::Annotations;
use cairo_lang_sierra::program::StatementIdx;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Deref;

const PROFILER_NAMESPACE: &str = "github.com/software-mansion/cairo-profiler";
const COVERAGE_NAMESPACE: &str = "github.com/software-mansion/cairo-coverage";

type FileLocation = String;
type FunctionName = String;

type CodeLocations = (FileLocation, Range);

#[repr(transparent)]
pub struct StatementMap(HashMap<StatementIdx, StatementDetails>);

impl Deref for StatementMap {
    type Target = HashMap<StatementIdx, StatementDetails>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Position {
    pub col: usize,
    pub line: usize,
}

#[derive(Deserialize)]
struct CoverageAnnotations {
    statements_code_locations: HashMap<StatementIdx, Vec<CodeLocations>>,
}

#[derive(Deserialize)]
struct ProfilerAnnotations {
    statements_functions: HashMap<StatementIdx, Vec<FunctionName>>,
}

#[allow(dead_code)] // Temporary
#[derive(Debug)]
pub struct StatementDetails {
    pub code_locations: Vec<CodeLocations>,
    pub function_names: Vec<FunctionName>,
}

impl TryFrom<&Annotations> for StatementMap {
    type Error = anyhow::Error;

    fn try_from(annotations: &Annotations) -> Result<Self> {
        let CoverageAnnotations {
            statements_code_locations,
        } = deserialize::by_namespace(annotations, COVERAGE_NAMESPACE)?;

        let ProfilerAnnotations {
            statements_functions,
        } = deserialize::by_namespace(annotations, PROFILER_NAMESPACE)?;

        ensure!(
            have_same_keys(&statements_code_locations, &statements_functions),
            "{COVERAGE_NAMESPACE} and {PROFILER_NAMESPACE} doesn't have the same statement idx"
        );

        let statement_map = statements_code_locations
            .into_iter()
            .map(|(key, code_locations)| {
                // Safe to unwrap because we ensured that the keys are the same
                let function_names = statements_functions.get(&key).unwrap().to_owned();
                (
                    key,
                    StatementDetails {
                        code_locations,
                        function_names,
                    },
                )
            })
            .collect();

        Ok(Self(statement_map))
    }
}

fn have_same_keys<K, V1, V2>(map1: &HashMap<K, V1>, map2: &HashMap<K, V2>) -> bool
where
    K: Eq + Hash,
{
    map1.keys().collect::<HashSet<&K>>() == map2.keys().collect::<HashSet<&K>>()
}
