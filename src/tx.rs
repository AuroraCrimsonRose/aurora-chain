use ed25519_dalek::{PublicKey, Signature};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub public_key: PublicKey,
    pub signature: Signature,
}
