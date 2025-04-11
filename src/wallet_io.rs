use crate::wallet::Wallet;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use std::{fs, io::Write, path::Path};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct WalletFile {
    pub name: String,
    pub secret: Vec<u8>,
    pub public: Vec<u8>,
}

impl WalletFile {
    pub fn from_wallet(wallet: &Wallet) -> Self {
        WalletFile {
            name: wallet.name.clone(),
            secret: wallet.keypair.secret.to_bytes().to_vec(),
            public: wallet.keypair.public.to_bytes().to_vec(),
        }
    }

    pub fn to_wallet(&self) -> Wallet {
        let secret = ed25519_dalek::SecretKey::from_bytes(&self.secret).unwrap();
        let public = ed25519_dalek::PublicKey::from_bytes(&self.public).unwrap();
        let keypair = Keypair { secret, public };
        Wallet {
            name: self.name.clone(),
            keypair,
        }
    }
}

pub fn save_wallet(wallet: &Wallet) {
    fs::create_dir_all("wallets").unwrap();
    let file = format!("wallets/{}.json", wallet.name.to_lowercase());
    let data = serde_json::to_string_pretty(&WalletFile::from_wallet(wallet)).unwrap();
    fs::write(file, data).unwrap();
}

pub fn load_wallet(name: &str) -> Option<Wallet> {
    let file = format!("wallets/{}.json", name.to_lowercase());
    if !Path::new(&file).exists() {
        return None;
    }
    let data = fs::read_to_string(file).ok()?;
    let wf: WalletFile = serde_json::from_str(&data).ok()?;
    Some(wf.to_wallet())
}
