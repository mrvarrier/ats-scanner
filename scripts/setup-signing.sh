#!/bin/bash

# ATS Scanner - Setup Signing for Distribution
# This script helps set up code signing for secure updates

set -e

echo "🔐 Setting up code signing for ATS Scanner"
echo "============================================"

# Check if we're in the right directory
if [ ! -f "src-tauri/tauri.conf.json" ]; then
    echo "❌ Error: Run this script from the project root directory"
    exit 1
fi

# Generate Tauri update keys if they don't exist
if [ ! -f "$HOME/.tauri/ats-scanner.key" ]; then
    echo "📝 Generating Tauri signing keys..."
    
    # Create .tauri directory if it doesn't exist
    mkdir -p "$HOME/.tauri"
    
    # Generate private key
    if command -v tauri >/dev/null 2>&1; then
        tauri signer generate -w "$HOME/.tauri/ats-scanner.key"
        echo "✅ Generated private key at: $HOME/.tauri/ats-scanner.key"
        echo "✅ Generated public key at: $HOME/.tauri/ats-scanner.key.pub"
    else
        echo "❌ Tauri CLI not found. Install it with:"
        echo "   npm install -g @tauri-apps/cli"
        exit 1
    fi
else
    echo "✅ Tauri signing keys already exist"
fi

# Display the public key
echo ""
echo "📋 Your public key (save this for update verification):"
echo "======================================================"
if [ -f "$HOME/.tauri/ats-scanner.key.pub" ]; then
    cat "$HOME/.tauri/ats-scanner.key.pub"
else
    echo "❌ Public key file not found"
fi

echo ""
echo "🔧 Next Steps:"
echo "=============="
echo "1. Save your PRIVATE key securely - you'll need it for signing releases"
echo "2. Add the following secrets to your GitHub repository:"
echo "   - TAURI_PRIVATE_KEY: Contents of $HOME/.tauri/ats-scanner.key"
echo "   - TAURI_KEY_PASSWORD: Password for the private key (if you set one)"
echo ""
echo "3. Update tauri.conf.json with your update server URL"
echo "4. Test building locally with: npm run build"
echo ""
echo "🚀 Ready for distribution!"

# Make the script executable
chmod +x "$0"