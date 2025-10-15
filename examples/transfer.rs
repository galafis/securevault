// Example: Transfer operations between wallets
//
// Run with: cargo run --example transfer

use securevault::{CustodySystem, WalletType};

fn main() {
    println!("=== Transfer Operations Example ===\n");

    let mut system = CustodySystem::new();

    // Setup wallets
    println!("Setting up wallets...");
    system
        .create_wallet(
            "operations".to_string(),
            "0x1111111111111111".to_string(),
            WalletType::Hot,
        )
        .unwrap();

    system
        .create_wallet(
            "savings".to_string(),
            "0x2222222222222222".to_string(),
            WalletType::Cold,
        )
        .unwrap();

    system
        .create_wallet(
            "backup".to_string(),
            "0x3333333333333333".to_string(),
            WalletType::Cold,
        )
        .unwrap();

    println!("✓ Created 3 wallets: operations (Hot), savings (Cold), backup (Cold)\n");

    // Initial deposit to operations wallet
    println!("Initial deposit...");
    system.deposit("operations", 100.0).unwrap();
    println!("✓ Deposited 100.0 BTC to operations wallet\n");

    // Show initial state
    print_balances(&system);

    // Transfer to savings (cold storage)
    println!("\nTransferring 60.0 BTC from operations to savings...");
    match system.transfer("operations", "savings", 60.0) {
        Ok(_) => {
            println!("✓ Transfer successful");
            print_balances(&system);
        }
        Err(e) => println!("✗ Transfer failed: {}", e),
    }

    // Transfer to backup
    println!("\nTransferring 20.0 BTC from operations to backup...");
    match system.transfer("operations", "backup", 20.0) {
        Ok(_) => {
            println!("✓ Transfer successful");
            print_balances(&system);
        }
        Err(e) => println!("✗ Transfer failed: {}", e),
    }

    // Attempt to transfer more than available (should fail)
    println!("\nAttempting to transfer 50.0 BTC from operations (only has 20.0)...");
    match system.transfer("operations", "savings", 50.0) {
        Ok(_) => println!("✓ Transfer successful"),
        Err(e) => println!("✗ Expected failure: {}", e),
    }

    // Rebalancing: move some funds from savings back to operations
    println!("\nRebalancing: Moving 30.0 BTC from savings back to operations...");
    match system.transfer("savings", "operations", 30.0) {
        Ok(_) => {
            println!("✓ Transfer successful");
            print_balances(&system);
        }
        Err(e) => println!("✗ Transfer failed: {}", e),
    }

    // Final summary
    println!("\n=== Final Summary ===");
    println!("Total system balance: {} BTC", system.get_total_balance());
    println!("Number of wallets: {}", system.wallet_count());
    println!(
        "Total transactions: {}",
        system.get_all_transactions().len()
    );
}

fn print_balances(system: &CustodySystem) {
    println!("\nCurrent balances:");
    for (id, wallet) in system.get_all_wallets() {
        println!(
            "  {} ({:?}): {} BTC",
            id, wallet.wallet_type, wallet.balance
        );
    }
}
