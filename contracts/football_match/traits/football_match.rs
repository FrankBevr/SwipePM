use crate::libs::errors::Error;

#[ink::trait_definition]
pub trait FootballMatch {
    #[ink(message)]
    fn get_game(&self) -> Result<(), Error>;

    #[ink(message)]
    fn set_winner(&mut self, number: u8) -> Result<(), Error>;

    #[ink(message)]
    fn set_particpant_chelsea(&mut self) -> Result<(), Error>;

    #[ink(message)]
    fn set_particpant_manchester(&mut self) -> Result<(), Error>;
}
