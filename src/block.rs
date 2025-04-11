use crate::tx::Transaction;
use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};
use ed25519_dalek::{Signature, PublicKey};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: String,
    pub transactions: Vec<Transaction>,
    pub producer: String,
    pub hash: String,
    pub signature: Signature,
}

impl Block {
    pub fn new(
        index: u64,
        timestamp: u64,
        previous_hash: String,
        transactions: Vec<Transaction>,
        producer: String,
        signature: Signature,
    ) -> Self {
        let hash = Self::calculate_hash(
            index,
            timestamp,
            &previous_hash,
            &transactions,
            &producer,
        );

        Block {
            index,
            timestamp,
            previous_hash,
            transactions,
            producer,
            hash,
            signature,
        }
    }

    pub fn calculate_hash(
        index: u64,
        timestamp: u64,
        previous_hash: &str,
        transactions: &Vec<Transaction>,
        producer: &str,
    ) -> String {
        let tx_data = serde_json::to_string(transactions).unwrap();
        let input = format!(
            "{}{}{}{}{}",
            index,
            timestamp,
            previous_hash,
            tx_data,
            producer
        );

        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
