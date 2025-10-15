// Example: Transaction history and audit trail
//
// Run with: cargo run --example transaction_history

use securevault::{CustodySystem, TransactionType, WalletType};

fn main() {
    println!("=== Transaction History Example ===\n");

    let mut system = CustodySystem::new();

    // Create a wallet
    println!("Creating wallet...");
    system
        .create_wallet(
            "trader_wallet".to_string(),
            "0xTRADER123456789".to_string(),
            WalletType::Hot,
        )
        .unwrap();
    println!("✓ Created trader_wallet\n");

    // Perform various operations
    println!("Performing transactions...");
    system.deposit("trader_wallet", 100.0).unwrap();
    println!("✓ Deposited 100.0 BTC");

    system.withdraw("trader_wallet", 25.0).unwrap();
    println!("✓ Withdrew 25.0 BTC");

    system.deposit("trader_wallet", 50.0).unwrap();
    println!("✓ Deposited 50.0 BTC");

    system.withdraw("trader_wallet", 30.0).unwrap();
    println!("✓ Withdrew 30.0 BTC");

    system.deposit("trader_wallet", 15.0).unwrap();
    println!("✓ Deposited 15.0 BTC");

    // Show current balance
    let wallet = system.get_wallet("trader_wallet").unwrap();
    println!("\nCurrent balance: {} BTC", wallet.balance);

    // Display transaction history
    println!("\n=== Transaction History ===");
    let transactions = system.get_wallet_transactions("trader_wallet");

    let mut total_deposits = 0.0;
    let mut total_withdrawals = 0.0;

    for (i, tx) in transactions.iter().enumerate() {
        let tx_type = match tx.transaction_type {
            TransactionType::Deposit => {
                total_deposits += tx.amount;
                "DEPOSIT   "
            }
            TransactionType::Withdrawal => {
                total_withdrawals += tx.amount;
                "WITHDRAWAL"
            }
        };

        println!(
            "{}. {} | Amount: {:>8.2} BTC | Timestamp: {}",
            i + 1,
            tx_type,
            tx.amount,
            tx.timestamp
        );
    }

    // Summary
    println!("\n=== Summary ===");
    println!("Total transactions: {}", transactions.len());
    println!("Total deposits: {:.2} BTC", total_deposits);
    println!("Total withdrawals: {:.2} BTC", total_withdrawals);
    println!("Net change: {:.2} BTC", total_deposits - total_withdrawals);
    println!("Current balance: {:.2} BTC", wallet.balance);

    // Verify balance matches transaction history
    let calculated_balance = total_deposits - total_withdrawals;
    if (calculated_balance - wallet.balance).abs() < 0.001 {
        println!("\n✓ Balance verified against transaction history");
    } else {
        println!("\n✗ Balance mismatch detected!");
    }
}
