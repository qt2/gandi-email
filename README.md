# gandi-email

[![Crates.io](https://img.shields.io/crates/v/gandi-email)](https://crates.io/crates/gandi-email)
[![API reference](https://docs.rs/gandi-email/badge.svg)](https://docs.rs/gandi-email/)

CLI tool for Gandi Email API

## Features
- [x] Manage aliases
- [x] List domains/mailboxes
- [x] Save config file

## Installation
```sh
cargo install gandi-email
```

## Usage
```
gandi-email 1.0.1
CLI tool for Gandi Email API

USAGE:
    gandi-email <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    alias        Manage aliases
    config       Set up your config
    domains      List domains
    help         Print this message or the help of the given subcommand(s)
    mailboxes    List mailboxes
```

## License
Dual-licensed under either of the following, at your option:

* MIT License (http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)