extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_span;

use std::collections::HashSet;

use rustc_lint::LateContext;
use rustc_middle::ty::{Ty, TyKind};
use rustc_span::def_id::DefId;

/// Constants defining the fully qualified names of Soroban types.
const SOROBAN_ENV: &str = "soroban_sdk::Env";
const SOROBAN_ADDRESS: &str = "soroban_sdk::Address";

/// Determines whether a function defined by its `DefId` is part of a Soroban contract implementation.
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

/// Checks if the provided type is a Soroban environment (`soroban_sdk::Env`).
pub fn is_soroban_env(cx: &LateContext<'_>, expr_type: Ty<'_>) -> bool {
    match expr_type.kind() {
        TyKind::Adt(adt_def, _) => cx.tcx.def_path_str(adt_def.did()).contains(SOROBAN_ENV),
        TyKind::Ref(_, ty, _) => is_soroban_env(cx, *ty),
        _ => false,
    }
}

/// Checks if the provided type is a Soroban Address (`soroban_sdk::Address`).
pub fn is_soroban_address(cx: &LateContext<'_>, expr_type: Ty<'_>) -> bool {
    match expr_type.kind() {
        TyKind::Adt(adt_def, _) => cx.tcx.def_path_str(adt_def.did()).contains(SOROBAN_ADDRESS),
        TyKind::Ref(_, ty, _) => is_soroban_address(cx, *ty),
        _ => false,
    }
}
