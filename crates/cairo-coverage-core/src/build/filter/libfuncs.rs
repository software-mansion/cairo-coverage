use cairo_lang_sierra::ids::ConcreteLibfuncId;
use cairo_lang_sierra::program::{ConcreteLibfuncLongId, Program, Statement, StatementIdx};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

/// This is not the best way to do this, and I'm not proud of it.
/// However, it is definitely the easiest way to achieve this.
/// Some functions like `store_temp` are used in many places.
/// Removing it would eliminate a lot of true positives.
/// However, users would likely be more frustrated by false negatives.
pub static NOT_RELIABLE_LIBFUNCS: LazyLock<HashSet<String>> = LazyLock::new(|| {
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
    .collect()
});

/// Build a map from statement index to the name of the statement.
pub fn build_names_map(program: &Program) -> HashMap<StatementIdx, String> {
    let libfuncs_long_ids_by_ids = program
        .libfunc_declarations
        .iter()
        .map(|libfunc_declaration| (&libfunc_declaration.id, &libfunc_declaration.long_id))
        .collect::<HashMap<_, _>>();

    program
        .statements
        .iter()
        .enumerate()
        .filter_map(|(idx, statement)| {
            Some((
                StatementIdx(idx),
                statement_to_string(statement, &libfuncs_long_ids_by_ids)?,
            ))
        })
        .collect()
}

/// Convert a statement to a string representation excluding the generic prefix.
fn statement_to_string(
    statement: &Statement,
    libfuncs_long_ids_by_ids: &HashMap<&ConcreteLibfuncId, &ConcreteLibfuncLongId>,
) -> Option<String> {
    Some(match statement {
        Statement::Invocation(invocation_statement) => remove_prefix(
            libfuncs_long_ids_by_ids
                .get(&invocation_statement.libfunc_id)?
                .to_string(),
        ),
        Statement::Return(_) => "return".to_string(),
    })
}

/// Remove the generic prefix from the string.
fn remove_prefix(input: String) -> String {
    truncate_at_chars(input, &['<', '('])
}

/// Truncate the string at the first occurrence of any of the delimiters.
fn truncate_at_chars(input: String, delimiters: &[char]) -> String {
    match input.find(|c| delimiters.contains(&c)) {
        Some(index) => input[..index].into(),
        None => input,
    }
}
