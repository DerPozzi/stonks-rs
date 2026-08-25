# stonks-rs

`stonks-rs` is a terminal-based investment dashboard written in Rust. It tracks your stock transactions in a local SQLite database and enriches your portfolio with live market data.

## Features

- Terminal UI built with `ratatui`
- Portfolio value, per-ticker metrics, and gains/losses
- Transaction tracking with local persistence (`SQLite`)
- Automatic market-data updates in the background
- Configurable default currency (`EUR`/`USD`) and theme

## Quick start

### Prerequisites

- Rust (stable toolchain)
- Network access (required for live Yahoo Finance data)

### Build and run

From the repository root:

```bash
cargo run -p stonks-tui
```

On first start, the app creates local files/directories automatically.

## Basic usage

- `h` / `←`: previous page
- `l` / `→`: next page
- `a`: add a transaction (from the Transactions page)
- `s`: save a new transaction (in Add Transaction page)
- `?`: show hotkeys
- `Ctrl+C`: quit

## Local data and config

- Database: `~/.stonks-rs/stonks-rs.db`
- Log file: `~/.stonks-rs/stonks.log`
- Config file: `~/.config/stonks-rs/config.toml`

## Development

```bash
cargo fmt --all --check
cargo clippy --workspace --all-features --all-targets -- -D warnings
cargo build --workspace --all-features
cargo test --workspace --all-features
```

## Project structure

- `backend/`: portfolio logic, database access, market-data integration
- `tui/`: terminal application and UI pages

## Notes

- Currency conversion currently supports `EUR` and `USD`.
- Some pages are still work in progress.
