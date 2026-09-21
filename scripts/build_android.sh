#!/usr/bin/env bash
set -euo pipefail

# ==============================================================================
# Fortune Note - Android Native Library & UniFFI Binding Build Script
# ==============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
CRATE_DIR="${ROOT_DIR}/crates/fortune-backend"

DEFAULT_JNILIBS_DIR="${ROOT_DIR}/android/app/src/main/jniLibs"
DEFAULT_KOTLIN_DIR="${ROOT_DIR}/android/app/src/main/java"

JNILIBS_OUT="${1:-${DEFAULT_JNILIBS_DIR}}"
KOTLIN_OUT="${2:-${DEFAULT_KOTLIN_DIR}}"

echo "================================================================="
echo " Building Fortune Backend for Android (UniFFI & Native Libraries)"
echo "================================================================="
echo "Root Directory       : ${ROOT_DIR}"
echo "Native Output Target : ${JNILIBS_OUT}"
echo "Kotlin Output Target : ${KOTLIN_OUT}"
echo "-----------------------------------------------------------------"

# 1. Detect Android NDK
if [ -z "${ANDROID_NDK_HOME:-}" ]; then
    if [ -n "${ANDROID_HOME:-}" ] && [ -d "${ANDROID_HOME}/ndk" ]; then
        LATEST_NDK=$(ls -1d "${ANDROID_HOME}/ndk"/* 2>/dev/null | sort -V | tail -n1 || true)
        if [ -n "${LATEST_NDK}" ] && [ -d "${LATEST_NDK}" ]; then
            export ANDROID_NDK_HOME="${LATEST_NDK}"
        fi
    elif [ -d "${HOME}/Android/Sdk/ndk" ]; then
        LATEST_NDK=$(ls -1d "${HOME}/Android/Sdk/ndk"/* 2>/dev/null | sort -V | tail -n1 || true)
        if [ -n "${LATEST_NDK}" ] && [ -d "${LATEST_NDK}" ]; then
            export ANDROID_NDK_HOME="${LATEST_NDK}"
        fi
    fi
fi

if [ -z "${ANDROID_NDK_HOME:-}" ] || [ ! -d "${ANDROID_NDK_HOME}" ]; then
    echo "ERROR: Android NDK not found."
    echo "Please set ANDROID_NDK_HOME or install NDK via Android Studio / sdkmanager."
    echo "Example: sdkmanager \"ndk;27.2.12479018\""
    exit 1
fi
echo "Using Android NDK: ${ANDROID_NDK_HOME}"

# 2. Check cargo-ndk
if ! command -v cargo-ndk &> /dev/null; then
    echo "cargo-ndk is not installed. Installing cargo-ndk..."
    cargo install cargo-ndk
fi

# 3. Ensure Rust targets are installed
TARGETS=(
    "aarch64-linux-android"
    "armv7-linux-androideabi"
    "x86_64-linux-android"
    "i686-linux-android"
)

echo "Verifying rustup targets..."
INSTALLED_TARGETS=$(rustup target list --installed)
for target in "${TARGETS[@]}"; do
    if ! echo "${INSTALLED_TARGETS}" | grep -q "^${target}$"; then
        echo "Adding target: ${target}"
        rustup target add "${target}"
    fi
done

# 4. Build host release library for UniFFI bindings generation
echo "Building host release library for UniFFI bindgen..."
cargo build --manifest-path "${CRATE_DIR}/Cargo.toml" --release

HOST_SO="${ROOT_DIR}/target/release/libfortune_backend.so"
if [ ! -f "${HOST_SO}" ]; then
    echo "ERROR: Host release library not found at ${HOST_SO}"
    exit 1
fi

# 5. Generate Kotlin bindings
echo "Generating Kotlin bindings via uniffi-bindgen..."
mkdir -p "${KOTLIN_OUT}"
cargo run --manifest-path "${CRATE_DIR}/Cargo.toml" \
    --features cli \
    --bin uniffi-bindgen -- \
    generate "${HOST_SO}" \
    --language kotlin \
    --no-format \
    --out-dir "${KOTLIN_OUT}"

echo "Generated Kotlin bindings at ${KOTLIN_OUT}"

# 6. Cross-compile for Android architectures using cargo-ndk
echo "Compiling Android native libraries (.so) across target architectures..."
mkdir -p "${JNILIBS_OUT}"

cargo ndk \
    --manifest-path "${CRATE_DIR}/Cargo.toml" \
    -t arm64-v8a \
    -t armeabi-v7a \
    -t x86_64 \
    -t x86 \
    -o "${JNILIBS_OUT}" \
    build --release

echo "================================================================="
echo " SUCCESS: Android native libraries and Kotlin bindings built."
echo " JNI Libraries : ${JNILIBS_OUT}"
echo " Kotlin Source : ${KOTLIN_OUT}"
echo "================================================================="
