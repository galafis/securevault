use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a cryptocurrency wallet in the custody system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Wallet {
    pub id: String,
    pub address: String,
    pub balance: f64,
    pub wallet_type: WalletType,
}

/// Represents the type of wallet: Hot (operational) or Cold (storage)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WalletType {
    /// Hot wallet for operational use with frequent transactions
    Hot,
    /// Cold wallet for long-term secure storage
    Cold,
}

/// Represents a transaction in the audit trail
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub wallet_id: String,
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub timestamp: u64,
}

/// Type of transaction: Deposit or Withdrawal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
}

/// Main custody system that manages wallets and transactions
#[derive(Debug)]
pub struct CustodySystem {
    wallets: HashMap<String, Wallet>,
    transactions: Vec<Transaction>,
}

impl Default for CustodySystem {
    fn default() -> Self {
        Self::new()
    }
}

impl CustodySystem {
    /// Creates a new custody system
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
            transactions: Vec::new(),
        }
    }

    /// Creates a new wallet in the custody system
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the wallet
    /// * `address` - Cryptocurrency address
    /// * `wallet_type` - Type of wallet (Hot or Cold)
    ///
    /// # Returns
    /// The created wallet
    ///
    /// # Example
    /// ```
    /// use securevault::{CustodySystem, WalletType};
    /// let mut system = CustodySystem::new();
    /// let wallet = system.create_wallet(
    ///     "wallet_001".to_string(),
    ///     "0x1234".to_string(),
    ///     WalletType::Hot
    /// );
    /// ```
    pub fn create_wallet(
        &mut self,
        id: String,
        address: String,
        wallet_type: WalletType,
    ) -> Result<Wallet, String> {
        if self.wallets.contains_key(&id) {
            return Err(format!("Wallet with id '{}' already exists", id));
        }

        let wallet = Wallet {
            id: id.clone(),
            address,
            balance: 0.0,
            wallet_type,
        };
        self.wallets.insert(id, wallet.clone());
        Ok(wallet)
    }

    /// Gets a wallet by its ID
    pub fn get_wallet(&self, id: &str) -> Option<&Wallet> {
        self.wallets.get(id)
    }

    /// Deposits funds to a wallet
    ///
    /// # Arguments
    /// * `id` - Wallet identifier
    /// * `amount` - Amount to deposit
    ///
    /// # Returns
    /// Ok(()) on success, Err with message on failure
    pub fn deposit(&mut self, id: &str, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be positive".to_string());
        }

        if let Some(wallet) = self.wallets.get_mut(id) {
            wallet.balance += amount;

            // Record transaction
            self.transactions.push(Transaction {
                wallet_id: id.to_string(),
                transaction_type: TransactionType::Deposit,
                amount,
                timestamp: Self::current_timestamp(),
            });

            Ok(())
        } else {
            Err(format!("Wallet '{}' not found", id))
        }
    }

    /// Withdraws funds from a wallet
    ///
    /// # Arguments
    /// * `id` - Wallet identifier
    /// * `amount` - Amount to withdraw
    ///
    /// # Returns
    /// Ok(()) on success, Err with message on failure
    pub fn withdraw(&mut self, id: &str, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be positive".to_string());
        }

        if let Some(wallet) = self.wallets.get_mut(id) {
            if wallet.balance >= amount {
                wallet.balance -= amount;

                // Record transaction
                self.transactions.push(Transaction {
                    wallet_id: id.to_string(),
                    transaction_type: TransactionType::Withdrawal,
                    amount,
                    timestamp: Self::current_timestamp(),
                });

                Ok(())
            } else {
                Err(format!(
                    "Insufficient balance: {} available, {} requested",
                    wallet.balance, amount
                ))
            }
        } else {
            Err(format!("Wallet '{}' not found", id))
        }
    }

    /// Gets the total balance across all wallets
    pub fn get_total_balance(&self) -> f64 {
        self.wallets.values().map(|w| w.balance).sum()
    }

    /// Gets all wallets in the system
    pub fn get_all_wallets(&self) -> &HashMap<String, Wallet> {
        &self.wallets
    }

    /// Gets transaction history for a specific wallet
    pub fn get_wallet_transactions(&self, wallet_id: &str) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|t| t.wallet_id == wallet_id)
            .collect()
    }

    /// Gets all transactions in the system
    pub fn get_all_transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    /// Gets the number of wallets in the system
    pub fn wallet_count(&self) -> usize {
        self.wallets.len()
    }

    /// Checks if a wallet exists
    pub fn wallet_exists(&self, id: &str) -> bool {
        self.wallets.contains_key(id)
    }

    /// Transfers funds between wallets
    pub fn transfer(&mut self, from_id: &str, to_id: &str, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Transfer amount must be positive".to_string());
        }

        // Validate both wallets exist first
        if !self.wallet_exists(from_id) {
            return Err(format!("Source wallet '{}' not found", from_id));
        }
        if !self.wallet_exists(to_id) {
            return Err(format!("Destination wallet '{}' not found", to_id));
        }

        // Check source balance
        let source_balance = self.get_wallet(from_id).unwrap().balance;
        if source_balance < amount {
            return Err(format!(
                "Insufficient balance in source wallet: {} available, {} requested",
                source_balance, amount
            ));
        }

        // Perform transfer
        self.withdraw(from_id, amount)?;
        self.deposit(to_id, amount)?;

        Ok(())
    }

    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_wallet() {
        let mut system = CustodySystem::new();
        let wallet = system
            .create_wallet(
                "test_001".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();

        assert_eq!(wallet.id, "test_001");
        assert_eq!(wallet.address, "0x1234");
        assert_eq!(wallet.balance, 0.0);
    }

    #[test]
    fn test_create_duplicate_wallet() {
        let mut system = CustodySystem::new();
        system
            .create_wallet(
                "test_001".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();

        let result = system.create_wallet(
            "test_001".to_string(),
            "0x5678".to_string(),
            WalletType::Cold,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }

    #[test]
    fn test_deposit() {
        let mut system = CustodySystem::new();
        system
            .create_wallet(
                "test_001".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();

        let result = system.deposit("test_001", 10.5);
        assert!(result.is_ok());

        let wallet = system.get_wallet("test_001").unwrap();
        assert_eq!(wallet.balance, 10.5);
    }

    #[test]
    fn test_deposit_negative_amount() {
        let mut system = CustodySystem::new();
        system
            .create_wallet(
                "test_001".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();

        let result = system.deposit("test_001", -10.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("positive"));
    }

    #[test]
    fn test_deposit_zero_amount() {
        let mut system = CustodySystem::new();
        system
            .create_wallet(
                "test_001".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();

        let result = system.deposit("test_001", 0.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("positive"));
    }

    #[test]
    fn test_withdraw_success() {
        let mut system = CustodySystem::new();
        system
            .create_wallet(
                "test_001".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();
        system.deposit("test_001", 10.0).unwrap();

        let result = system.withdraw("test_001", 5.0);
        assert!(result.is_ok());

        let wallet = system.get_wallet("test_001").unwrap();
        assert_eq!(wallet.balance, 5.0);
    }

    #[test]
    fn test_withdraw_insufficient_balance() {
        let mut system = CustodySystem::new();
        system
            .create_wallet(
                "test_001".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();
        system.deposit("test_001", 5.0).unwrap();

        let result = system.withdraw("test_001", 10.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Insufficient balance"));
    }

    #[test]
    fn test_withdraw_negative_amount() {
        let mut system = CustodySystem::new();
        system
            .create_wallet(
                "test_001".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();
        system.deposit("test_001", 10.0).unwrap();

        let result = system.withdraw("test_001", -5.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("positive"));
    }

    #[test]
    fn test_total_balance() {
        let mut system = CustodySystem::new();
        system
            .create_wallet("hot_001".to_string(), "0x1234".to_string(), WalletType::Hot)
            .unwrap();
        system
            .create_wallet(
                "cold_001".to_string(),
                "0x5678".to_string(),
                WalletType::Cold,
            )
            .unwrap();

        system.deposit("hot_001", 10.5).unwrap();
        system.deposit("cold_001", 100.0).unwrap();

        assert_eq!(system.get_total_balance(), 110.5);
    }

    #[test]
    fn test_withdraw_from_nonexistent_wallet() {
        let mut system = CustodySystem::new();

        let result = system.withdraw("nonexistent", 10.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_deposit_to_nonexistent_wallet() {
        let mut system = CustodySystem::new();

        let result = system.deposit("nonexistent", 10.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_transaction_history() {
        let mut system = CustodySystem::new();
        system
            .create_wallet(
                "test_001".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();

        system.deposit("test_001", 10.0).unwrap();
        system.withdraw("test_001", 3.0).unwrap();
        system.deposit("test_001", 5.0).unwrap();

        let transactions = system.get_wallet_transactions("test_001");
        assert_eq!(transactions.len(), 3);
        assert_eq!(transactions[0].amount, 10.0);
        assert_eq!(transactions[1].amount, 3.0);
        assert_eq!(transactions[2].amount, 5.0);
    }

    #[test]
    fn test_wallet_exists() {
        let mut system = CustodySystem::new();
        system
            .create_wallet(
                "test_001".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();

        assert!(system.wallet_exists("test_001"));
        assert!(!system.wallet_exists("test_002"));
    }

    #[test]
    fn test_wallet_count() {
        let mut system = CustodySystem::new();
        assert_eq!(system.wallet_count(), 0);

        system
            .create_wallet(
                "test_001".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();
        assert_eq!(system.wallet_count(), 1);

        system
            .create_wallet(
                "test_002".to_string(),
                "0x5678".to_string(),
                WalletType::Cold,
            )
            .unwrap();
        assert_eq!(system.wallet_count(), 2);
    }

    #[test]
    fn test_transfer_success() {
        let mut system = CustodySystem::new();
        system
            .create_wallet(
                "wallet_1".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();
        system
            .create_wallet(
                "wallet_2".to_string(),
                "0x5678".to_string(),
                WalletType::Cold,
            )
            .unwrap();

        system.deposit("wallet_1", 100.0).unwrap();
        let result = system.transfer("wallet_1", "wallet_2", 30.0);
        assert!(result.is_ok());

        assert_eq!(system.get_wallet("wallet_1").unwrap().balance, 70.0);
        assert_eq!(system.get_wallet("wallet_2").unwrap().balance, 30.0);
    }

    #[test]
    fn test_transfer_insufficient_balance() {
        let mut system = CustodySystem::new();
        system
            .create_wallet(
                "wallet_1".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();
        system
            .create_wallet(
                "wallet_2".to_string(),
                "0x5678".to_string(),
                WalletType::Cold,
            )
            .unwrap();

        system.deposit("wallet_1", 10.0).unwrap();
        let result = system.transfer("wallet_1", "wallet_2", 30.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Insufficient balance"));
    }

    #[test]
    fn test_transfer_nonexistent_source() {
        let mut system = CustodySystem::new();
        system
            .create_wallet(
                "wallet_2".to_string(),
                "0x5678".to_string(),
                WalletType::Cold,
            )
            .unwrap();

        let result = system.transfer("wallet_1", "wallet_2", 30.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_transfer_nonexistent_destination() {
        let mut system = CustodySystem::new();
        system
            .create_wallet(
                "wallet_1".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();

        system.deposit("wallet_1", 100.0).unwrap();
        let result = system.transfer("wallet_1", "wallet_2", 30.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_transfer_negative_amount() {
        let mut system = CustodySystem::new();
        system
            .create_wallet(
                "wallet_1".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();
        system
            .create_wallet(
                "wallet_2".to_string(),
                "0x5678".to_string(),
                WalletType::Cold,
            )
            .unwrap();

        system.deposit("wallet_1", 100.0).unwrap();
        let result = system.transfer("wallet_1", "wallet_2", -30.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("positive"));
    }

    #[test]
    fn test_get_all_transactions() {
        let mut system = CustodySystem::new();
        system
            .create_wallet(
                "wallet_1".to_string(),
                "0x1234".to_string(),
                WalletType::Hot,
            )
            .unwrap();

        system.deposit("wallet_1", 10.0).unwrap();
        system.withdraw("wallet_1", 3.0).unwrap();

        let transactions = system.get_all_transactions();
        assert_eq!(transactions.len(), 2);
    }

    #[test]
    fn test_default_implementation() {
        let system = CustodySystem::default();
        assert_eq!(system.wallet_count(), 0);
        assert_eq!(system.get_total_balance(), 0.0);
    }
}
