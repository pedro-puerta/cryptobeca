use crate::transaction::*;
use chrono::{DateTime, Utc};
use sha3::Digest;

/// Block struct.
///
/// Represents a block in the blockchain.
///
/// # Fields
///
/// * `timestamp` - The timestamp when the block was created. 
/// * `transactions` - The transactions included in this block.
/// * `previous_hash` - The hash of the previous block in the chain.
/// * `hash` - The hash of this block.
/// * `nonce` - The nonce used to mine this block.
#[derive(Debug)]
pub struct Block {
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    /// Creates a new Block instance.
    ///
    /// # Parameters
    ///
    /// * `transactions` - The transactions to include in the block
    /// * `previous_hash` - The hash of the previous block
    ///
    /// # Returns
    ///
    /// A new Block instance 
    ///
    /// # Functionality
    ///
    /// - Gets the current timestamp
    /// - Calculates the hash for the new block
    /// - Returns a Block with the provided transactions, previous hash, 
    ///   calculated hash, and nonce of 0
    pub fn new(transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now();
        let hash = Self::calculate_hash(&timestamp, &transactions, &previous_hash, 0);

        Self {
            timestamp,
            transactions,
            previous_hash,
            hash,
            nonce: 0,
        }
    }

    /// Calculates the hash for a block.
    ///
    /// # Parameters
    ///
    /// * `timestamp` - The timestamp of the block
    /// * `transactions` - The transactions in the block
    /// * `previous_hash` - The previous block hash  
    /// * `nonce` - The mining nonce
    ///
    /// # Returns
    ///  
    /// The SHA3-256 hash of the block details as a hex encoded string.
    ///
    /// # Functionality
    ///
    /// - Converts transactions to a string  
    /// - Concatenates transaction string, timestamp, previous hash and nonce
    /// - Feeds concatenated string into SHA3-256
    /// - Encodes the raw bytes as hex
    pub fn calculate_hash(
        timestamp: &DateTime<Utc>,
        transactions: &[Transaction],
        previous_hash: &str,
        nonce: u64,
    ) -> String {
        let transactions_str: String = transactions
            .iter()
            .map(|transaction| format!("{:?}", transaction))
            .collect();

        format!(
            "{:x}",
            sha3::Sha3_256::digest(
                format!(
                    "{}{}{}{}",
                    transactions_str,
                    timestamp.to_rfc3339(),
                    previous_hash,
                    nonce
                )
                .as_bytes()
            )
        )
    }

    /// Mines the block by finding a valid nonce.
    ///
    /// # Parameters
    ///
    /// * `difficulty` - The mining difficulty 
    ///
    /// # Returns 
    ///
    /// A success message with the block hash
    ///
    /// # Functionality
    ///
    /// - Generates a string of 0s equal to the difficulty
    /// - Increments the nonce and recalculates the hash until it starts with the 0s string
    /// - Returns a success message with the final hash
    pub fn mine_block(&mut self, difficulty: i64) -> String {
        let prefix_zeros: String = (0..difficulty).map(|_| '0').collect();

        while !self.hash.starts_with(&prefix_zeros) {
            self.nonce += 1;
            self.hash = Self::calculate_hash(
                &self.timestamp,
                &self.transactions,
                &self.previous_hash,
                self.nonce,
            );
        }

        format!("Block successfully mined: {}", self.hash)
    }

    /// Validates all transactions in the block.
    ///
    /// # Returns  
    ///
    /// `Result<bool, String>`
    ///
    /// - `Ok(true)` if all transactions are valid
    /// - `Ok(false)` if any transaction is invalid
    /// - `Err(String)` if there was an error validating a transaction
    ///
    /// # Functionality
    ///  
    /// - Iterates through each transaction
    /// - Calls transaction.is_valid() to validate
    /// - If any transaction is invalid, returns Ok(false)
    /// - If all are valid, returns Ok(true)
    /// - Prints any validation error messages
    pub fn has_valid_transactions(&self) -> Result<bool, String> {
        for transaction in &self.transactions {
            match transaction.is_valid() {
                Ok(is_valid) => {
                    if !is_valid {
                        return Ok(false);
                    }
                }
                Err(err_msg) => println!("Error validating transaction: {}", err_msg),
            }
        }
        Ok(true)
    }
}
