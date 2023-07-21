#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub mod traits;

#[ink::contract]
mod football_match {
    pub use crate::traits::football_match::FootballMatch;

    #[ink(storage)]
    pub struct GameData {
        winner: u8,
        particpant_chelsea: AccountId,
        particpant_manchester: AccountId,
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
        fn get_game(&self) -> Result<u8, ()> {
            ink::env::debug_println!("Thast my winer {:?}", self.winner);
            ink::env::debug_println!("Thast my chelsea {:?}", self.particpant_chelsea);
            ink::env::debug_println!("Thast my manchester {:?}", self.particpant_manchester);
            Ok(self.winner)
        }
        #[ink(message)]
        fn set_winner(&mut self, number: u8) -> Result<(), ()> {
            self.winner = number;
            ink::env::debug_println!("Now the winner is set. The Winner is {:?}", self.winner);
            Ok(())
        }
        #[ink(message)]
        fn set_particpant_chelsea(&mut self) -> Result<(), ()> {
            self.particpant_chelsea = Self::env().caller();
            ink::env::debug_println!(
                "Now the Chelsea is set. The Chelsea is {:?}",
                self.particpant_chelsea
            );
            Ok(())
        }
        #[ink(message)]
        fn set_particpant_manchester(&mut self) -> Result<(), ()> {
            self.particpant_manchester = Self::env().caller();
            ink::env::debug_println!(
                "Now the Manchester is set. The Manchester is {:?}",
                self.particpant_manchester
            );
            Ok(())
        }
    }
}
