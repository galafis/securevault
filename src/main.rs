use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub id: String,
    pub address: String,
    pub balance: f64,
    pub wallet_type: WalletType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletType {
    Hot,
    Cold,
}

#[derive(Debug)]
pub struct CustodySystem {
    wallets: HashMap<String, Wallet>,
}

impl CustodySystem {
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
        }
    }

    pub fn create_wallet(&mut self, id: String, address: String, wallet_type: WalletType) -> Wallet {
        let wallet = Wallet {
            id: id.clone(),
            address,
            balance: 0.0,
            wallet_type,
        };
        self.wallets.insert(id, wallet.clone());
        wallet
    }

    pub fn get_wallet(&self, id: &str) -> Option<&Wallet> {
        self.wallets.get(id)
    }

    pub fn deposit(&mut self, id: &str, amount: f64) -> Result<(), String> {
        if let Some(wallet) = self.wallets.get_mut(id) {
            wallet.balance += amount;
            Ok(())
        } else {
            Err("Wallet not found".to_string())
        }
    }

    pub fn withdraw(&mut self, id: &str, amount: f64) -> Result<(), String> {
        if let Some(wallet) = self.wallets.get_mut(id) {
            if wallet.balance >= amount {
                wallet.balance -= amount;
                Ok(())
            } else {
                Err("Insufficient balance".to_string())
            }
        } else {
            Err("Wallet not found".to_string())
        }
    }

    pub fn get_total_balance(&self) -> f64 {
        self.wallets.values().map(|w| w.balance).sum()
    }
}

fn main() {
    println!("🔐 SecureVault - Cryptocurrency Custody System");
    println!("==============================================\n");

    let mut system = CustodySystem::new();

    let hot_wallet = system.create_wallet(
        "hot_001".to_string(),
        "0x1234567890abcdef".to_string(),
        WalletType::Hot,
    );
    println!("✓ Created hot wallet: {} ({})", hot_wallet.id, hot_wallet.address);

    let cold_wallet = system.create_wallet(
        "cold_001".to_string(),
        "0xfedcba0987654321".to_string(),
        WalletType::Cold,
    );
    println!("✓ Created cold wallet: {} ({})", cold_wallet.id, cold_wallet.address);

    system.deposit("hot_001", 10.5).unwrap();
    println!("\n✓ Deposited 10.5 BTC to hot wallet");

    system.deposit("cold_001", 100.0).unwrap();
    println!("✓ Deposited 100.0 BTC to cold wallet");

    println!("\n📊 Wallet Balances:");
    for (id, wallet) in &system.wallets {
        println!("  {} ({:?}): {} BTC", id, wallet.wallet_type, wallet.balance);
    }

    println!("\n💰 Total Balance: {} BTC", system.get_total_balance());

    match system.withdraw("hot_001", 5.0) {
        Ok(_) => println!("\n✓ Withdrew 5.0 BTC from hot wallet"),
        Err(e) => println!("\n✗ Withdrawal failed: {}", e),
    }

    println!("\n📊 Final Total Balance: {} BTC", system.get_total_balance());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_wallet() {
        let mut system = CustodySystem::new();
        let wallet = system.create_wallet(
            "test_001".to_string(),
            "0x1234".to_string(),
            WalletType::Hot,
        );
        
        assert_eq!(wallet.id, "test_001");
        assert_eq!(wallet.address, "0x1234");
        assert_eq!(wallet.balance, 0.0);
    }

    #[test]
    fn test_deposit() {
        let mut system = CustodySystem::new();
        system.create_wallet("test_001".to_string(), "0x1234".to_string(), WalletType::Hot);
        
        let result = system.deposit("test_001", 10.5);
        assert!(result.is_ok());
        
        let wallet = system.get_wallet("test_001").unwrap();
        assert_eq!(wallet.balance, 10.5);
    }

    #[test]
    fn test_withdraw_success() {
        let mut system = CustodySystem::new();
        system.create_wallet("test_001".to_string(), "0x1234".to_string(), WalletType::Hot);
        system.deposit("test_001", 10.0).unwrap();
        
        let result = system.withdraw("test_001", 5.0);
        assert!(result.is_ok());
        
        let wallet = system.get_wallet("test_001").unwrap();
        assert_eq!(wallet.balance, 5.0);
    }

    #[test]
    fn test_withdraw_insufficient_balance() {
        let mut system = CustodySystem::new();
        system.create_wallet("test_001".to_string(), "0x1234".to_string(), WalletType::Hot);
        system.deposit("test_001", 5.0).unwrap();
        
        let result = system.withdraw("test_001", 10.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient balance");
    }

    #[test]
    fn test_total_balance() {
        let mut system = CustodySystem::new();
        system.create_wallet("hot_001".to_string(), "0x1234".to_string(), WalletType::Hot);
        system.create_wallet("cold_001".to_string(), "0x5678".to_string(), WalletType::Cold);
        
        system.deposit("hot_001", 10.5).unwrap();
        system.deposit("cold_001", 100.0).unwrap();
        
        assert_eq!(system.get_total_balance(), 110.5);
    }

    #[test]
    fn test_withdraw_from_nonexistent_wallet() {
        let mut system = CustodySystem::new();
        
        let result = system.withdraw("nonexistent", 10.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Wallet not found");
    }
}
