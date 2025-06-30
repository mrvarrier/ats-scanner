#!/bin/bash

# ATS Scanner - Build for All Platforms
# This script builds the application for macOS, Windows, and Linux

set -e

echo "🚀 Building ATS Scanner for all platforms"
echo "=========================================="

# Check if we're in the right directory
if [ ! -f "src-tauri/tauri.conf.json" ]; then
    echo "❌ Error: Run this script from the project root directory"
    exit 1
fi

# Function to print colored output
print_status() {
    echo -e "\033[1;34m$1\033[0m"
}

print_success() {
    echo -e "\033[1;32m$1\033[0m"
}

print_error() {
    echo -e "\033[1;31m$1\033[0m"
}

# Check prerequisites
print_status "📋 Checking prerequisites..."

# Check Node.js
if ! command -v node >/dev/null 2>&1; then
    print_error "❌ Node.js not found. Please install Node.js"
    exit 1
fi

# Check npm
if ! command -v npm >/dev/null 2>&1; then
    print_error "❌ npm not found. Please install npm"
    exit 1
fi

# Check Rust
if ! command -v rustc >/dev/null 2>&1; then
    print_error "❌ Rust not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check Tauri CLI
if ! command -v tauri >/dev/null 2>&1; then
    print_status "📦 Installing Tauri CLI..."
    npm install -g @tauri-apps/cli
fi

print_success "✅ All prerequisites met"

# Install dependencies
print_status "📦 Installing dependencies..."
npm ci

# Add Rust targets for cross-compilation
print_status "🎯 Adding Rust targets..."

# Add targets based on the current platform
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS - can build universal binaries
    rustup target add aarch64-apple-darwin x86_64-apple-darwin
    print_success "✅ Added macOS targets"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    rustup target add x86_64-unknown-linux-gnu
    print_success "✅ Added Linux targets"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
    # Windows
    rustup target add x86_64-pc-windows-msvc
    print_success "✅ Added Windows targets"
fi

# Build for current platform
print_status "🔨 Building for current platform..."
npm run build

# Get build artifacts location
BUILD_DIR="src-tauri/target/release/bundle"

print_success "🎉 Build completed!"
echo ""
echo "📁 Build artifacts location: $BUILD_DIR"
echo ""

# Display what was built
if [ -d "$BUILD_DIR" ]; then
    print_status "📦 Built packages:"
    
    # macOS
    if [ -d "$BUILD_DIR/dmg" ]; then
        echo "  🍎 macOS:"
        ls -la "$BUILD_DIR/dmg/"*.dmg 2>/dev/null || echo "    No DMG files found"
    fi
    
    # Windows
    if [ -d "$BUILD_DIR/msi" ]; then
        echo "  🪟 Windows:"
        ls -la "$BUILD_DIR/msi/"*.msi 2>/dev/null || echo "    No MSI files found"
    fi
    
    # Linux
    if [ -d "$BUILD_DIR/appimage" ]; then
        echo "  🐧 Linux:"
        ls -la "$BUILD_DIR/appimage/"*.AppImage 2>/dev/null || echo "    No AppImage files found"
    fi
    
    if [ -d "$BUILD_DIR/deb" ]; then
        echo "  📦 Debian:"
        ls -la "$BUILD_DIR/deb/"*.deb 2>/dev/null || echo "    No DEB files found"
    fi
else
    print_error "❌ Build directory not found"
fi

echo ""
print_status "🔗 Next steps:"
echo "1. Test the built application"
echo "2. Create a GitHub release with: git tag v1.0.0 && git push origin v1.0.0"
echo "3. The GitHub Action will automatically build and publish releases"
echo ""
print_success "✨ Ready for distribution!"