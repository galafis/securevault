# 🔐 SecureVault - Cryptocurrency Custody System

[![Code Coverage](https://codecov.io/gh/galafis/securevault/branch/main/graph/badge.svg)](https://codecov.io/gh/galafis/securevault)


[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![Security](https://img.shields.io/badge/security-enterprise-green.svg)]()

[English](#english) | [Português](#português)

---

## English

### 🚀 Overview

**SecureVault** is an enterprise-grade cryptocurrency custody system built in Rust, designed for secure storage and management of digital assets. It features multi-signature wallets, hot/cold wallet separation, and comprehensive audit trails.

### ✨ Key Features

- **Multi-Signature Wallets**: Require multiple approvals for transactions
- **Hot/Cold Wallet Separation**: Segregate operational and storage funds
- **Secure Key Management**: Hardware Security Module (HSM) integration ready
- **Audit Trail**: Complete transaction history and compliance reporting
- **Type-Safe Operations**: Leverages Rust's safety guarantees
- **Real-Time Balance Tracking**: Monitor all wallet balances instantly

### 🛠️ Installation

```bash
git clone https://github.com/galafis/securevault.git
cd securevault
cargo build --release
```

### 🎯 Quick Start

```bash
cargo run --release
```

Output:
```
🔐 SecureVault - Cryptocurrency Custody System
==============================================

✓ Created hot wallet: hot_001 (0x1234567890abcdef)
✓ Created cold wallet: cold_001 (0xfedcba0987654321)

✓ Deposited 10.5 BTC to hot wallet
✓ Deposited 100.0 BTC to cold wallet

📊 Wallet Balances:
  hot_001 (Hot): 10.5 BTC
  cold_001 (Cold): 100 BTC

💰 Total Balance: 110.5 BTC

✓ Withdrew 5.0 BTC from hot wallet

📊 Final Total Balance: 105.5 BTC
```

### 📚 Usage Examples

```rust
use securevault::{CustodySystem, WalletType};

fn main() {
    let mut system = CustodySystem::new();

    // Create hot wallet for daily operations
    let hot_wallet = system.create_wallet(
        "hot_001".to_string(),
        "0x1234567890abcdef".to_string(),
        WalletType::Hot,
    );

    // Create cold wallet for long-term storage
    let cold_wallet = system.create_wallet(
        "cold_001".to_string(),
        "0xfedcba0987654321".to_string(),
        WalletType::Cold,
    );

    // Deposit funds
    system.deposit("hot_001", 10.5).unwrap();
    system.deposit("cold_001", 100.0).unwrap();

    // Check balance
    let total = system.get_total_balance();
    println!("Total Balance: {} BTC", total);

    // Withdraw from hot wallet
    system.withdraw("hot_001", 5.0).unwrap();
}
```

### 🏗️ Architecture

![Architecture Diagram](docs/images/architecture.png)

The system implements a layered security architecture:

1. **Client Layer**: CLI and future REST API
2. **Custody System**: Wallet management and operations
3. **Wallet Types**: Hot wallets (operational) and Cold wallets (storage)
4. **Security Layer**: Key management, multi-signature, HSM integration
5. **Audit & Compliance**: Complete transaction logging

### 🔒 Security Flow

![Security Flow](docs/images/security_flow.png)

### 🔒 Security Features

- **Separation of Concerns**: Hot wallets for operations, cold wallets for storage
- **Balance Verification**: Automatic checks before withdrawals
- **Error Handling**: Comprehensive error messages for all operations
- **Type Safety**: Rust's ownership system prevents common security bugs

### 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### 👤 Author

**Gabriel Demetrios Lafis**
- Systems Analyst & Developer
- IT Manager
- Cybersecurity Specialist
- Business Intelligence / Business Analyst
- Data Analyst & Data Scientist

---

## Português

### 🚀 Visão Geral

**SecureVault** é um sistema de custódia de criptomoedas de nível empresarial construído em Rust, projetado para armazenamento e gerenciamento seguro de ativos digitais. Possui carteiras multi-assinatura, separação de carteiras quentes/frias e trilhas de auditoria abrangentes.

### ✨ Principais Recursos

- **Carteiras Multi-Assinatura**: Requer múltiplas aprovações para transações
- **Separação Hot/Cold Wallet**: Segrega fundos operacionais e de armazenamento
- **Gerenciamento Seguro de Chaves**: Integração com Hardware Security Module (HSM) pronta
- **Trilha de Auditoria**: Histórico completo de transações e relatórios de conformidade
- **Operações Type-Safe**: Aproveita as garantias de segurança do Rust
- **Rastreamento de Saldo em Tempo Real**: Monitore todos os saldos de carteiras instantaneamente

### 🏗️ Arquitetura

![Diagrama de Arquitetura](docs/images/architecture.png)

O sistema implementa uma arquitetura de segurança em camadas:

1. **Camada de Cliente**: CLI e futura REST API
2. **Sistema de Custódia**: Gerenciamento de carteiras e operações
3. **Tipos de Carteira**: Hot wallets (operacionais) e Cold wallets (armazenamento)
4. **Camada de Segurança**: Gerenciamento de chaves, multi-assinatura, integração HSM
5. **Auditoria e Compliance**: Registro completo de transações

### 🔒 Fluxo de Segurança

![Fluxo de Segurança](docs/images/security_flow.png)

### 🛠️ Instalação

```bash
git clone https://github.com/gabriellafis/securevault.git
cd securevault
cargo build --release
```

### 🎯 Início Rápido

```bash
cargo run --release
```

### 📄 Licença

Este projeto está licenciado sob a Licença MIT - consulte o arquivo [LICENSE](LICENSE) para detalhes.

### 👤 Autor

**Gabriel Demetrios Lafis**
- Analista e Desenvolvedor de Sistemas
- Gestor de Tecnologia da Informação
- Especialista em Segurança Cibernética
- Business Intelligence / Business Analyst
- Analista e Cientista de Dados
