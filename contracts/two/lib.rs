#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod two {
    use one_two_project::traits::one::OneRef;

    #[ink(storage)]
    pub struct Two {
        strength: u16,
    }

    impl Two {
        #[ink(constructor)]
        pub fn new(s: u16) -> Self {
            Two { strength: s }
        }

        #[ink(message)]
        pub fn change(&mut self, l: u16) {
            self.strength = l;
        }
        #[ink(message)]
        pub fn show(&self) -> u16 {
            self.strength
        }
        #[ink(message)]
        pub fn change_one(&mut self, one: AccountId, l: u16) {
            let one_instance: &OneRef = &one;
            one_instance.change(l);
        }
    }
}
