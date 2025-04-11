use std::collections::HashMap;
use ed25519_dalek::PublicKey;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Validator {
    pub name: String,
    pub public_key: PublicKey,
    pub votes: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ValidatorRegistry {
    pub validators: HashMap<String, Validator>, // name → Validator
}

impl ValidatorRegistry {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, public_key: PublicKey) -> bool {
        if self.validators.contains_key(&name) {
            return false;
        }
        self.validators.insert(name.clone(), Validator {
            name,
            public_key,
            votes: 0,
        });
        true
    }

    pub fn vote(&mut self, name: &str) -> bool {
        if let Some(validator) = self.validators.get_mut(name) {
            validator.votes += 1;
            true
        } else {
            false
        }
    }

    pub fn top_validators(&self, count: usize) -> Vec<Validator> {
        let mut list: Vec<Validator> = self.validators.values().cloned().collect();
        list.sort_by(|a, b| b.votes.cmp(&a.votes));
        list.truncate(count);
        list
    }
}
