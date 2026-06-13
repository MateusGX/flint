#!/usr/bin/env sh
# Uninstall the flint CLI.
#
# Usage:
#   curl -fsSL https://flint.devlayer.app/uninstall.sh | sh
#
# Environment variables:
#   FLINT_INSTALL_DIR Directory the binary was installed into (default: $HOME/.local/bin)
set -eu

BIN_NAME="flint"
INSTALL_DIR="${FLINT_INSTALL_DIR:-$HOME/.local/bin}"
BIN_PATH="$INSTALL_DIR/$BIN_NAME"

say() {
  printf '%s\n' "$*" >&2
}

if [ -e "$BIN_PATH" ]; then
  rm -f "$BIN_PATH"
  say "Removed ${BIN_PATH}"
else
  say "${BIN_PATH} not found, nothing to remove"
fi

case ":$PATH:" in
  *":$INSTALL_DIR:"*)
    say ""
    say "${INSTALL_DIR} is still on your PATH. Remove it from your shell profile if you no longer need it."
    ;;
esac
