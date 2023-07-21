#[ink::trait_definition]
pub trait FootballMatch {
    #[ink(message)]
    fn get_game(&self) -> Result<u8, ()>;

    #[ink(message)]
    fn set_winner(&mut self, number: u8) -> Result<(), ()>;

    #[ink(message)]
    fn set_particpant_chelsea(&mut self) -> Result<(), ()>;

    #[ink(message)]
    fn set_particpant_manchester(&mut self) -> Result<(), ()>;
}
