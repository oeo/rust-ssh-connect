# Rust SSH Connect

Rust SSH Connect is a command-line tool that attempts to establish SSH connections using various combinations of usernames, keys, and ports. It's designed to simplify the process of connecting to servers when you're unsure of the exact credentials or configuration.

## Features

- Attempts connections using multiple SSH keys found in the user's `.ssh` directory
- Configurable usernames and ports
- Verbose and quiet modes for different levels of output
- Automatic copying of successful connection strings to clipboard (macOS only)
- Configuration via TOML file and environment variables

## Installation

1. Ensure you have Rust installed on your system. If not, install it from [https://rustup.rs/](https://rustup.rs/)

2. Clone this repository:
   ```
   git clone https://github.com/yourusername/rust-ssh-connect.git
   cd rust-ssh-connect
   ```

3. Build the project:
   ```
   cargo build --release
   ```

4. The binary will be available at `target/release/rust-ssh-connect`

## Usage

Basic usage:
```
rust-ssh-connect <SERVER_IP>
```

Options:
- `-u, --username <USERNAME>`: Specify a username
- `-k, --key-file <KEY_FILE>`: Specify a key file
- `-v, --verbose`: Enable verbose output
- `-q, --quiet`: Enable quiet mode (only errors)
- `--list-keys`: List available SSH keys

Examples:
```
rust-ssh-connect 192.168.1.100
rust-ssh-connect 192.168.1.100 -u ubuntu
rust-ssh-connect 192.168.1.100 -k /path/to/key -v
```

## Configuration

The tool looks for a configuration file at `~/.config/rust-ssh-connect.toml`. If not found, it will create a default one. You can specify a different location using the `RUST_SSH_CONNECT_CONFIG` environment variable.

Example configuration:

```toml
[ssh]
certs = ["/home/user/.ssh/id_rsa", "/home/user/.ssh/id_ed25519"]
ports = [22, 2222]
users = ["ubuntu", "ec2-user", "admin"]
```

## Environment Variables

- `RUST_SSH_CONNECT_CONFIG`: Path to the configuration file
- `RUST_SSH_CONNECT_USERS`: Comma-separated list of usernames to try

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.

Copyright (c) 2024 taky@taky.com

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
