use crate::transaction::*;
use chrono::{DateTime, Utc};
use sha3::Digest;

#[derive(Debug)]
pub struct Block {
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
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
