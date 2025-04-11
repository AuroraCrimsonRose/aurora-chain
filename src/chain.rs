use crate::block::Block;
use crate::db::BlockDatabase;
use crate::tx::Transaction;
use crate::wallet_io;
use ed25519_dalek::Signature;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Blockchain {
    pub db: BlockDatabase,
    pub length: u64,
}

impl Blockchain {
    pub fn new(db_path: &str) -> Self {
        let db = BlockDatabase::new(db_path).expect("Failed to open DB");

        let length = match db.get_block(0).unwrap() {
            Some(_) => {
                let mut i = 1;
                while let Ok(Some(_)) = db.get_block(i) {
                    i += 1;
                }
                i
            }
            None => {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                // Dummy genesis signature (you can generate real one if desired)
                let fake_sig = Signature::from_bytes(&[0u8; 64]).unwrap();

                let genesis = Block::new(
                    0,
                    timestamp,
                    "0".into(),
                    vec![],
                    "GENESIS".into(),
                    fake_sig,
                );

                db.put_block(&genesis).unwrap();
                1
            }
        };

        Blockchain { db, length }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let prev_block = self.db.get_block(self.length - 1).unwrap().unwrap();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let producer = transactions[0].from.clone();

        let hash = Block::calculate_hash(
            self.length,
            timestamp,
            &prev_block.hash,
            &transactions,
            &producer,
        );

        let wallet = wallet_io::load_wallet(&producer).expect("Validator wallet not found");
        let signature = wallet.sign(hash.as_bytes());

        let new_block = Block::new(
            self.length,
            timestamp,
            prev_block.hash.clone(),
            transactions,
            producer,
            signature,
        );

        self.db.put_block(&new_block).unwrap();
        self.length += 1;

        println!("🧱 Block {} added: {}", new_block.index, new_block.hash);
    }

    pub fn get_block(&self, index: u64) -> Option<Block> {
        self.db.get_block(index).unwrap()
    }
}
