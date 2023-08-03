use std::str::FromStr;

use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Sha3_256};

/// Transaction struct.
///
/// Represents a transaction in the blockchain.
///
/// # Fields
///
/// * `from_address` - The sender address. Optional, for mining rewards.
/// * `to_address` - The recipient address. 
/// * `amount` - The amount transferred.
/// * `signature` - The cryptographic signature of the transaction.
/// * `hash` - The hash of the transaction.
#[derive(Debug, Clone)]
pub struct Transaction {
    pub from_address: Option<String>,
    pub to_address: String,
    pub amount: f64,
    pub signature: Option<String>,
    pub hash: Option<String>,
}

/// TransactionError enum.
/// 
/// Represents the possible errors when validating a transaction.
///
/// # Variants
///
/// * `InvalidTransaction` - Returned when the transaction is invalid.
#[derive(Debug)]
pub enum TransactionError {
    InvalidTransaction,
}

impl Transaction {
    /// Calculates the hash for the transaction.
    ///
    /// # Parameters
    ///
    /// * `from_address` - The sender address 
    /// * `to_address` - The recipient address
    /// * `amount` - The amount transferred
    ///
    /// # Returns
    /// 
    /// The SHA3-256 hash of the transaction details as a hex encoded string.
    ///
    /// # Functionality
    ///
    /// - Formats the transaction details into an input string
    /// - Feeds the input string into a SHA3-256 hasher
    /// - Finalizes the hash 
    /// - Encodes the hash bytes as hex
    fn calculate_hash(
        &self,
        from_address: Option<String>,
        to_address: String,
        amount: f64,
    ) -> String {
        let mut hasher = Sha3_256::new();
        let input = format!("{:?}:{:?}:{:?}", from_address, to_address, amount);
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// Signs the transaction using the provided private key.
    ///
    /// # Parameters
    ///
    /// * `signing_key` - The private key to sign the transaction with 
    ///
    /// # Returns
    ///
    /// `Result<(), String>`
    ///
    /// - `Ok(())` if signing succeeded 
    /// - `Err(String)` containing the error message if signing failed
    ///
    /// # Functionality
    ///
    /// - Validates the provided public and private keys match
    /// - Calculates the transaction hash 
    /// - Creates a secp256k1 message from the hash 
    /// - Signs the message using the private key  
    /// - Serializes the signature to DER format
    /// - Sets the transaction signature
    pub fn sign(&mut self, signing_key: &str) -> Result<(), String> {
        if let Some(ref from_address) = self.from_address {
            let secp = Secp256k1::new();

            let public_key = PublicKey::from_str(from_address)
                .map_err(|_| "Invalid public key format".to_string())?;

            let private_key = SecretKey::from_str(signing_key)
                .map_err(|_| "Invalid private key format".to_string())?;

            let derived_public_key = PublicKey::from_secret_key(&secp, &private_key);

            if derived_public_key != public_key {
                return Err(
                    "The private key does not correspond to the provided public key".to_string(),
                );
            }

            let hash_transaction = self.calculate_hash(
                Some(
                    self.from_address
                        .clone()
                        .unwrap_or("Mining reward".to_string()),
                ),
                self.to_address.clone(),
                self.amount,
            );

            self.hash = Some(hash_transaction.clone());

            let decoded_hash =
                hex::decode(&hash_transaction).map_err(|_| "Invalid hex format".to_string())?;
            let message = match Message::from_slice(&decoded_hash) {
                Ok(message) => message,
                Err(_) => return Err("Invalid message format".to_string()),
            };

            let signature = secp.sign_ecdsa(&message, &private_key);

            let signature_bytes = signature.serialize_der();

            self.signature = Some(hex::encode(signature_bytes));

            Ok(())
        } else {
            Err("Transaction cannot be signed as it does not have a from address".to_string())
        }
    }

    /// Validates the transaction's signature.
    ///
    /// # Returns
    ///
    /// `Result<bool, String>`
    ///
    /// - `Ok(true)` if signature is valid
    /// - `Ok(false)` if no signature present 
    /// - `Err(String)` containing error message if validation failed
    ///
    /// # Functionality
    ///
    /// - Returns Ok(true) if no from_address  
    /// - Checks signature is present
    /// - Decodes signature from hex
    /// - Decodes public key from address
    /// - Decodes hash from transaction hash
    /// - Constructs secp256k1 message from hash
    /// - Verifies signature against public key & message 
    /// - Returns result of signature verification
    pub fn is_valid(&self) -> Result<bool, String> {
        if self.from_address.is_none() {
            return Ok(true);
        }
        if let Some(ref signature) = self.signature {
            if signature.is_empty() {
                return Err("No signature in this transaction".to_string());
            }

            let secp = Secp256k1::new();

            let public_key =
                PublicKey::from_str(self.from_address.as_ref().ok_or("Missing from_address")?)
                    .map_err(|_| "Invalid public key format".to_string())?;

            let message_bytes = hex::decode(
                self.hash
                    .as_ref()
                    .ok_or("Transaction hash not found".to_string())?,
            )
            .map_err(|_| "Error decoding transaction hash".to_string())?;

            let message = Message::from_slice(&message_bytes)
                .map_err(|_| "Invalid message format".to_string())?;

            let signature_bytes =
                hex::decode(signature).map_err(|_| "Invalid signature format".to_string())?;

            let signature = Signature::from_der(&signature_bytes)
                .map_err(|_| "Invalid signature".to_string())?;

            let is_valid_signature = secp.verify_ecdsa(&message, &signature, &public_key).is_ok();

            Ok(is_valid_signature)
        } else {
            Err("No signature in this transaction".to_string())
        }
    }
}
