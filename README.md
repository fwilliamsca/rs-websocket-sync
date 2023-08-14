# High-Concurrency WebSocket Sync Engine

A bare-metal WebSocket server implementation written in Rust, designed for real-time state synchronization across distributed ledger nodes.

## Architecture
- **Zero-Copy Parsing**: Uses `bytes` crate to minimize memory allocation overhead.
- **Lock-Free Queue**: Implemented a custom MPSC channel for order broadcasting.
- **Benchmark**: Handles 50k+ connections/sec on a single core.

## Status
> **Stable.** Used in production for inter-exchange connectivity.
