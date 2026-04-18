#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT_DIR="$ROOT_DIR/macos/SqueeezoActionExtension"
PROJECT_SPEC="$PROJECT_DIR/project.yml"
PROJECT_FILE="$PROJECT_DIR/SqueeezoActionExtension.xcodeproj"
DERIVED_DATA_DIR="${DERIVED_DATA_DIR:-$ROOT_DIR/build/macos/SqueeezoActionExtension}"
CONFIGURATION="${CONFIGURATION:-Release}"
APP_BUNDLE_PATH="${1:-}"

if ! command -v xcodegen >/dev/null 2>&1; then
  echo "xcodegen is required to generate the macOS extension project." >&2
  exit 1
fi

if ! command -v xcodebuild >/dev/null 2>&1; then
  echo "xcodebuild is required to build the macOS extension." >&2
  exit 1
fi

if [[ ! -f "$PROJECT_SPEC" ]]; then
  echo "Missing XcodeGen spec at $PROJECT_SPEC" >&2
  exit 1
fi

xcodegen generate --spec "$PROJECT_SPEC"

cargo build -p compression-cli --release

xcodebuild \
  -project "$PROJECT_FILE" \
  -scheme "SqueeezoActionExtension" \
  -configuration "$CONFIGURATION" \
  -derivedDataPath "$DERIVED_DATA_DIR" \
  CODE_SIGNING_ALLOWED=NO \
  build

APPEX_PATH="$(find "$DERIVED_DATA_DIR/Build/Products/$CONFIGURATION" -maxdepth 1 -name '*.appex' -print -quit)"
if [[ -z "$APPEX_PATH" ]]; then
  echo "Unable to locate built .appex in $DERIVED_DATA_DIR/Build/Products/$CONFIGURATION" >&2
  exit 1
fi

mkdir -p "$APPEX_PATH/Contents/Resources"
cp "$ROOT_DIR/target/release/compression-cli" "$APPEX_PATH/Contents/Resources/compression-cli"
chmod +x "$APPEX_PATH/Contents/Resources/compression-cli"

if [[ -n "$APP_BUNDLE_PATH" ]]; then
  mkdir -p "$APP_BUNDLE_PATH/Contents/PlugIns"
  rm -rf "$APP_BUNDLE_PATH/Contents/PlugIns/$(basename "$APPEX_PATH")"
  cp -R "$APPEX_PATH" "$APP_BUNDLE_PATH/Contents/PlugIns/"
fi

echo "Built Action Extension at $APPEX_PATH"
if [[ -n "$APP_BUNDLE_PATH" ]]; then
  echo "Embedded Action Extension into $APP_BUNDLE_PATH"
fi
