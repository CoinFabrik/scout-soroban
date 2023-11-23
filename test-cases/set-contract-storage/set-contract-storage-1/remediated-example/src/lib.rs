use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Map, Symbol};

#[contract]
pub struct SetContractStorage;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Storage {
    balances: Map<Address, i128>,
}
const STORAGE: Symbol = symbol_short!("STORAGE");

#[contractimpl]
impl UnsafeExpect {
    pub fn set_contract_storage(env: Env, address: Address, balance: i128) -> State {
        // Only the owner can set the balance.
        address.require_auth();

        // Get the current state.
        let mut state = Self::get_state(env.clone());

        // Set the new account to have total supply if it doesn't exist.
        if !state.balances.contains_key(address.clone()) {
            state.balances.set(address, balance);
            // Save the state.
            env.storage().persistent().set(&STATE, &state);
        }

        state
    }

    // Returns the balance of a given account.
    pub fn balance_of(env: Env, owner: Address) -> i128 {
        let state = Self::get_state(env);
        state.balances.get(owner).unwrap_or(0)
    }

    /// Return the current state.
    pub fn get_contract_storage(env: Env) -> State {
        env.storage().persistent().get(&STATE).unwrap_or(State {
            balances: Map::new(&env),
        })
    }
}
