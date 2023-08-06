#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub mod impls;
pub mod libs;
pub mod traits;

#[ink::contract]
pub mod football_match {
    /***********
     * Imports *
     ***********/
    use crate::libs::errors::Errors;
    use crate::libs::errors::Errors::*;
    use crate::traits::football_match::FootballMatch;
    use core::prelude::v1::Err;
    /***********
     *   Data  *
     ***********/
    #[ink(storage)]
    pub struct GameData {
        pub winning_team: u8,
        pub admin: AccountId,
        pub particpant_chelsea: AccountId,
        pub particpant_manchester: AccountId,
        pub particpant_chelsea_is_set: bool,
        pub particpant_manchester_is_set: bool,
    }
    /******************
     * Initialisation *
     ******************/
    impl GameData {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                winning_team: 0u8,
                admin: Self::env().caller(),
                particpant_chelsea: AccountId::from([0xFF as u8; 32]),
                particpant_manchester: AccountId::from([0xFF as u8; 32]),
                particpant_chelsea_is_set: false,
                particpant_manchester_is_set: false,
            }
        }
    }
    /**********
     * Events *
     **********/
    #[ink(event)]
    pub struct GameState {
        #[ink(topic)]
        particpant_chelsea_is_set: bool,
        #[ink(topic)]
        particpant_manchester_is_set: bool,
        #[ink(topic)]
        winning_team: u8,
    }
    /***********
     * Methods *
     ***********/
    impl FootballMatch for GameData {
        #[ink(message)]
        fn get_game(&self) -> Result<(), Errors> {
            self.env().emit_event(GameState {
                particpant_chelsea_is_set: self.particpant_chelsea_is_set,
                particpant_manchester_is_set: self.particpant_manchester_is_set,
                winning_team: self.winning_team,
            });
            Ok(())
        }

        #[ink(message)]
        fn set_winner(&mut self, number: u8) -> Result<(), Errors> {
            if Self::env().caller() != self.admin {
                return Err(Errors::OnlyAdmin);
            }
            if number != 1u8 || number != 2u8 {
                return Err(Errors::OnlyOneOrTwo);
            }
            self.winning_team = number;
            if number == 1u8 {
                let balance = self.env().balance();
                let call = self.env().transfer(self.particpant_manchester, balance);
                if call.is_err() {
                    return Err(Errors::DontWork);
                }
            }
            if number == 2u8 {
                let balance = self.env().balance();
                let call = self.env().transfer(self.particpant_chelsea, balance);
                if call.is_err() {
                    return Err(Errors::DontWork);
                }
            }
            Ok(())
        }
        #[ink(message, payable)]
        fn set_particpant_chelsea(&mut self) -> Result<(), Errors> {
            if self.particpant_chelsea_is_set == true {
                return Err(ParticipantChelseaIsAlreadySet);
            }
            self.particpant_chelsea = Self::env().caller();
            self.particpant_chelsea_is_set = true;
            self.env().transferred_value();
            Ok(())
        }
        #[ink(message, payable)]
        fn set_particpant_manchester(&mut self) -> Result<(), Errors> {
            if self.particpant_manchester_is_set == true {
                return Err(ParticipantManchesterIsAlreadySet);
            }
            self.particpant_manchester = Self::env().caller();
            self.particpant_manchester_is_set = true;
            self.env().transferred_value();
            Ok(())
        }

        #[ink(message)]
        fn change_admin(&mut self, new_admin: AccountId) -> Result<(), Errors> {
            if Self::env().caller() != self.admin {
                return Err(OnlyAdmin);
            }
            self.admin = new_admin;
            Ok(())
        }

        #[ink(message)]
        fn restart_match(&mut self) -> Result<(), Errors> {
            if Self::env().caller() != self.admin {
                return Err(OnlyAdmin);
            }
            self.winning_team = 0u8;
            self.admin = self.admin;
            self.particpant_chelsea = AccountId::from([0xFF as u8; 32]);
            self.particpant_manchester = AccountId::from([0xFF as u8; 32]);
            self.particpant_chelsea_is_set = false;
            self.particpant_manchester_is_set = false;
            Ok(())
        }
    }
}
