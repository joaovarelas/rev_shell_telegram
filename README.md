# rev_shell_telegram
Rust program to execute commands remotely through Telegram Bot API.

# Demo

## Windows

<img src="static/demo_windows.png " width="700">


## Linux

<img src="static/demo_linux.png " width="700">


# Usage

1. Install Rust: [https://rustup.rs/](https://rustup.rs/)

2. Create a bot in Telegram: [https://core.telegram.org/bots#how-do-i-create-a-bot](https://core.telegram.org/bots#how-do-i-create-a-bot)

3. Edit the following lines in `src/main.rs` and replace with your bot `token` and your `userid`:

```rust
static TOKEN: &str = "YOUR-BOT-TOKEN";
static BOT_ID: &str = "rust-bot-dev";
static PREFIX: &str = ".";
static MASTER: u64 = YOUR-TELEGRAM-USER-ID;
```

4. Compile targeting Linux or Windows (`compile.sh`):

- `cargo rustc --release --target x86_64-unknown-linux-gnu`

- `cargo rustc --release --target x86_64-pc-windows-gnu  -- -C link-args=-mwindows`




## Dependencies

- Frankenstein (Telegram API Client in Rust, credits to @ayrat555): [https://github.com/ayrat555/frankenstein/](https://github.com/ayrat555/frankenstein/)
- tokio.rs
- sysinfo


## Disclaimer

For educational and ethical purposes. Expect bugs, crashes and ugly Rust code. 
