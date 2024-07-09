# Rust SSH Connect

This tool attempts to connect to a specified SSH server using various combinations of certificates, ports, and usernames.

## Features

- Attempts connections using multiple certificates, ports, and usernames
- Configurable via TOML file
- Copies successful connection string to clipboard (macOS only)

## Installation

1. Ensure you have Rust installed on your system. If not, install it from [https://rustup.rs/](https://rustup.rs/)

2. Clone this repository:
```
git clone https://github.com/oeo/rust-ssh-connect.git
cd rust-ssh-connect
```

3. Build the project:
```
cargo build --release
````

## Configuration

Create a `connect_config.toml` file in `~/.config/` directory:

```toml
[ssh]
certs = [
 "~/keys/key.prv",
 "~/keys/shared.pem"
]

ports = [22, 2222]

users = [
 "ec2-user",
 "ubuntu"
]
```

If the configuration file is not found in ~/.config/, the tool will look for connect_config.toml in the current directory.

## Usage

Run the tool with the target IP address as an argument:

```
./target/release/rust-ssh-connect 192.168.1.100
```

The tool will attempt to connect using each combination of certificate, port, and username specified in the configuration file. If a successful connection is made, the connection string will be copied to the clipboard (on macOS).

