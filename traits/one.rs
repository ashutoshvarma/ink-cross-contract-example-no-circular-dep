use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type OneRef = dyn One;

#[openbrush::trait_definition]
pub trait One {
    #[ink(message)]
    fn change(&mut self, l: u16);

    #[ink(message)]
    fn show(&self) -> u16;

    #[ink(message)]
    fn change_two(&mut self, two: AccountId, l: u16);
}
