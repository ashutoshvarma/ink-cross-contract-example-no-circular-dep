use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type TwoRef = dyn Two;

#[openbrush::trait_definition]
pub trait Two {
    #[ink(message)]
    fn change(&mut self, l: u16);

    #[ink(message)]
    fn show(&self) -> u16;

    #[ink(message)]
    fn change_one(&mut self, one: AccountId, l: u16);
}
