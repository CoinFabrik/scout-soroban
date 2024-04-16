extern crate rustc_lint;
extern crate rustc_span;

use std::collections::HashSet;

use rustc_lint::LateContext;
use rustc_span::def_id::DefId;

/// Determines whether a function defined by its `DefId` is part of a Soroban contract implementation.
///
/// This function checks if the provided `DefId` corresponds to a function that is part of an
/// implementation marked with the `#[contractimpl]` attribute specific to Soroban contracts.
/// It does this by comparing the function's path to a set of known patterns that are associated
/// with Soroban contract implementations.
///
/// # Parameters
/// - `cx`: The context from the compiler's late lint pass.
/// - `checked_functions`: A set containing strings that represent function names.
/// - `function_def_id`: The `DefId` of the current function being analyzed.
///
/// # Returns
/// - `true` if the function belongs to a Soroban contract implementation.
/// - `false` if the function does not belong to a Soroban contract implementation.
pub fn is_soroban_function(
    cx: &LateContext<'_>,
    checked_functions: &HashSet<String>,
    function_def_id: &DefId,
) -> bool {
    let def_path_str = cx.tcx.def_path_str(*function_def_id);
    let mut parts = def_path_str.rsplitn(2, "::");

    // Safely extract function_name and contract_path, checking for empty strings
    let function_name = match parts.next() {
        Some(name) if !name.is_empty() => name,
        _ => return false, // Return false if function_name is None or empty
    };
    let contract_path = match parts.next() {
        Some(path) if !path.is_empty() => path,
        _ => return false, // Return false if contract_path is None or empty
    };

    // Define the patterns to check against
    let patterns = [
        format!("{}Client::<'a>::try_{}", contract_path, function_name),
        format!("{}::{}", contract_path, function_name),
        format!("{}::spec_xdr_{}", contract_path, function_name),
        format!("{}Client::<'a>::{}", contract_path, function_name),
    ];

    // Check if all defined patterns are contained within the checked_functions
    patterns
        .iter()
        .all(|pattern| checked_functions.contains(pattern))
}
