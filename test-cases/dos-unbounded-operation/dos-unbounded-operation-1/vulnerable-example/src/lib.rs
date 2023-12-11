#![no_std]

use soroban_sdk::{contract, contractimpl};

// /// A payment to be made to an account.
// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct Payee {
//     /// The account to which the payment is to be made.
//     pub address: Address,
//     /// The amount to be paid.
//     pub value: i128,
// }

// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct State {
//     /// The payees of the operation.
//     pub payees: Map<u128, Payee>,
//     /// The next payee index.
//     pub next_payee_ix: u128,
// }

// const STATE: Symbol = symbol_short!("STATE");

#[contract]
pub struct DosUnboundedOperation;

const VALUE: i128 = 100;

#[contractimpl]
impl DosUnboundedOperation {
    pub fn asd(this_is_a_test: i128) {
        let this_is_a_local_variable = 100;
        let this_is_another_local_variable = this_is_a_local_variable + 2;
        for c in 0..this_is_a_local_variable {
            let b = c + 1;
        }
    }

    // /// Adds a payee to the operation.
    // pub fn add_payee(env: Env, address: Address, value: i128) -> u128 {
    //     // Get the state from storage
    //     let mut state = env.storage().persistent().get(&STATE).unwrap_or(State {
    //         payees: Map::new(&env),
    //         next_payee_ix: 0,
    //     });

    //     // Add the payee to the state and increment the next payee index
    //     let new_payee = Payee { address, value };
    //     state.payees.set(state.next_payee_ix, new_payee);
    //     state.next_payee_ix = state.next_payee_ix.checked_add(1).unwrap();

    //     // Save the state to storage
    //     env.storage().persistent().set(&STATE, &state);

    //     // Return the index of the new payee
    //     state.next_payee_ix.checked_sub(1).unwrap()
    // }

    // /// Returns the payee at the given index.
    // pub fn get_payee(env: Env, ix: u128) -> Option<Payee> {
    //     // Get the state from storage
    //     let state = env.storage().persistent().get(&STATE).unwrap_or(State {
    //         payees: Map::new(&env),
    //         next_payee_ix: 0,
    //     });

    //     // Return the payee at the given index
    //     state.payees.get(ix)
    // }

    // Pays out all payees.
    // pub fn payout(env: Env) {
    //     // Get the state from storage
    //     let mut state = env.storage().persistent().get(&STATE).unwrap_or(State {
    //         payees: Map::new(&env),
    //         next_payee_ix: 0,
    //     });

    //     // Iterate over all payees and pay them out
    //     for ix in 0..state.next_payee_ix {
    //         let payee = state.payees.get(ix).unwrap();

    //         // let client: TokenClient<'_> = TokenClient::new(&env, &payee.address);
    //         // client.transfer(
    //         //     &env.current_contract_address(),
    //         //     &payee.address,
    //         //     &payee.value,
    //         // );
    //         let topics = (
    //             symbol_short!("transfer"),
    //             env.current_contract_address(),
    //             payee.value,
    //         );
    //         env.events().publish(topics, payee.value);
    //     }

    //     // Clear the state
    //     state = State {
    //         payees: Map::new(&env),
    //         next_payee_ix: 0,
    //     };

    //     // Save the state to storage
    //     env.storage().persistent().set(&STATE, &state);
    // }
}

// #[cfg(test)]
// mod tests {

//     use soroban_sdk::{log, testutils, token::StellarAssetInterface, Address, Env};

//     use super::*;

//     #[test]
//     fn test_add_payee() {
//         // Given
//         let env = Env::default();
//         let contract_id = env.register_contract(None, DosUnboundedOperation);
//         let client = DosUnboundedOperationClient::new(&env, &contract_id);

//         // When
//         let payee_1 = <Address as testutils::Address>::generate(&env);
//         let payee_value_1 = 100;
//         let ix_1 = client.add_payee(&payee_1, &payee_value_1);

//         let payee_2 = <Address as testutils::Address>::generate(&env);
//         let payee_value_2 = 200;
//         let ix_2 = client.add_payee(&payee_2, &payee_value_2);

//         // Then
//         assert_eq!(ix_1, 0);
//         assert_eq!(
//             client.get_payee(&ix_1),
//             Some(Payee {
//                 address: payee_1,
//                 value: payee_value_1
//             })
//         );

//         assert_eq!(ix_2, 1);
//         assert_eq!(
//             client.get_payee(&ix_2),
//             Some(Payee {
//                 address: payee_2,
//                 value: payee_value_2
//             })
//         );
//     }

//     #[test]
//     fn test() {
//         let env = Env::default();
//         let payee_1 = <Address as testutils::Address>::generate(&env);
//         let asd = TokenClient::new(&env, &payee_1);
//         asd.approve(&asd.address, &payee_1, &100, &0);
//         // let a = asd.balance(&payee_1);

//         // Print out "a"

//         // log!(&env, "a log entry", a);
//     }

//     #[test]
//     fn test_payout() {
//         // Given
//         let env = Env::default();
//         let contract_id = env.register_contract(None, DosUnboundedOperation);
//         let client = DosUnboundedOperationClient::new(&env, &contract_id);

//         // When
//         let payee_1 = <Address as testutils::Address>::generate(&env);
//         let payee_value_1 = 100;
//         let ix_1 = client.add_payee(&payee_1, &payee_value_1);

//         let payee_2 = <Address as testutils::Address>::generate(&env);
//         let payee_value_2 = 200;
//         let ix_2 = client.add_payee(&payee_2, &payee_value_2);

//         client.payout();

//         // Then
//         // assert_eq!(ix_1, 0);
//         // assert_eq!(
//         //     client.get_payee(&ix_1),
//         //     Some(Payee {
//         //         address: payee_1,
//         //         value: payee_value_1
//         //     })
//         // );

//         // assert_eq!(ix_2, 1);
//         // assert_eq!(
//         //     client.get_payee(&ix_2),
//         //     Some(Payee {
//         //         address: payee_2,
//         //         value: payee_value_2
//         //     })
//         // );
//     }
// }
