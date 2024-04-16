# Iterators-over-indexing

### What it does

It warns if the for loop uses indexing instead of iterator. If the indexing goes to length it will not raise a warning.

### Why is this bad?


Accessing a vector by index is slower than using an iterator. Also, if the index is out of bounds, it will panic.



### Example​

```raw

pub fn sum(e: Env) -> Result<i32, Error>{




       let mut ret = 0_i32;


       let vec = e.storage().instance().get::<DataKey, Vec<i32>>(&DataKey::Data).ok_or(Error::NoData)?;


       for i in 0..4{


           ret = ret.checked_add(vec.get(i).ok_or(Error::NoData)?).ok_or(Error::IntegerOverflow)?;


       }


       Ok(ret)


   }

```
Use instead:
```rust
pub fn sum(e: Env) -> Result<i32, Error>{




       let mut ret = 0_i32;


       let vec = e.storage().instance().get::<DataKey, Vec<i32>>(&DataKey::Data).ok_or(Error::NoData)?;


       for i in vec{


           ret = ret.checked_add(i).ok_or(Error::IntegerOverflow)?;


       }


       Ok(ret)


   }
```
### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/iterators-over-indexing).

