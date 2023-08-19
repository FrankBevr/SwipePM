#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod testo {

    #[ink(storage)]
    pub struct Testo {
        charlie: AccountId,
        alice: AccountId,
    }

    impl Testo {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                charlie: AccountId::from([0xFF as u8; 32]),
                alice: AccountId::from([0xFF as u8; 32]),
            }
        }

        #[ink(message)]
        pub fn get_charlie(&self) {
            ink::env::debug_println!("This is Charlie {:?}", self.charlie)
        }

        #[ink(message)]
        pub fn set_charlie(&mut self) {
            self.charlie = self.env().caller();
        }

        #[ink(message)]
        pub fn get_alice(&self) {
            ink::env::debug_println!("This is Alice {:?}", self.alice)
        }

        #[ink(message)]
        pub fn set_alice(&mut self) {
            self.alice = self.env().caller();
        }

        #[ink(message, payable)]
        pub fn charlie_sends_contract(&self) {
            if self.charlie == self.env().caller() {
                let _ = self.env().transferred_value();
            }
        }

        #[ink(message, payable)]
        pub fn alice_sends_contract(&self) {
            if self.alice == self.env().caller() {
                let _ = self.env().transferred_value();
            }
        }

        #[ink(message)]
        pub fn get_contract_balance(&self) {
            ink::env::debug_println!("Current Contract Balance {:?}", self.env().balance())
        }

        #[ink(message, payable)]
        pub fn contract_sends_alice(&self) {
            if self.alice == self.env().caller() {
                let _ = self.env().transfer(self.alice, self.env().balance() - 1_000_000_000_000_000);
            }
        }
    }
}
