# stonks-rs

A simple Rust workspace for tracking investment transactions and dividends with a terminal UI.

## Workspace

- `backend/` — core portfolio logic, calculations, and SQLite persistence
- `tui/` — terminal dashboard built with Ratatui

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable toolchain)

## Getting started

Build the full workspace:

```bash
cargo build
```

Run the TUI app:

```bash
cargo run -p stonks-tui
```

Run tests:

```bash
cargo test
```

## Data and logs

The app stores data and logs in your home directory:

- Database: `~/.stonks-rs/stonks-rs.db`
- Logs: `~/.stonks-rs/stonks.log`
