//! # football_match
//!
//! `football_match` is a small smart contract.\
//! football_match contains the logic for [SwipePM Dapp](https://github.com/FrankBevr/SwipePM).\
//! SwipePM is a betting dapp.\
//! It allows to bet on Manchester or Chelsea.\
//!
//! ## Overview (Userflow & Logic)
//! 1. **Alice** instantiates the contract.
//! 2. **Alice** becomes the admin.
//! 3. **Bob** uses the frontend and bets on Manchester.
//! 4. **Charlie** uses the frontend and isn't allowed to bet on Manchester.
//! 5. **Charlie** bets on Chelsea.
//! 6. The amount of the bet is controlled by the frontend.
//! 7. **Alice**, the admin, calls `set_winner` with the winner value.\
//!     If Manchester won the value has to be 1\
//!     If Chelsea won the value has to be 2\
//!     The Default value is 0\
//! 8. **Alice** sends value of 1 via `set_winner`.
//! 9. Manchester won.
//! 10. **Bob**, the manchester better, recieves all funds.
//! 11. **Alice** calls `restart_match`, if a new manchester vs chelsea game is happening.\
//!      `restart_match` resets all values, expect the current admin.
//! 12. **Alice** will be busy in the upcomming match.
//! 13. **Alice** calls `change_admin` and sets **Django** as the new admin.
//! 14. **Djange** is excited.
#[warn(missing_docs)]
#[cfg_attr(not(feature = "std"), no_std, no_main)]

/// The Utilities libary for football_match
pub mod libs;
/// The Interface for football_match
pub mod traits;
/// The football_match contract
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
            if number != 1u8 && number != 2u8 {
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

#[cfg(test)]
mod tests {

    use crate::{libs::errors::Errors, traits::football_match::FootballMatch};
    use ink::env::Error;

    use super::*;
    use ink::{env::test::default_accounts, primitives::AccountId};

    #[test]
    fn winning_team_is_0() {
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let contract = football_match::GameData::new();

        assert!(contract.winning_team == 0u8);
    }

    #[test]
    fn particpant_is_empty() {
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let contract = football_match::GameData::new();
        let empty = AccountId::from([0xFF as u8; 32]);

        assert!(contract.particpant_chelsea == empty);
    }

    #[test]
    fn admin_is_to_caller_alice() {
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let contract = football_match::GameData::new();

        assert!(contract.admin == accounts.alice);
    }

    #[test]
    fn get_game_emits_one_event() -> Result<(), Errors> {
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let contract = football_match::GameData::new();
        let get_game_work_is_err = contract.get_game().is_err();
        if get_game_work_is_err == true {
            return Err(Errors::DontWork);
        }

        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();

        assert_eq!(emitted_events.len(), 1);
        Ok(())
    }

    #[ink::test]
    fn set_winner() {
        let mut contract = football_match::GameData::new();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(
            default_accounts::<ink::env::DefaultEnvironment>().bob,
        );
        let set_particpant_chelsea_is_err = contract.set_particpant_chelsea().is_err();
        if set_particpant_chelsea_is_err == true {
            return Err(Error::Unknown);
        }

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(
            default_accounts::<ink::env::DefaultEnvironment>().charlie,
        );
        let set_particpant_manchester_is_err = contract.set_particpant_manchester().is_err();
        if set_particpant_manchester_is_err == true {
            return Err(Error::Unknown);
        }

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(
            default_accounts::<ink::env::DefaultEnvironment>().alice,
        );
        let set_winner_is_err = contract.set_winner(1u8).is_err();
        if set_winner_is_err == true {
            return Err(Error::Unknown);
        }

        assert!(contract.winning_team == 1u8);
    }

    #[test]
    fn set_particpant_chelsea_is_caller_bob() -> Result<(), Errors> {
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let mut contract = football_match::GameData::new();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
        let set_particpant_chelsea_is_err = contract.set_particpant_chelsea().is_err();
        if set_particpant_chelsea_is_err == true {
            return Err(Errors::DontWork);
        }

        assert_eq!(contract.particpant_chelsea, accounts.bob);
        Ok(())
    }

    #[test]
    fn set_particpant_manchester_is_caller_charlie() -> Result<(), Errors> {
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let mut contract = football_match::GameData::new();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.charlie);
        let set_particpant_manchester_is_err = contract.set_particpant_manchester().is_err();
        if set_particpant_manchester_is_err == true {
            return Err(Errors::DontWork);
        }

        assert_eq!(contract.particpant_manchester, accounts.charlie);
        Ok(())
    }

    #[test]
    fn change_admin_is_changeable() -> Result<(), Errors> {
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let mut contract = football_match::GameData::new();

        let change_admin_is_err = contract.change_admin(accounts.django).is_err();
        if change_admin_is_err == true {
            return Err(Errors::DontWork);
        }

        assert!(contract.admin != accounts.alice);
        Ok(())
    }

    #[test]
    fn restart_match_sets_winner_team_0() -> Result<(), Errors> {
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let mut contract = football_match::GameData::new();

        let restart_match_is_err = contract.restart_match().is_err();
        if restart_match_is_err == true {
            return Err(Errors::DontWork);
        }
        assert!(contract.winning_team == 0u8);
        Ok(())
    }
}
