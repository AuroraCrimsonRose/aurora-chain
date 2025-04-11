use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

//#[derive(Debug, Clone)]
pub struct Wallet {
    pub name: String,
    pub keypair: Keypair,
}
impl Wallet {
    pub fn generate(name: &str) -> Self {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        Wallet {
            name: name.to_string(),
            keypair,
        }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.keypair.sign(message)
    }

    pub fn public_key(&self) -> PublicKey {
        self.keypair.public
    }

    pub fn verify(public_key: &PublicKey, message: &[u8], sig: &Signature) -> bool {
        public_key.verify(message, sig).is_ok()
    }
}
