# Anime CLI 

A lightweight, fast, and simple command-line interface tool to track your anime episodes directly from your terminal.

## Example
<img width="933" height="771" alt="image" src="https://github.com/user-attachments/assets/c9c6ad3f-08d2-4f60-b2f0-ea594940cf0e" />

## Features
- **Offline by default:** Zero internet access required. Everything runs locally.
- **Centralized Storage:** All your data is safely stored in a single SQLite `.db` file inside your `~/.anime_cli` directory.
- **Blazing Fast & Memory-Safe:** Powered by Rust 🦀.
- **Familiar Workflow:** Intuitive and easy to use if you are used to CLI tools.

> **Note:** This tool is currently designed for **Linux** users.

---

## Manual installation

To install and run this project, you will need [Rust and Cargo](https://rustup.rs/) installed on your machine.

**1. Clone the repository:**
```bash
git clone https://github.com/Fanty1107/anime-cli.git
```
**2. Build the release version**
```bash
cargo build --release
```
**3. Move the binary to your local bin path**
```bash
mkdir -p ~/.local/bin
cp target/release/anime-cli ~/.local/bin/anime-cli
```
(Make sure ~/.local/bin is added to your shell's $PATH)


***AI only used for help text***
