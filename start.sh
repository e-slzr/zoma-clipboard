#!/usr/bin/env bash
# Script para iniciar Zoma Clipboard en elementary OS / Linux
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN="$DIR/src-tauri/target/release/zoma-clipboard"
export PATH="$HOME/.cargo/bin:$PATH"
export GDK_BACKEND=x11

if [ ! -f "$BIN" ]; then
    echo "Compilando Zoma Clipboard en modo release..."
    (cd "$DIR" && npx tauri build --no-bundle)
fi

echo "Iniciando Zoma Clipboard..."
exec "$BIN" "$@"
