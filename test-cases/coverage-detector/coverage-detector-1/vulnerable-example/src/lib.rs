#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct Coverage;

#[contractimpl]
impl Coverage {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}
