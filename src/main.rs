use blockchain::*;
use transaction::*;
use std::env;
mod block;
mod blockchain;
mod transaction;

fn main() {
    dotenv::dotenv().ok();

    let my_key: &str = &env::var("PRIVATE_KEY").unwrap_or("Invalid PRIVATE_KEY".to_string());
    let my_wallet_address: &str =
        &env::var("PUBLIC_KEY").unwrap_or("Invalid PUBLIC_KEY".to_string());

    let mut blockchain = Blockchain::new(2, 100.0);

    let mut transaction = Transaction {
        from_address: Some(my_wallet_address.to_string()),
        to_address: "public key of someone's address".to_string(),
        amount: 10.0,
        signature: None,
        hash: None,
    };

    match transaction.sign(my_key) {
        Ok(()) => {
            println!("Transaction signed successfully!");
            println!("Transaction with signature: {:#?}", transaction);
        }
        Err(err) => println!("Error signing transaction: {}", err),
    }

    match transaction.is_valid() {
        Ok(valid) => {
            if valid {
                println!("Transaction is valid.");
            } else {
                println!("Transaction is NOT valid.");
            }
        }
        Err(err) => println!("Error verifying transaction: {}", err),
    }

    match blockchain.add_transaction(transaction) {
        Ok(()) => {
            println!("Transaction added to the chain pending transactions!");
        }
        Err(_err) => println!("Invalid Transaction!"),
    }

    println!("Starting the miner...");

    blockchain.mine_pending_transactions(my_wallet_address.to_string());

    println!(
        "Balance of my wallet address: {}",
        blockchain.get_balance_of_address(my_wallet_address)
    );

    println!("Is the chain valid? {}", blockchain.is_valid());
}
