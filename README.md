# block-explorer

[![pipeline](https://github.com/zenoxygen/block-explorer/actions/workflows/ci.yaml/badge.svg)](https://github.com/zenoxygen/block-explorer/actions/workflows/ci.yaml)

A block explorer for Bitcoin Core.

## Requirements

Run Bitcoin Core with `txindex` and `blockfilters` enabled.
If your Bitcoin Core node is on a distant server, use SSH port forwarding: `ssh -L 8332:127.0.0.1:8332 username@host -N`.

## Installation

- Edit the `Rocket.toml` file with Bitcoin Core RPC credentials
- Build the project `cargo build`
- Launch the explorer `cargo run`
- Access the explorer at [http://localhost:8000](http://localhost:8000)

## Contributing

All contributions to this project are welcome.

## License

This project is released under the [MIT License](LICENSE).
