#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod two {
    use one_two_project::traits::{one::*, two::*};

    #[ink(storage)]
    pub struct TwoContract {
        strength: u16,
    }

    impl TwoContract {
        #[ink(constructor)]
        pub fn new(s: u16) -> Self {
            TwoContract { strength: s }
        }
    }

    impl Two for TwoContract {
        #[ink(message)]
        fn change(&mut self, l: u16) {
            self.strength = l;
        }
        #[ink(message)]
        fn show(&self) -> u16 {
            self.strength
        }
        #[ink(message)]
        fn change_one(&mut self, one: AccountId, l: u16) {
            let one_instance: &OneRef = &one;
            one_instance.change(l);
        }
    }
}
