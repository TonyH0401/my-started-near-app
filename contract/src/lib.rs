use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Contract {
    count: u32,
    message: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            count: 0,
            message: "Default message".to_string(),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - Get the current count
    pub fn get_number(&self) -> u32 {
        self.count
    }
    pub fn get_message(&self) -> String {
        let _temp = &self.message;
        return _temp.to_string();
    }
    pub fn get_current_contract(&self) -> String {
        let _temp = "Counter: ";
        let counter = self.count.to_string();
        let message = self.message.to_string();
        let _result = _temp.to_owned() + &counter + " - Message: " + &message;
        return _result;
    }

    // Private method - Call this method to increment the count by a given number
    pub fn plus(&mut self, number: u32) {
        self.count += number;
    }
    pub fn subtract(&mut self, number: u32) {
        self.count -= number;
    }

    pub fn message(&mut self, message: String) {
        self.message = message;
    }

    pub fn default(&mut self) {
      self.count = 0;
      self.message = "Default message".to_string();
    }

    fn plus_one(&mut self) {
        self.count += 1;
    }
}
