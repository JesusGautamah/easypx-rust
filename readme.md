# Easypx Rust
#### Debian based
`easypx` is a Rust-based command-line tool that helps manage and interact with `tor` and `privoxy` services. It provides a simple interface to start, stop, restart these services, shuffle IP, check IP, and navigate or download from the web using these services.

## Features

- Start, stop, and restart `tor` and `privoxy` services.
- Shuffle IP using `tor`.
- Check current IP.
- Navigate to a website using `firefox`.
- Download files securely and quickly using `torify wget`.
- Use `w3m` terminal browser to connect to a website via proxy.

## Usage

```bash
# Start tor & privoxy
easypx start

# Stop tor & privoxy
easypx stop

# Restart tor & privoxy
easypx restart

# Shuffle IP
easypx shuffle

# Check IP
easypx check

# Firefox connect to a website
easypx nav LINKTOWEBSITE

# W3M (terminal browser) proxy connect to a website
easypx snav LINKTOWEBSITE

# Proxy fast & secure download
easypx dw LINKTOARCHIVE
```

## Dependencies
- Rust
- Cargo
- Tor
- Privoxy
- Firefox
- W3m
- Wget

## Installation

Please ensure that you have the above dependencies installed on your system. Then, clone this repository and build the project using Cargo:

```bash
git clone https://github.com/JesusGautamah/easypx-rust.git
cd easypx-rust
cargo build --release
sudo cp target/release/easypx /usr/bin/
```

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.