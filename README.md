Перед запуском проекта проверить и установить:

rustup toolchain install stable
rustup target add wasm32-unknown-unknown

curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
или
cargo install cargo-binstall

cargo binstall dioxus-cli

Установить:
https://github.com/llvm/llvm-project/releases/tag/llvmorg-21.1.0

Проект запускается командой
cargo run