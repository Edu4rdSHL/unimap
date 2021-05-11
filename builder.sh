#!/usr/bin/env bash
# Rusolver releaser
NAME="unimap"

LINUX_TARGET="x86_64-unknown-linux-musl"
WIN_TARGET="x86_64-pc-windows-gnu"
ARMV7_TARGET="armv7-unknown-linux-gnueabihf"
AARCH_TARGET="aarch64-unknown-linux-gnu"
OSX_TARGET="x86_64-apple-darwin"
MANPAGE_DIR="./$NAME.1"

# Linux build
echo "Building Linux artifact."
if cargo build -q --release --target="$LINUX_TARGET"; then
  echo "Linux artifact build: SUCCESS"
  cp "target/$LINUX_TARGET/release/$NAME" "target/$LINUX_TARGET/release/$NAME-linux"
else
  echo "Linux artifact build: FAILED"
fi

# Windows build
echo "Building Windows artifact."
if cross build -q --release --target="$WIN_TARGET"; then
  echo "Windows artifact build: SUCCESS"
  cp "target/$WIN_TARGET/release/$NAME.exe" "target/$WIN_TARGET/release/$NAME-windows.exe"
else
  echo "Windows artifact build: FAILED"
fi

# ARMV7 build
echo "Building ARMv7 artifact."
if cross build -q --release --target="$ARMV7_TARGET"; then
  echo "ARMv7 artifact build: SUCCESS"
  cp "target/$ARMV7_TARGET/release/$NAME" "target/$ARMV7_TARGET/release/$NAME-armv7"
else
  echo "ARMv7 artifact build: FAILED"
fi

# Aarch64 build
echo "Building Aarch64 artifact."
if cross build -q --release --target="$AARCH_TARGET"; then
  echo "Aarch64 artifact build: SUCCESS"
  cp "target/$AARCH_TARGET/release/$NAME" "target/$AARCH_TARGET/release/$NAME-aarch64"
else
  echo "Aarch64 artifact build: FAILED"
fi

# Mac OS build
echo "Building OSX artifact."
if CC=o64-clang CXX=o64-clang++ LIBZ_SYS_STATIC=1 cargo build -q --release --target="$OSX_TARGET"; then
  echo "OSX artifact build: SUCCESS"
  cp "target/$OSX_TARGET/release/$NAME" "target/$OSX_TARGET/release/$NAME-osx"
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

#echo "Uploading crate to crates.io..."
#if cargo publish --no-verify > /dev/null; then
#  echo "Crate uploaded."
#else
#  echo "An error has occurred while uploading the crate to crates.io."
#  exit
#fi

echo "All builds have passed!"
