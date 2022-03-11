#!/usr/bin/env bash
# Rusolver releaser
NAME="unimap"

LINUX_TARGET="x86_64-unknown-linux-musl"
LINUX_X86_TARGET="i686-unknown-linux-musl"
WIN_TARGET="x86_64-pc-windows-gnu"
ARMV7_TARGET="armv7-unknown-linux-gnueabihf"
AARCH_TARGET="aarch64-unknown-linux-gnu"
OSX_TARGET="x86_64-apple-darwin"
MANPAGE_DIR="./$NAME.1"

if ! systemctl is-active docker >/dev/null 2>&1; then
  echo "Docker is not running. Starting docker."
  if ! sudo systemctl start docker; then
    echo "Failed to start docker."
    exit 1
  fi
fi

# Linux build
echo "Building Linux artifact."
if cargo build -q --release --target="$LINUX_TARGET"; then
  echo "Linux artifact build: SUCCESS"
  cp "target/$LINUX_TARGET/release/$NAME" "target/$LINUX_TARGET/release/$NAME-linux"
  strip "target/$LINUX_TARGET/release/$NAME-linux"
  sha512sum "target/$LINUX_TARGET/release/$NAME-linux" >"target/$LINUX_TARGET/release/$NAME-linux.sha512"
else
  echo "Linux artifact build: FAILED"
fi

# Linux x86 build
echo "Building Linux x86 artifact."
if cross build -q --release --target="$LINUX_X86_TARGET"; then
  echo "Linux x86 artifact build: SUCCESS"
  cp "target/$LINUX_X86_TARGET/release/$NAME" "target/$LINUX_X86_TARGET/release/$NAME-linux-i386"
  strip "target/$LINUX_X86_TARGET/release/$NAME-linux-i386"
  sha512sum "target/$LINUX_X86_TARGET/release/$NAME-linux-i386" >"target/$LINUX_X86_TARGET/release/$NAME-linux-i386.sha512"
else
  echo "Linux x86 artifact build: FAILED"
fi

# Windows build
echo "Building Windows artifact."
if cross build -q --release --target="$WIN_TARGET"; then
  echo "Windows artifact build: SUCCESS"
  cp "target/$WIN_TARGET/release/$NAME.exe" "target/$WIN_TARGET/release/$NAME-windows.exe"
  strip "target/$WIN_TARGET/release/$NAME-windows.exe"
  sha512sum "target/$WIN_TARGET/release/$NAME-windows.exe" >"target/$WIN_TARGET/release/$NAME-windows.exe.sha512"
else
  echo "Windows artifact build: FAILED"
fi

# ARMV7 build
echo "Building ARMv7 artifact."
if cross build -q --release --target="$ARMV7_TARGET"; then
  echo "ARMv7 artifact build: SUCCESS"
  cp "target/$ARMV7_TARGET/release/$NAME" "target/$ARMV7_TARGET/release/$NAME-armv7"
  strip "target/$ARMV7_TARGET/release/$NAME-armv7"
  sha512sum "target/$ARMV7_TARGET/release/$NAME-armv7" >"target/$ARMV7_TARGET/release/$NAME-armv7.sha512"
else
  echo "ARMv7 artifact build: FAILED"
fi

# Aarch64 build
echo "Building Aarch64 artifact."
if cross build -q --release --target="$AARCH_TARGET"; then
  echo "Aarch64 artifact build: SUCCESS"
  cp "target/$AARCH_TARGET/release/$NAME" "target/$AARCH_TARGET/release/$NAME-aarch64"
  strip "target/$AARCH_TARGET/release/$NAME-aarch64"
else
  echo "Aarch64 artifact build: FAILED"
fi

# Mac OS build
echo "Building OSX artifact."
if CC=o64-clang CXX=o64-clang++ LIBZ_SYS_STATIC=1 cargo build -q --release --target="$OSX_TARGET"; then
  echo "OSX artifact build: SUCCESS"
  cp "target/$OSX_TARGET/release/$NAME" "target/$OSX_TARGET/release/$NAME-osx"
  strip "target/$OSX_TARGET/release/$NAME-osx"
  sha512sum "target/$OSX_TARGET/release/$NAME-osx" >"target/$OSX_TARGET/release/$NAME-osx.sha512"
else
  echo "OSX artifact build: FAILED"
fi

echo "Creating manpage..."
if command -v help2man >/dev/null; then
  if help2man -o "$MANPAGE_DIR" "target/$LINUX_TARGET/release/$NAME"; then
    echo "Manpage created sucessfully and saved in $MANPAGE_DIR"
  else
    echo "Error creating manpage."
  fi
else
  echo "Please install the help2man package."
fi

if command -v git >/dev/null; then
  git add .
  git commit -m "Bump version."
  git push
fi

# Stop docker
echo "Stopping docker."
if ! sudo systemctl stop docker; then
  echo "Failed to stop docker."
  exit 1
fi
