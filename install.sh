#!/usr/bin/env bash
set -euo pipefail

INSTALL_DIR="${1:-$HOME/.local/bin}"

echo "Building fdf (release)..."
cargo build --release

mkdir -p "$INSTALL_DIR"
cp target/release/fdf "$INSTALL_DIR/fdf"

# macOS: re-sign the binary so Gatekeeper doesn't SIGKILL it
if [[ "$(uname -s)" == "Darwin" ]]; then
    codesign --force --sign - "$INSTALL_DIR/fdf"
fi

echo "Installed fdf to $INSTALL_DIR/fdf"
