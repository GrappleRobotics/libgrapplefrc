#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$ROOT_DIR"

#######################################
# Fix Gradle permissions
#######################################
echo ">>> Fixing Gradle permissions..."
sudo chown -R $USER:$USER "$ROOT_DIR/.gradle" || true
sudo chown -R $USER:$USER "$ROOT_DIR/build" || true

#######################################
# Docker-based builds
#######################################
build_docker() {
  local container="$1"
  local artifact="$2"
  local build_opts="$3"
  local platform="$4"

  echo ">>> Running Docker build for $artifact ($platform)"
  sudo docker run --rm -v "$ROOT_DIR:/workspace" -w /workspace "$container" bash -c "
    set -e

    # Install Rust
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    export PATH=\$HOME/.cargo/bin:\$PATH

    # Install Clang/LLVM
    apt-get update && apt-get install -y clang-15 llvm-15-dev libclang-15-dev
    ln -sf /usr/bin/llvm-config-15 /usr/bin/llvm-config
    export LIBCLANG_PATH=/usr/lib/llvm-15/lib
    export LLVM_CONFIG_PATH=/usr/bin/llvm-config

    # Cross-compilers + libc headers
    apt-get install -y gcc-arm-linux-gnueabi libc6-dev-armel-cross \
                       gcc-arm-linux-gnueabihf libc6-dev-armhf-cross \
                       gcc-aarch64-linux-gnu libc6-dev-arm64-cross \
                       build-essential

    # Rust linker environment variables
    export CC_arm_unknown_linux_gnueabi=arm-linux-gnueabi-gcc
    export CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_LINKER=arm-linux-gnueabi-gcc
    export CC_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc
    export CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUHF_LINKER=arm-linux-gnueabihf-gcc
    export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc

    # Add Rust targets
    rustup target add arm-unknown-linux-gnueabi || true
    rustup target add arm-unknown-linux-gnueabihf || true
    rustup target add aarch64-unknown-linux-gnu || true

    # Build
    ./gradlew updateRustLibs --max-workers 1 $build_opts
    cd grapplefrcdriver && python3 build.py $platform
    cd ..
    ./gradlew -PreleaseMode --max-workers 1 $build_opts publishToMavenLocal
    cd libgrapplefrc-py && pip install maturin[patchelf] && python3 build.py $platform
  "
}

#######################################
# Combine artifacts
#######################################
combine_artifacts() {
  echo ">>> Combining artifacts..."
  mkdir -p combiner/products/out
  rsync -a --delete combiner/products/m2/*/* combiner/products/out/ || true
}

#######################################
# Main
#######################################
# Build Docker Linux targets only
# build_docker "wpilib/roborio-cross-ubuntu:2025-22.04" "Athena" "-Ponlylinuxathena" "linuxathena"
# build_docker "wpilib/raspbian-cross-ubuntu:bookworm-22.04" "Arm32" "-Ponlylinuxarm32" "linuxarm32"
# build_docker "wpilib/aarch64-cross-ubuntu:bookworm-22.04" "Arm64" "-Ponlylinuxarm64" "linuxarm64"
build_docker "wpilib/ubuntu-base:22.04" "Linux" "" "linuxx86-64"

# Combine artifacts
combine_artifacts

echo ">>> Linux Docker builds completed successfully!"
