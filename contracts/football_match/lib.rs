#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub mod impls;
pub mod libs;
pub mod traits;

#[ink::contract]
pub mod football_match {
    pub use crate::libs::errors::Error;
    pub use crate::traits::football_match::FootballMatch;

    #[ink(storage)]
    pub struct GameData {
        pub winner: u8,
        pub particpant_chelsea: AccountId,
        pub particpant_manchester: AccountId,
    }

    impl GameData {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                winner: 0u8,
                particpant_chelsea: Self::env().caller(),
                particpant_manchester: Self::env().caller(),
            }
        }
    }

    impl FootballMatch for GameData {
        #[ink(message)]
        fn get_game(&self) -> Result<(), Error> {
            Ok(())
        }
        #[ink(message)]
        fn set_winner(&mut self, number: u8) -> Result<(), Error> {
            self.winner = number;
            Ok(())
        }
        #[ink(message)]
        fn set_particpant_chelsea(&mut self) -> Result<(), Error> {
            self.particpant_chelsea = Self::env().caller();
            Ok(())
        }
        #[ink(message)]
        fn set_particpant_manchester(&mut self) -> Result<(), Error> {
            self.particpant_manchester = Self::env().caller();
            Ok(())
        }
    }
}
