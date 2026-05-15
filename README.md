# AnchorFlow Monorepo

A full-stack Soroban dApp using Turborepo + pnpm workspaces (TypeScript) and Cargo Workspaces (Rust).

## Structure

```
anchorflow-monorepo/
├── apps/
│   ├── web/          # Next.js 14 frontend
│   └── api/          # Node/Express backend
├── contracts/
│   └── anchorflow/   # Soroban smart contract (Rust)
├── packages/
│   └── types/        # Shared TypeScript types
├── Cargo.toml        # Rust workspace root
├── package.json      # pnpm workspace root
└── turbo.json        # Turborepo pipeline
```

## Prerequisites

- Node.js 20+ / pnpm 9+
- Rust + `soroban-cli` (`cargo install --locked soroban-cli`)

## Getting Started

```bash
# Install JS dependencies
pnpm install

# Run all apps in dev mode
pnpm dev

# Build everything
pnpm build
```

## Smart Contract

```bash
# Build the Soroban contract
cargo build --release --target wasm32-unknown-unknown -p anchorflow-contract

# Run contract tests
cargo test -p anchorflow-contract

# Deploy to Stellar testnet (requires soroban-cli)
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/anchorflow_contract.wasm \
  --network testnet
```

## Apps

| App | Port | Description |
|-----|------|-------------|
| `web` | 3000 | Next.js frontend |
| `api` | 3001 | Express REST API |
