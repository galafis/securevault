use securevault::{CustodySystem, WalletType};

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
    for (id, wallet) in system.get_all_wallets() {
        println!("  {} ({:?}): {} BTC", id, wallet.wallet_type, wallet.balance);
    }

    println!("\n💰 Total Balance: {} BTC", system.get_total_balance());

    match system.withdraw("hot_001", 5.0) {
        Ok(_) => println!("\n✓ Withdrew 5.0 BTC from hot wallet"),
        Err(e) => println!("\n✗ Withdrawal failed: {}", e),
    }

    println!("\n📊 Final Total Balance: {} BTC", system.get_total_balance());
}

