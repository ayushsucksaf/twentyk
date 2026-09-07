# 2K

A simple command-line time tracker for coding sessions, written in Rust.

Press start, code, press `Ctrl+C` when you're done — 2K keeps a running countdown, autosaves your progress every minute so nothing's lost if your machine shuts down, and lets you check back on how much time you've clocked today, this week, and this month.

## Features

- **Live session timer** — start a session and watch elapsed time tick up in real time.
- **Crash-safe autosave** — progress is written to a local database every minute, so an unexpected shutdown only costs you a minute of data, not the whole session.
- **Stats at a glance** — check total time tracked today, over the last 7 days, and over the last 30 days.
- **Graceful stop** — `Ctrl+C` ends the session cleanly and records the final elapsed time.
- **Local-first** — all data is stored in a local SQLite database file; nothing leaves your machine.

## Installation

You'll need [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

Clone the repository and install it locally:

```bash
git clone https://github.com/ayushsucksaf/twentyk
cd twentyk
cargo install --path .
```

This builds the project in release mode and installs the `2k` command, making it available from anywhere in your terminal (as long as `~/.cargo/bin` is on your `PATH`, which it usually is if you installed Rust via `rustup`).

## Usage

Launch the app:

```bash
20k
```

You'll see a menu of available commands:

```
20K
•start   •stats    •exit
>
```

- **`start`** — begins a tracking session. A live elapsed-time counter is shown; press `Ctrl+C` to stop and record the session.
- **`stats`** — shows total time tracked today, this week, and this month.
- **`exit`** — quits the app.

## How it works

- Each session's elapsed time is measured with a simple countdown loop.
- Every 60 seconds while a session is running, the elapsed time so far is saved to a local SQLite database (`time_storage.db`, created automatically on first run) — this is what protects you against losing an entire session if your laptop shuts down or crashes mid-session.
- When you stop a session, the final elapsed time is recorded and printed.
- Stats are calculated by summing recorded time from the database, filtered by date.

## Built with

- [`rusqlite`](https://crates.io/crates/rusqlite) — local SQLite storage
- [`ctrlc`](https://crates.io/crates/ctrlc) — graceful `Ctrl+C` handling
- [`chrono`](https://crates.io/crates/chrono) — date handling for grouping sessions by day/week/month
## License

MIT