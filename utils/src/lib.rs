#![feature(rustc_private)]

mod function_call_visitor;
pub use function_call_visitor::FunctionCallVisitor;

mod soroban_utils;
pub use soroban_utils::is_soroban_function;
