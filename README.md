
# TrustLance – Decentralized Service Marketplace

A blockchain-powered service marketplace built on Solana, enabling customers to connect with service providers, manage bookings, and make secure payments through smart-contract-controlled escrow.

## Overview

The platform allows customers to discover service providersand create bookings. Providers can accept bookings and mark jobs as completed. Payments are held in an on-chain escrow account and released to the provider only after the customer approves the completed service.

## Key Features

- **Customer Registration:** Customers can register their identity on-chain using their connected Solana wallet.
- **Provider Registration:** Service providers can register their name, service category, and pricing.
- **Service Marketplace:** Customers can browse available service providers and their listed prices.
- **On-chain Bookings:** Customers can create service bookings through Solana transactions.
- **Booking Management:** Providers can accept bookings and mark services as completed.
- **Escrow-based Payments:** Customer payments are held in a program-controlled escrow account.
- **Customer Approval:** Payments are released to providers after customers approve completed jobs.
- **Booking Cancellation:** Customers can cancel bookings before provider acceptance and receive their escrowed payment back.
- **Wallet Integration:** Phantom wallet is used to sign and authorize blockchain transactions.

## Technology Stack

| Category | Technologies |
|---|---|
| Blockchain | Solana |
| Smart Contracts | Rust, Anchor Framework |
| Frontend | React, JavaScript, Vite |
| Wallet Integration | Phantom Wallet |
| Blockchain Interaction | Solana Web3.js, Anchor Client |
| Account Management | Program Derived Addresses (PDAs) |
| Testing | Anchor Tests, LiteSVM |
| Network | Solana Devnet |

## Architecture

ServiceHub Lite consists of two primary layers:

### 1. Frontend Layer

The React application provides the user interface for wallet connection, customer and provider registration, marketplace browsing, and booking management.

The frontend communicates with the Solana blockchain through the Anchor client and Solana Web3.js.

### 2. Blockchain Layer

The Solana smart contract, developed using Rust and Anchor, handles registration, booking creation, booking status transitions, and escrow payment operations.

Program Derived Addresses (PDAs) are used to manage customer, provider, booking, and escrow accounts.

## Booking and Escrow Workflow

1. A customer connects their Phantom wallet and registers on the platform.
2. A service provider registers their service and pricing.
3. The customer selects a provider and creates a booking.
4. The booking and escrow accounts are initialized on-chain, and the booking amount is transferred into escrow.
5. The provider accepts the booking.
6. After completing the service, the provider then marks the job as completed.
7. The customer approves the completed job.
8. The smart contract releases the payment to the provider.
9. Customers can also cancel a booking before it is accepted, triggering a refund from escrow.

## Smart Contract Instructions

The Anchor program implements the following instructions:

| Instruction | Purpose |
|---|---|
| `register_customer` | Registers a customer |
| `register_provider` | Registers a service provider |
| `create_booking` | Creates a booking and initializes escrow |
| `accept_booking` | Allows the provider to accept a booking |
| `complete_job` | Marks an accepted job as completed |
| `approve_job` | Approves the completed job and releases payment |
| `cancel_booking` | Cancels an eligible booking and refunds the customer |

## Project Structure

```text
servicehub-lite/
│
├── frontend/
│   ├── public/
│   └── src/
│       ├── Components/
│       ├── Pages/
│       ├── config/
│       ├── idl/
│       ├── App.jsx
│       ├── App.css
│       ├── index.css
│       └── main.jsx
│
├── programs/
│   └── project/
│       ├── src/
│       │   ├── instructions/
│       │   ├── state/
│       │   ├── constants.rs
│       │   ├── errors.rs
│       │   └── lib.rs
│       └── tests/
│
├── tests/
├── Anchor.toml
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
└── README.md
```
