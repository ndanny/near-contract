//! This smart contract implements a very simple counter that
//! is backed by a storage on the near blockchain.
//!
//! Several methods are implemented:
//!     increment()
//!     decrement()
//!     get_count()
//!     reset()
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

/// Attributes are used to prepare code for de/serialization and
/// invocation on the blockchain.
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    data: i8,
}

#[near_bindgen]
impl Counter {
    /// Returns the 8-bit signed integer value from the counter
    ///
    /// Using the near-cli, we can call this by running
    /// `near view counter.pw.testnet get_count`
    pub fn get_count(&self) -> i8 {
        self.data
    }

    /// Increments the counter by 1 unit
    ///
    /// Using the near-cli, can can call this by running
    /// `near call counter.pw.testnet increment --accountId donation.pw.testnet`
    pub fn increment(&mut self) {
        self.data = self.data.wrapping_add(1);
        let info = format!("Increased counter to {}", self.data);
        env::log_str(&info);
    }

    /// Decrements the counter by 1 unit
    ///
    /// Using the near-cli, can can call this by running
    /// `near call counter.pw.testnet decrement --accountId donation.pw.testnet`
    pub fn decrement(&mut self) {
        self.data = self.data.wrapping_sub(1);
        let info = format!("Decreased counter to {}", self.data);
        env::log_str(&info);
    }

    /// Resets the counter to 0. Nothing fancy.
    pub fn reset(&mut self) {
        self.data = 0;
        let info = format!("Counter has been reset to {}", self.data);
        env::log_str(&info);
    }
}

/// Good code isn't complete without unit tests.
/// Todo: Investigate how to use near_sdk::MockedBlockchain and VMContext
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment() {
        let mut contract = Counter { data: 0 };
        contract.increment();
        assert_eq!(1, contract.get_count());
    }

    #[test]
    fn decrement() {
        let mut contract = Counter { data: 0 };
        contract.decrement();
        assert_eq!(-1, contract.get_count());
    }

    #[test]
    fn reset() {
        let mut contract = Counter { data: 0 };
        contract.increment();
        contract.reset();
        assert_eq!(0, contract.get_count());
    }

    #[test]
    fn overflow_wraps() {
        let mut contract = Counter { data: 127 };
        contract.increment();
        assert_eq!(-128, contract.get_count());
    }

    #[test]
    fn underflow_wraps() {
        let mut contract = Counter { data: -128 };
        contract.decrement();
        assert_eq!(127, contract.get_count());
    }
}
