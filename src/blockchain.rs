use crate::block::*;
use crate::transaction::*;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: i64,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: f64,
}

impl Blockchain {
    pub fn new(difficulty: i64, mining_reward: f64) -> Self {
        let genesis_block = Block::new(vec![], "0".to_string());
        let chain = vec![genesis_block];
        Self {
            chain,
            difficulty,
            pending_transactions: vec![],
            mining_reward,
        }
    }

    fn get_latest_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn mine_pending_transactions(&mut self, mining_reward_address: String) {
        let reward_transaction = Transaction {
            from_address: None,
            to_address: mining_reward_address,
            amount: self.mining_reward,
            signature: None,
            hash: None,
        };
        self.pending_transactions.push(reward_transaction);

        let prev_block_hash = match self.get_latest_block() {
            Some(block) => block.hash.clone(),
            None => String::from("GenesisBlockHash"),
        };

        let mut block = Block::new(self.pending_transactions.clone(), prev_block_hash);
        block.mine_block(self.difficulty);

        self.chain.push(block);
        self.pending_transactions = vec![];
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), TransactionError> {
        if transaction.from_address.is_none() || transaction.to_address.is_empty() {
            return Err(TransactionError::InvalidTransaction);
        }

        match transaction.is_valid() {
            Ok(is_valid) => {
                if !is_valid {
                    return Err(TransactionError::InvalidTransaction);
                }
            }
            Err(err_msg) => println!("Error validating transaction: {}", err_msg),
        }

        self.pending_transactions.push(transaction);
        Ok(())
    }

    pub fn get_balance_of_address(&self, address: &str) -> f64 {
        let balance = self.chain.iter().fold(0.0, |acc, block| {
            block.transactions.iter().fold(acc, |acc, transaction| {
                if transaction.to_address == address {
                    acc + transaction.amount
                } else if transaction.from_address.as_deref() == Some(address) {
                    acc - transaction.amount
                } else {
                    acc
                }
            })
        });
        balance
    }

    pub fn is_valid(&self) -> bool {
        self.chain
            .iter()
            .zip(self.chain.iter().skip(1))
            .all(|(current_block, next_block)| {
                current_block.hash
                    == Block::calculate_hash(
                        &current_block.timestamp,
                        &current_block.transactions,
                        &current_block.previous_hash,
                        current_block.nonce,
                    )
                    && current_block.hash == next_block.previous_hash
                    && current_block.has_valid_transactions().unwrap_or(false)
            })
    }
}
