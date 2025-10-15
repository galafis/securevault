// Example: Basic wallet operations
//
// Run with: cargo run --example basic

use securevault::{CustodySystem, WalletType};

fn main() {
    println!("=== Basic Wallet Operations Example ===\n");

    // Create a new custody system
    let mut system = CustodySystem::new();

    // Create wallets
    println!("Creating wallets...");
    match system.create_wallet(
        "alice_hot".to_string(),
        "0xABCDEF1234567890".to_string(),
        WalletType::Hot,
    ) {
        Ok(wallet) => println!("✓ Created wallet: {} ({})", wallet.id, wallet.address),
        Err(e) => println!("✗ Failed to create wallet: {}", e),
    }

    match system.create_wallet(
        "alice_cold".to_string(),
        "0x0987654321FEDCBA".to_string(),
        WalletType::Cold,
    ) {
        Ok(wallet) => println!("✓ Created wallet: {} ({})", wallet.id, wallet.address),
        Err(e) => println!("✗ Failed to create wallet: {}", e),
    }

    // Deposit operations
    println!("\nPerforming deposits...");
    match system.deposit("alice_hot", 50.0) {
        Ok(_) => println!("✓ Deposited 50.0 BTC to alice_hot"),
        Err(e) => println!("✗ Deposit failed: {}", e),
    }

    match system.deposit("alice_cold", 200.0) {
        Ok(_) => println!("✓ Deposited 200.0 BTC to alice_cold"),
        Err(e) => println!("✗ Deposit failed: {}", e),
    }

    // Check balances
    println!("\nWallet balances:");
    for (id, wallet) in system.get_all_wallets() {
        println!(
            "  {} ({:?}): {} BTC",
            id, wallet.wallet_type, wallet.balance
        );
    }

    println!("\nTotal balance: {} BTC", system.get_total_balance());

    // Withdrawal operation
    println!("\nWithdrawing 10.0 BTC from alice_hot...");
    match system.withdraw("alice_hot", 10.0) {
        Ok(_) => println!("✓ Withdrawal successful"),
        Err(e) => println!("✗ Withdrawal failed: {}", e),
    }

    // Final balances
    println!("\nFinal wallet balances:");
    for (id, wallet) in system.get_all_wallets() {
        println!(
            "  {} ({:?}): {} BTC",
            id, wallet.wallet_type, wallet.balance
        );
    }

    println!("\nFinal total balance: {} BTC", system.get_total_balance());
}
