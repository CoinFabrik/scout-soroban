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
impl SetContractStorage {
    pub fn set_contract_storage(env: Env, address: Address, balance: i128) -> Storage {
        // Get the current state.
        let mut state = Self::get_contract_storage(env.clone());

        // Set the new account to have total supply if it doesn't exist.
        if !state.balances.contains_key(address.clone()) {
            state.balances.set(address, balance);
            // Save the state.
            env.storage().persistent().set(&STORAGE, &state);
        }

        state
    }

    // Transfer tokens from one account to another.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Storage {

        env.
        // Get the current state.
        let mut state = Self::get_contract_storage(env.clone());

        // Get the balance of the sender.
        let from_balance = state.balances.get(from.clone()).unwrap_or(0);

        // Get the balance of the receiver.
        let to_balance = state.balances.get(to.clone()).unwrap_or(0);

        // Make sure the sender has enough tokens.
        assert!(from_balance >= amount);

        // Update the sender's balance.
        state.balances.set(from, from_balance - amount);

        // Update the receiver's balance.
        state.balances.set(to, to_balance + amount);

        // Save the state.
        env.storage().persistent().set(&STORAGE, &state);

        state
    }

    /// Return the current state.
    pub fn get_contract_storage(env: Env) -> Storage {
        env.storage().persistent().get(&STORAGE).unwrap_or(Storage {
            balances: Map::new(&env),
        })
    }
}
