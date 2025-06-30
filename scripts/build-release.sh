#!/bin/bash

# Build script for ATS Scanner releases
set -e

echo "🚀 Building ATS Scanner Release..."

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf src-tauri/target/release/bundle/
rm -rf dist/

# Install dependencies
echo "📦 Installing dependencies..."
npm install

# Build frontend
echo "🔨 Building frontend..."
npm run build

# Build Tauri app for all platforms
echo "🔧 Building Tauri application..."
npm run tauri build

echo "✅ Build complete!"
echo "📁 Release files are in:"
echo "  • macOS: src-tauri/target/release/bundle/dmg/"
echo "  • Windows: src-tauri/target/release/bundle/msi/"
echo "  • Linux: src-tauri/target/release/bundle/appimage/"