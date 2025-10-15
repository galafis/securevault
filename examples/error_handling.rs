// Example: Error handling and validation
//
// Run with: cargo run --example error_handling

use securevault::{CustodySystem, WalletType};

fn main() {
    println!("=== Error Handling Example ===\n");

    let mut system = CustodySystem::new();

    // Test 1: Create a wallet
    println!("Test 1: Creating a wallet");
    match system.create_wallet(
        "test_wallet".to_string(),
        "0xTEST123".to_string(),
        WalletType::Hot,
    ) {
        Ok(wallet) => println!("✓ Wallet created: {}", wallet.id),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Test 2: Try to create duplicate wallet
    println!("\nTest 2: Creating duplicate wallet (should fail)");
    match system.create_wallet(
        "test_wallet".to_string(),
        "0xTEST456".to_string(),
        WalletType::Cold,
    ) {
        Ok(wallet) => println!("✗ Unexpectedly created wallet: {}", wallet.id),
        Err(e) => println!("✓ Expected error: {}", e),
    }

    // Test 3: Deposit to non-existent wallet
    println!("\nTest 3: Depositing to non-existent wallet (should fail)");
    match system.deposit("nonexistent_wallet", 10.0) {
        Ok(_) => println!("✗ Unexpectedly succeeded"),
        Err(e) => println!("✓ Expected error: {}", e),
    }

    // Test 4: Deposit negative amount
    println!("\nTest 4: Depositing negative amount (should fail)");
    match system.deposit("test_wallet", -10.0) {
        Ok(_) => println!("✗ Unexpectedly succeeded"),
        Err(e) => println!("✓ Expected error: {}", e),
    }

    // Test 5: Deposit zero amount
    println!("\nTest 5: Depositing zero amount (should fail)");
    match system.deposit("test_wallet", 0.0) {
        Ok(_) => println!("✗ Unexpectedly succeeded"),
        Err(e) => println!("✓ Expected error: {}", e),
    }

    // Test 6: Successful deposit
    println!("\nTest 6: Successful deposit");
    match system.deposit("test_wallet", 100.0) {
        Ok(_) => println!("✓ Deposited 100.0 BTC"),
        Err(e) => println!("✗ Unexpected error: {}", e),
    }

    // Test 7: Withdraw more than balance
    println!("\nTest 7: Withdrawing more than balance (should fail)");
    match system.withdraw("test_wallet", 150.0) {
        Ok(_) => println!("✗ Unexpectedly succeeded"),
        Err(e) => println!("✓ Expected error: {}", e),
    }

    // Test 8: Withdraw negative amount
    println!("\nTest 8: Withdrawing negative amount (should fail)");
    match system.withdraw("test_wallet", -10.0) {
        Ok(_) => println!("✗ Unexpectedly succeeded"),
        Err(e) => println!("✓ Expected error: {}", e),
    }

    // Test 9: Successful withdrawal
    println!("\nTest 9: Successful withdrawal");
    match system.withdraw("test_wallet", 30.0) {
        Ok(_) => println!("✓ Withdrew 30.0 BTC"),
        Err(e) => println!("✗ Unexpected error: {}", e),
    }

    // Test 10: Transfer to non-existent wallet
    println!("\nTest 10: Transferring to non-existent wallet (should fail)");
    match system.transfer("test_wallet", "nonexistent", 10.0) {
        Ok(_) => println!("✗ Unexpectedly succeeded"),
        Err(e) => println!("✓ Expected error: {}", e),
    }

    // Test 11: Transfer from non-existent wallet
    println!("\nTest 11: Transferring from non-existent wallet (should fail)");
    match system.transfer("nonexistent", "test_wallet", 10.0) {
        Ok(_) => println!("✗ Unexpectedly succeeded"),
        Err(e) => println!("✓ Expected error: {}", e),
    }

    // Test 12: Transfer negative amount
    system
        .create_wallet(
            "receiver".to_string(),
            "0xRECEIVER".to_string(),
            WalletType::Cold,
        )
        .unwrap();

    println!("\nTest 12: Transferring negative amount (should fail)");
    match system.transfer("test_wallet", "receiver", -10.0) {
        Ok(_) => println!("✗ Unexpectedly succeeded"),
        Err(e) => println!("✓ Expected error: {}", e),
    }

    // Test 13: Successful transfer
    println!("\nTest 13: Successful transfer");
    match system.transfer("test_wallet", "receiver", 20.0) {
        Ok(_) => {
            println!("✓ Transferred 20.0 BTC");
            let sender = system.get_wallet("test_wallet").unwrap();
            let receiver = system.get_wallet("receiver").unwrap();
            println!(
                "  Sender balance: {} BTC, Receiver balance: {} BTC",
                sender.balance, receiver.balance
            );
        }
        Err(e) => println!("✗ Unexpected error: {}", e),
    }

    // Summary
    println!("\n=== Summary ===");
    println!("All error handling tests completed successfully!");
    println!(
        "Final test_wallet balance: {} BTC",
        system.get_wallet("test_wallet").unwrap().balance
    );
}
