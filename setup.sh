#!/bin/bash

# Personal ATS Scanner Setup Script
# This script sets up the application for first-time use

set -e

echo "🚀 Personal ATS Scanner Setup"
echo "=============================="
echo

# Check if we're in the correct directory
if [ ! -f "package.json" ]; then
    echo "❌ Error: package.json not found. Please run this script from the project root directory."
    exit 1
fi

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check Node.js version
check_node_version() {
    local required_major=18
    local node_version=$(node -v | cut -d 'v' -f 2 | cut -d '.' -f 1)
    
    if [ "$node_version" -lt "$required_major" ]; then
        echo "❌ Node.js version $required_major or higher is required. Found: $(node -v)"
        echo "   Please update Node.js: https://nodejs.org/"
        return 1
    fi
    return 0
}

# Step 1: Check prerequisites
echo "🔍 Checking prerequisites..."

if ! command_exists node; then
    echo "❌ Node.js is not installed. Please install Node.js 18+ from https://nodejs.org/"
    exit 1
else
    if check_node_version; then
        echo "✅ Node.js $(node -v) is installed"
    else
        exit 1
    fi
fi

if ! command_exists npm; then
    echo "❌ npm is not installed. Please install npm."
    exit 1
else
    echo "✅ npm $(npm -v) is installed"
fi

# Step 2: Install dependencies
echo
echo "📦 Installing dependencies..."
echo "This may take a few minutes..."
echo

if npm install; then
    echo "✅ Dependencies installed successfully"
else
    echo "❌ Failed to install dependencies"
    exit 1
fi

# Step 3: Create uploads directory
echo
echo "📁 Creating uploads directory..."
mkdir -p uploads
echo "✅ Uploads directory created"

# Step 4: Initialize database
echo
echo "💾 Initializing database..."
if node -e "
const db = require('./server/utils/database');
db.initialize()
  .then(() => {
    console.log('✅ Database initialized successfully');
    process.exit(0);
  })
  .catch((err) => {
    console.error('❌ Database initialization failed:', err.message);
    process.exit(1);
  });
"; then
    echo "✅ Database ready"
else
    echo "❌ Database initialization failed"
    exit 1
fi

# Step 5: Check Ollama installation
echo
echo "🤖 Checking Ollama installation..."

if command_exists ollama; then
    echo "✅ Ollama is installed"
    
    # Check if Ollama service is running
    if curl -s http://localhost:11434/api/tags >/dev/null 2>&1; then
        echo "✅ Ollama service is running"
        
        # Check for required models
        echo "🔍 Checking for required AI models..."
        
        models_output=$(curl -s http://localhost:11434/api/tags)
        
        if echo "$models_output" | grep -q "mistral"; then
            echo "✅ mistral model is available"
        else
            echo "⚠️  mistral model not found. Installing..."
            ollama pull mistral:latest
            echo "✅ mistral model installed"
        fi
        
        if echo "$models_output" | grep -q "qwen2.5"; then
            echo "✅ qwen2.5 model is available"
        else
            echo "⚠️  qwen2.5 model not found. Installing..."
            ollama pull qwen2.5:14b
            echo "✅ qwen2.5 model installed"
        fi
        
    else
        echo "⚠️  Ollama service is not running. Please start it with: ollama serve"
    fi
else
    echo "⚠️  Ollama is not installed."
    echo "   📋 To install Ollama:"
    echo "   1. Visit: https://ollama.ai"
    echo "   2. Download and install Ollama for your system"
    echo "   3. Run: ollama pull mistral:latest"
    echo "   4. Run: ollama pull qwen2.5:14b"
    echo "   5. Run this setup script again"
fi

# Step 6: Create environment file
echo
echo "⚙️  Creating environment configuration..."
cat > .env << EOF
# Personal ATS Scanner Environment Configuration
PORT=3001
NODE_ENV=development

# Database
DATABASE_PATH=./database/ats-scanner.db

# Ollama Configuration
OLLAMA_BASE_URL=http://localhost:11434

# File Upload Configuration
MAX_FILE_SIZE=10485760
UPLOAD_DIR=./uploads
EOF
echo "✅ Environment file created"

# Step 7: Setup complete
echo
echo "🎉 Setup Complete!"
echo "=================="
echo
echo "✅ All dependencies installed"
echo "✅ Database initialized"
echo "✅ Upload directory created"
echo "✅ Environment configured"

if command_exists ollama && curl -s http://localhost:11434/api/tags >/dev/null 2>&1; then
    echo "✅ Ollama is ready"
else
    echo "⚠️  Ollama needs to be set up (see instructions above)"
fi

echo
echo "🚀 Next Steps:"
echo "1. Run: npm run dev"
echo "2. Open: http://localhost:3000"
echo "3. Upload a resume and start analyzing!"
echo
echo "📚 For troubleshooting, check the README.md file"
echo

# Optional: Ask if user wants to start the app
echo -n "Would you like to start the application now? (y/N): "
read -r response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
    echo
    echo "🚀 Starting Personal ATS Scanner..."
    echo "   Frontend: http://localhost:3000"
    echo "   Backend API: http://localhost:3001"
    echo
    echo "Press Ctrl+C to stop the application"
    echo
    npm run dev
fi