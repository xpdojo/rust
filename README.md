# Rust

- [Rust](#rust)
  - [Install](#install)
    - [Ubuntu 22.04](#ubuntu-2204)
    - [Windows](#windows)
  - [Developing](#developing)
  - [Unit Testing](#unit-testing)

## Install

- [Using rustup (Recommended)](https://www.rust-lang.org/tools/install)

### Ubuntu 22.04

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```sh
source "$HOME/.cargo/env"
```

```sh
cargo --version
```

### Windows

- [Other ways to install rustup](https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup)

## Developing

```sh
cargo init
```

```sh
cargo run
```

## Unit Testing

```sh
cargo test
```
