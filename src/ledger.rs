use std::collections::HashMap;
use crate::tx::Transaction;
use crate::block::Block;

pub struct Ledger {
    pub balances: HashMap<String, u64>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            balances: HashMap::new(),
        }
    }

    // Replay blocks to rebuild the ledger
    pub fn apply_block(&mut self, block: &Block) -> Result<(), String> {
        for tx in &block.transactions {
            if tx.from != "GENESIS" {
                let sender_balance = self.balances.get(&tx.from).copied().unwrap_or(0);
                if sender_balance < tx.amount {
                    return Err(format!(
                        "Insufficient funds: {} has {}, tried to send {}",
                        tx.from, sender_balance, tx.amount
                    ));
                }
                self.balances.insert(tx.from.clone(), sender_balance - tx.amount);
            }

            let receiver_balance = self.balances.get(&tx.to).copied().unwrap_or(0);
            self.balances.insert(tx.to.clone(), receiver_balance + tx.amount);
        }
        Ok(())
    }

    pub fn get_balance(&self, name: &str) -> u64 {
        self.balances.get(name).copied().unwrap_or(0)
    }
}
