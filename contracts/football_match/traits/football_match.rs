use crate::libs::errors::Errors;

#[ink::trait_definition]
pub trait FootballMatch {
    #[ink(message)]
    fn get_game(&self) -> Result<(), Errors>;

    #[ink(message)]
    fn set_winner(&mut self, winner: u8) -> Result<(), Errors>;

    #[ink(message)]
    fn set_particpant_chelsea(&mut self) -> Result<(), Errors>;

    #[ink(message)]
    fn set_particpant_manchester(&mut self) -> Result<(), Errors>;
}
