//! Integration tests exercising the public API end to end.

use securevault::{CustodySystem, TransactionType, WalletType};

fn system_with_hot_and_cold() -> CustodySystem {
    let mut system = CustodySystem::new();
    system
        .create_wallet("hot".to_string(), "0xhot".to_string(), WalletType::Hot)
        .unwrap();
    system
        .create_wallet("cold".to_string(), "0xcold".to_string(), WalletType::Cold)
        .unwrap();
    system
}

#[test]
fn transfer_moves_funds_and_records_both_sides() {
    let mut system = system_with_hot_and_cold();
    system.deposit("hot", 10.0).unwrap();

    system.transfer("hot", "cold", 4.0).unwrap();

    assert_eq!(system.get_wallet("hot").unwrap().balance, 6.0);
    assert_eq!(system.get_wallet("cold").unwrap().balance, 4.0);
    assert_eq!(system.get_total_balance(), 10.0);

    let hot_tx = system.get_wallet_transactions("hot");
    assert_eq!(hot_tx.len(), 2);
    assert_eq!(hot_tx[1].transaction_type, TransactionType::Withdrawal);
    let cold_tx = system.get_wallet_transactions("cold");
    assert_eq!(cold_tx.len(), 1);
    assert_eq!(cold_tx[0].transaction_type, TransactionType::Deposit);
}

#[test]
fn failed_transfer_leaves_balances_and_audit_trail_untouched() {
    let mut system = system_with_hot_and_cold();
    system.deposit("hot", 1.0).unwrap();
    let before = system.get_all_transactions().len();

    assert!(system.transfer("hot", "cold", 5.0).is_err());
    assert!(system.transfer("hot", "missing", 0.5).is_err());

    assert_eq!(system.get_wallet("hot").unwrap().balance, 1.0);
    assert_eq!(system.get_wallet("cold").unwrap().balance, 0.0);
    assert_eq!(system.get_all_transactions().len(), before);
}

#[test]
fn duplicate_wallet_id_is_rejected() {
    let mut system = system_with_hot_and_cold();
    assert!(system
        .create_wallet("hot".to_string(), "0xother".to_string(), WalletType::Cold)
        .is_err());
    assert_eq!(system.wallet_count(), 2);
}
