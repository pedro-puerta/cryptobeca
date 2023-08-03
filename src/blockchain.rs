use crate::block::*;
use crate::transaction::*;

/// Blockchain struct.
///
/// Represents the blockchain. 
///
/// # Fields
///
/// * `chain` - The chain of mined blocks
/// * `difficulty` - The mining difficulty 
/// * `pending_transactions` - Unmined transactions  
/// * `mining_reward` - The mining reward amount
#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: i64,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: f64,
}

impl Blockchain {
    /// Creates a new Blockchain instance.
    ///
    /// # Parameters
    ///
    /// * `difficulty` - The mining difficulty
    /// * `mining_reward` - The mining reward amount  
    ///
    /// # Returns
    ///
    /// A new Blockchain instance.
    ///
    /// # Functionality
    /// 
    /// - Creates a genesis block with no transactions and hash "0"
    /// - Initializes a chain with just the genesis block
    /// - Sets the provided difficulty and mining reward  
    /// - Returns the initialized Blockchain
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

    /// Gets the latest block in the chain.
    ///
    /// # Returns
    ///
    /// Option<&Block> - The latest block if available, else None.
    /// 
    /// # Functionality
    ///
    /// - Calls last() on the chain to get the latest block
    /// - Returns the block wrapped in Some(), or None if chain is empty
    fn get_latest_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    /// Mines pending transactions into a new block.
    ///
    /// # Parameters
    ///
    /// * `mining_reward_address`: The address to send the mining reward to.
    ///
    /// # Functionality
    ///
    /// - Creates a reward transaction to the provided address  
    /// - Gets previous block hash
    /// - Creates a new block with pending transactions 
    /// - Mines the block by finding a valid nonce
    /// - Adds the mined block to the chain
    /// - Resets pending transactions
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

    /// Adds a transaction to the blockchain pending transactions.
    ///
    /// # Parameters
    ///
    /// * `transaction` - The transaction to add
    ///
    /// # Returns  
    ///
    /// `Result<(), TransactionError>`
    ///
    /// - `Ok(())` if the transaction was added successfully
    /// - `Err(TransactionError)` if the transaction is invalid
    ///
    /// # Functionality
    ///
    /// - Validates the transaction fields are present
    /// - Calls transaction.is_valid() to validate the signature  
    /// - If valid, adds the transaction to pending_transactions
    /// - Returns a result indicating if the transaction was added
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

    /// Gets the balance for the provided address by iterating through the blockchain.
    ///
    /// # Parameters
    ///
    /// * `address` - The address to get the balance for
    ///
    /// # Returns
    ///  
    /// The current balance of the address as a f64
    ///
    /// # Functionality
    ///
    /// - Initializes the balance to 0.0
    /// - Iterates through each block in the chain
    ///   - In each block, iterates through the transactions
    ///     - If the address is the recipient, add the amount to the balance
    ///     - If the address is the sender, subtract the amount from the balance
    /// - Returns the calculated balance
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

    /// Validates the blockchain by checking:
    ///
    /// - The hash of each block matches the calculation
    /// - The previous hash matches the next block
    /// - Each block has valid transactions
    ///
    /// # Returns
    ///
    /// bool - True if the blockchain is valid, False otherwise
    ///
    /// # Functionality  
    ///
    /// - Zips the chain with itself offset by 1 to pair blocks
    /// - For each pair:
    ///   - Checks hash matches recalculation
    ///   - Checks previous hash matches next hash
    ///   - Checks block transactions are valid
    /// - Returns true if all checks pass, false otherwise
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
