// 📦 src/main.rs

mod block;
mod chain;
mod db;
mod ledger;
mod tx;
mod validator;
mod wallet;
mod wallet_io;

use block::Block;
use chain::Blockchain;
use ledger::Ledger;
use std::io::{self, Write};
use tx::Transaction;
use validator::ValidatorRegistry;
use wallet_io::{load_wallet, save_wallet};

fn main() {
    let mut chain = Blockchain::new("chain_data");
    let mut validator_registry = ValidatorRegistry::new();

    loop {
        print!("aurora> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input.");
            continue;
        }

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "gen-wallet" => {
                if parts.len() != 2 {
                    println!("Usage: gen-wallet <name>");
                    continue;
                }
                let name = parts[1];
                let wallet = wallet::Wallet::generate(name);
                save_wallet(&wallet);
                println!("🔐 Wallet '{}' generated and saved!", name);
            }

            "send" => {
                if parts.len() != 4 {
                    println!("Usage: send <from> <to> <amount>");
                    continue;
                }

                let from = parts[1].to_string();
                let to = parts[2].to_string();
                let amount = match parts[3].parse::<u64>() {
                    Ok(v) => v,
                    Err(_) => {
                        println!("Invalid amount.");
                        continue;
                    }
                };

                let Some(wallet) = load_wallet(&from) else {
                    println!("❌ Wallet '{}' not found.", from);
                    continue;
                };

                let tx = Transaction {
                    from: from.clone(),
                    to: to.clone(),
                    amount,
                    public_key: wallet.public_key(),
                    signature: wallet.sign(format!("{}{}{}", from, to, amount).as_bytes()),
                };

                let mut ledger = Ledger::new();
                for i in 0..chain.length {
                    let block = chain.get_block(i).unwrap();
                    ledger.apply_block(&block).unwrap_or_else(|e| println!("⚠️ Block {} skipped: {}", i, e));
                }

                let sender_balance = ledger.get_balance(&from);
                if from != "GENESIS" && sender_balance < amount {
                    println!("❌ Insufficient funds: {} < {}", sender_balance, amount);
                    continue;
                }

                // ✅ Validator check
                let top_validators = validator_registry.top_validators(3);
                if !top_validators.iter().any(|v| v.name == from) {
                    println!("❌ '{}' is not a current validator. Cannot produce blocks.", from);
                    continue;
                }

                let prev_block = chain.get_block(chain.length - 1).unwrap();
                let timestamp = chrono::Utc::now().timestamp() as u64;

                let hash = Block::calculate_hash(
                    chain.length,
                    timestamp,
                    &prev_block.hash,
                    &vec![tx.clone()],
                    &from,
                );

                let signature = wallet.sign(hash.as_bytes());

                let block = Block::new(
                    chain.length,
                    timestamp,
                    prev_block.hash.clone(),
                    vec![tx],
                    from.clone(),
                    signature,
                );

                if !wallet::Wallet::verify(&wallet.public_key(), block.hash.as_bytes(), &block.signature) {
                    println!("❌ Block signature invalid. Block rejected.");
                    continue;
                }

                chain.db.put_block(&block).unwrap();
                chain.length += 1;
                println!("✅ Block {} added: {}", block.index, block.hash);
            }

            "balance" => {
                if parts.len() != 2 {
                    println!("Usage: balance <address>");
                    continue;
                }
                let address = parts[1];
                let mut ledger = Ledger::new();
                for i in 0..chain.length {
                    let block = chain.get_block(i).unwrap();
                    ledger.apply_block(&block).unwrap_or_else(|e| println!("⚠️ Block {} skipped: {}", i, e));
                }
                let balance = ledger.get_balance(address);
                println!("💰 {} has {} ACR", address, balance);
            }

            "register-validator" => {
                if parts.len() != 2 {
                    println!("Usage: register-validator <name>");
                    continue;
                }
                let name = parts[1];
                let Some(wallet) = load_wallet(name) else {
                    println!("❌ Wallet '{}' not found.", name);
                    continue;
                };

                if validator_registry.register(name.to_string(), wallet.public_key()) {
                    println!("✅ '{}' registered as validator.", name);
                } else {
                    println!("⚠️ '{}' is already registered.", name);
                }
            }

            "vote" => {
                if parts.len() != 3 {
                    println!("Usage: vote <voter> <validator>");
                    continue;
                }
                let voter = parts[1];
                let validator = parts[2];
                if load_wallet(voter).is_none() {
                    println!("❌ Voter wallet '{}' not found.", voter);
                    continue;
                }
                if validator_registry.vote(validator) {
                    println!("🗳️ '{}' voted for '{}'", voter, validator);
                } else {
                    println!("❌ Validator '{}' not found.", validator);
                }
            }

            "validators" => {
                let top = validator_registry.top_validators(3);
                println!("🏆 Top Validators:");
                for v in top {
                    println!("- {} ({} votes)", v.name, v.votes);
                }
            }

            "chain" => {
                for i in 0..chain.length {
                    let block = chain.get_block(i).unwrap();
                    println!("Block {}: {:#?}", i, block);
                }
            }

            "help" => {
                println!("\n📘 AuroraChain CLI Commands:");
                println!("─────────────────────────────────────────────");
                println!("gen-wallet <name>          → Create a new wallet");
                println!("send <from> <to> <amount>  → Send ACR (validator-only)");
                println!("balance <address>          → Show wallet balance");
                println!("register-validator <name>  → Register as a validator");
                println!("vote <voter> <validator>   → Vote for a validator");
                println!("validators                 → Show top validators");
                println!("chain                      → Print entire blockchain");
                println!("help                       → Show this help menu");
                println!("exit / quit                → Exit the CLI");
                println!("─────────────────────────────────────────────\n");
            }

            "exit" | "quit" => {
                println!("👋 Exiting AuroraChain.");
                break;
            }

            _ => println!("Unknown command. Type 'help' for available commands."),
        }
    }
}