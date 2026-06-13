#!/usr/bin/env sh
# Install the flint CLI from GitHub Releases.
#
# Usage:
#   curl -fsSL https://flint.devlayer.app/install.sh | sh
#
# Environment variables:
#   FLINT_VERSION     Release tag to install (default: latest)
#   FLINT_INSTALL_DIR Directory to install the binary into (default: $HOME/.local/bin)
set -eu

REPO="MateusGX/flint"
BIN_NAME="flint"

VERSION="${FLINT_VERSION:-latest}"
INSTALL_DIR="${FLINT_INSTALL_DIR:-$HOME/.local/bin}"

say() {
  printf '%s\n' "$*" >&2
}

err() {
  say "error: $*"
  exit 1
}

detect_target() {
  os=$(uname -s)
  arch=$(uname -m)

  case "$os" in
    Linux) os_part="unknown-linux-gnu" ;;
    Darwin) os_part="apple-darwin" ;;
    *) err "unsupported OS: $os" ;;
  esac

  case "$arch" in
    x86_64 | amd64) arch_part="x86_64" ;;
    arm64 | aarch64) arch_part="aarch64" ;;
    *) err "unsupported architecture: $arch" ;;
  esac

  printf '%s-%s\n' "$arch_part" "$os_part"
}

main() {
  command -v curl >/dev/null 2>&1 || err "curl is required"
  command -v tar >/dev/null 2>&1 || err "tar is required"

  target=$(detect_target)
  archive="${BIN_NAME}-${target}.tar.gz"

  if [ "$VERSION" = "latest" ]; then
    url="https://github.com/${REPO}/releases/latest/download/${archive}"
  else
    url="https://github.com/${REPO}/releases/download/${VERSION}/${archive}"
  fi

  tmp_dir=$(mktemp -d)
  trap 'rm -rf "$tmp_dir"' EXIT

  say "Downloading ${url}"
  curl --fail --location --silent --show-error "$url" -o "$tmp_dir/$archive" \
    || err "download failed (no release published for ${target}?)"

  tar -xzf "$tmp_dir/$archive" -C "$tmp_dir"

  mkdir -p "$INSTALL_DIR"
  mv "$tmp_dir/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
  chmod +x "$INSTALL_DIR/$BIN_NAME"

  say "Installed ${BIN_NAME} to ${INSTALL_DIR}/${BIN_NAME}"

  case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
      say ""
      say "${INSTALL_DIR} is not on your PATH. Add it with:"
      say "  export PATH=\"${INSTALL_DIR}:\$PATH\""
      ;;
  esac

  "$INSTALL_DIR/$BIN_NAME" version || true
}

main "$@"
