# DoS unexpected revert with vector

### What it does

Checks for array pushes without access control.

### Why is this bad?

Arrays have a maximum size according to the storage cell. If the array is full, the push will revert. This can be used to prevent the execution of a function.

### Known problems

If the owner validation is performed in an auxiliary function, the warning will be shown, resulting in a false positive.

### Example

```rust
pub fn add_candidate(env: Env, candidate: Address, caller: Address) -> Result<(), URError> {
    let mut state = Self::get_state(env.clone());
    if Self::vote_ended(env.clone()) {
        return Err(URError::VoteEnded);
    }
    if state.already_voted.contains_key(caller.clone()) {
        return Err(URError::AccountAlreadyVoted); 
    } else {
        state.candidates.push_back(candidate.clone());
        state.votes.set(candidate, 0);
        Ok(())
    }
}
```

Use instead:

```rust
pub fn add_candidate(env: Env, candidate: Address, caller: Address) -> Result<(), URError> {
    let mut state = Self::get_state(env.clone());
     // Require authorization from an admin set at contract initalization.
      state.admin.require_auth(); 
    if Self::vote_ended(env.clone()) {
        return Err(URError::VoteEnded);
    }
    if state.already_voted.contains_key(caller.clone()) {
        return Err(URError::AccountAlreadyVoted); 
    } else {
        state.candidates.push_back(candidate.clone());
        state.votes.set(candidate, 0);
        Ok(())
    }
}
```
Or

```rust
 pub fn add_candidate(env: Env, candidate: Address, caller: Address) -> Result<(), URError> {
      caller.require_auth();
      let mut state = Self::get_state(env.clone());
      if Self::vote_ended(env.clone()) {
          return Err(URError::VoteEnded);
      }
      if Self::account_has_voted(env.clone(), caller.clone()) {
          return Err(URError::AccountAlreadyVoted); 
      } else {
          // Replace the Vector with a mapping like structure made with a DataKey enum.
          env.storage().instance().set(&DataKey::Candidate(candidate.clone()), &Candidate{votes: 0});
          state.total_candidates += 1; 
          env.storage().instance().set(&DataKey::State, &state);
          Ok(())
      }
}
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/dos-unexpected-revert-with-vector).
