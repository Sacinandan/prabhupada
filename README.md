# Bhaktivedanta Swami

OpenAI Telegram Bot

![version](https://img.shields.io/badge/version-0.1.1-green.svg)

<img src="https://raw.githubusercontent.com/github/explore/80688e429a7d4ef2fca1e82350fe8e3517d3494d/topics/rust/rust.png" class="d-block rounded-1 mr-3 flex-shrink-0" alt="rust" width="64" height="64">

## Links

- [Telegram](https://t.me/BhaktivedantaSwamiBot)

## Roadmap

- Supporting answers based on individual private conversation history
- Donations
- Powerful cloud hosting

## Installation

1. Install [rustup](https://rustup.rs/)
2. Run command `rustup update stable`
3. Add .env file with vars:
```
OPENAI_API_KEY=sk-...
TELOXIDE_TOKEN=...:...
RUST_LOG=info
```

## Development

- Run command `cargo run --package prabhupada --bin prabhupada`

## Build

- Run command `cargo build --release`

## License

[MIT](LICENSE)