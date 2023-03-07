#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod one {
    use one_two_project::traits::two::TwoRef;

    #[ink(storage)]
    pub struct One {
        strength: u16,
    }

    impl One {
        #[ink(constructor)]
        pub fn new(s: u16) -> Self {
            One { strength: s }
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
        pub fn change_two(&mut self, two: AccountId, l: u16) {
            let one_instance: &TwoRef = &two;
            one_instance.change(l);
        }
    }
}
