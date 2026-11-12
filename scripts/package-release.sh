#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ROOT_DIR"

VERSION="${1:-}"
SKIP_BUILD="${SKIP_BUILD:-0}"

if [[ -z "$VERSION" ]]; then
  VERSION="$(node -p "require('./src-tauri/tauri.conf.json').version")"
fi

CLEAN_VERSION="${VERSION#v}"
TAG_VERSION="v${CLEAN_VERSION}"
OS_NAME="$(uname -s)"
ARCH_NAME="$(uname -m)"

case "$OS_NAME" in
  Darwin)
    TARGET_TRIPLE="${TAURI_TARGET:-universal-apple-darwin}"
    PLATFORM_SLUG="macos-universal"
    if [[ "$TARGET_TRIPLE" == "aarch64-apple-darwin" ]]; then
      PLATFORM_SLUG="macos-arm64"
    elif [[ "$TARGET_TRIPLE" == "x86_64-apple-darwin" ]]; then
      PLATFORM_SLUG="macos-x64"
    fi
    ;;
  Linux)
    if [[ "$ARCH_NAME" == "aarch64" || "$ARCH_NAME" == "arm64" ]]; then
      TARGET_TRIPLE="${TAURI_TARGET:-aarch64-unknown-linux-gnu}"
      PLATFORM_SLUG="linux-arm64"
    else
      TARGET_TRIPLE="${TAURI_TARGET:-x86_64-unknown-linux-gnu}"
      PLATFORM_SLUG="linux-x64"
    fi
    ;;
  *)
    echo "Unsupported Unix OS: $OS_NAME (use scripts/package-release.ps1 on Windows)" >&2
    exit 1
    ;;
esac

RELEASE_DIR="$ROOT_DIR/release/$TAG_VERSION"
BUNDLE_NAME="GitSwitch-$TAG_VERSION-$PLATFORM_SLUG"
BUNDLE_DIR="$RELEASE_DIR/$BUNDLE_NAME"
ARCHIVE_PATH="$RELEASE_DIR/$BUNDLE_NAME.tar.gz"
MCP_OUT="$RELEASE_DIR/gitswitch-mcp-$TAG_VERSION-$PLATFORM_SLUG"

mkdir -p "$BUNDLE_DIR"

if [[ "$SKIP_BUILD" != "1" ]]; then
  echo "Building GitSwitch desktop bundle ($PLATFORM_SLUG / $TARGET_TRIPLE)..."
  if [[ "$OS_NAME" == "Darwin" && "$TARGET_TRIPLE" == "universal-apple-darwin" ]]; then
    rustup target add aarch64-apple-darwin x86_64-apple-darwin >/dev/null 2>&1 || true
    ./frontend/node_modules/.bin/tauri build --target universal-apple-darwin
    cargo build --release --target aarch64-apple-darwin --bin gitswitch-mcp --manifest-path src-tauri/Cargo.toml
    cargo build --release --target x86_64-apple-darwin --bin gitswitch-mcp --manifest-path src-tauri/Cargo.toml
    lipo -create \
      "src-tauri/target/aarch64-apple-darwin/release/gitswitch-mcp" \
      "src-tauri/target/x86_64-apple-darwin/release/gitswitch-mcp" \
      -output "$MCP_OUT"
  else
    ./frontend/node_modules/.bin/tauri build --target "$TARGET_TRIPLE"
    cargo build --release --target "$TARGET_TRIPLE" --bin gitswitch-mcp --manifest-path src-tauri/Cargo.toml
    cp "src-tauri/target/$TARGET_TRIPLE/release/gitswitch-mcp" "$MCP_OUT"
  fi
fi

chmod +x "$MCP_OUT"
cp "$MCP_OUT" "$BUNDLE_DIR/gitswitch-mcp"
cp mcp_config.json README.md CHANGELOG.md LICENSE "$BUNDLE_DIR/"

TARGET_RELEASE_DIR="$ROOT_DIR/src-tauri/target/$TARGET_TRIPLE/release"
if [[ "$OS_NAME" == "Darwin" ]]; then
  APP_PATH="$TARGET_RELEASE_DIR/bundle/macos/GitSwitch.app"
  if [[ -d "$APP_PATH" ]]; then
    cp -R "$APP_PATH" "$BUNDLE_DIR/GitSwitch.app"
  fi
  for dmg in "$TARGET_RELEASE_DIR"/bundle/dmg/*.dmg; do
    [[ -f "$dmg" ]] && cp "$dmg" "$RELEASE_DIR/$BUNDLE_NAME.dmg"
  done
else
  cp "$TARGET_RELEASE_DIR/gitswitch" "$BUNDLE_DIR/gitswitch"
  chmod +x "$BUNDLE_DIR/gitswitch"
  cp "$TARGET_RELEASE_DIR/gitswitch" "$RELEASE_DIR/gitswitch-$TAG_VERSION-$PLATFORM_SLUG"
  chmod +x "$RELEASE_DIR/gitswitch-$TAG_VERSION-$PLATFORM_SLUG"
  for appimg in "$TARGET_RELEASE_DIR"/bundle/appimage/*.AppImage; do
    [[ -f "$appimg" ]] && cp "$appimg" "$RELEASE_DIR/$BUNDLE_NAME.AppImage"
  done
  for deb in "$TARGET_RELEASE_DIR"/bundle/deb/*.deb; do
    [[ -f "$deb" ]] && cp "$deb" "$RELEASE_DIR/$BUNDLE_NAME.deb"
  done
  for rpm in "$TARGET_RELEASE_DIR"/bundle/rpm/*.rpm; do
    [[ -f "$rpm" ]] && cp "$rpm" "$RELEASE_DIR/$BUNDLE_NAME.rpm"
  done
fi

tar -czf "$ARCHIVE_PATH" -C "$RELEASE_DIR" "$BUNDLE_NAME"

CHECKSUM_FILE="$RELEASE_DIR/SHA256SUMS-$PLATFORM_SLUG.txt"
if command -v sha256sum >/dev/null 2>&1; then
  (cd "$RELEASE_DIR" && sha256sum gitswitch*-"$PLATFORM_SLUG"* GitSwitch-"$TAG_VERSION"-"$PLATFORM_SLUG".* 2>/dev/null > "$CHECKSUM_FILE")
elif command -v shasum >/dev/null 2>&1; then
  (cd "$RELEASE_DIR" && shasum -a 256 gitswitch*-"$PLATFORM_SLUG"* GitSwitch-"$TAG_VERSION"-"$PLATFORM_SLUG".* 2>/dev/null > "$CHECKSUM_FILE")
fi

echo "Release artifacts created in $RELEASE_DIR ($PLATFORM_SLUG)"
ls -la "$RELEASE_DIR"

