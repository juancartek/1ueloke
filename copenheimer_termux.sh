#!/bin/bash
pkg update -y && pkg install rust git -y
mkdir -p copenheimer_oxide/src
cd copenheimer_oxide
echo '[package]
name = "copenheimer_oxide"
version = "4.5.0"
edition = "2021"
[dependencies]
tokio = { version = "1.0", features = ["full"] }
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
colored = "2.0"
dns-lookup = "2.0"
indicatif = "0.17"
futures = "0.3"' > Cargo.toml
curl -L -o src/main.rs https://raw.githubusercontent.com/juancartek/1ueloke/main/src/main.rs 2>/dev/null || echo "Error: El archivo main.rs no se pudo descargar."
ulimit -n 65000 2>/dev/null
cargo run --release
