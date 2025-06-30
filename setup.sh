#!/bin/bash

# Local ATS Scanner Setup Script
# This script sets up the complete Tauri application with all dependencies

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
print_header() {
    echo -e "${BLUE}================================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}================================================${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check system requirements
check_requirements() {
    print_header "Checking System Requirements"
    
    local all_good=true
    
    # Check Node.js
    if command_exists node; then
        node_version=$(node --version)
        print_success "Node.js found: $node_version"
        
        # Check if version is >= 18
        major_version=$(echo $node_version | cut -d'.' -f1 | sed 's/v//')
        if [ "$major_version" -lt 18 ]; then
            print_error "Node.js version 18 or higher required. Found: $node_version"
            all_good=false
        fi
    else
        print_error "Node.js not found. Please install Node.js 18+ first."
        print_info "Download from: https://nodejs.org"
        all_good=false
    fi
    
    # Check npm
    if command_exists npm; then
        npm_version=$(npm --version)
        print_success "npm found: $npm_version"
    else
        print_error "npm not found. Please install npm."
        all_good=false
    fi
    
    # Check Rust
    if command_exists rustc; then
        rust_version=$(rustc --version)
        print_success "Rust found: $rust_version"
    else
        print_error "Rust not found. Please install Rust first."
        print_info "Install with: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        all_good=false
    fi
    
    # Check Cargo
    if command_exists cargo; then
        cargo_version=$(cargo --version)
        print_success "Cargo found: $cargo_version"
    else
        print_error "Cargo not found. Please install Rust/Cargo."
        all_good=false
    fi
    
    # Check Ollama
    if command_exists ollama; then
        ollama_version=$(ollama --version 2>/dev/null || echo "unknown")
        print_success "Ollama found: $ollama_version"
        
        # Test Ollama service
        if ollama list >/dev/null 2>&1; then
            print_success "Ollama service is running"
        else
            print_warning "Ollama service not running. Please start it with: ollama serve"
            print_info "You can continue setup, but you'll need to start Ollama to use the application"
        fi
    else
        print_error "Ollama not found. Please install Ollama first."
        print_info "Download from: https://ollama.ai"
        print_info "After installation, run: ollama serve"
        all_good=false
    fi
    
    if [ "$all_good" = false ]; then
        print_error "Please install missing dependencies and run setup again."
        exit 1
    fi
    
    print_success "All system requirements satisfied!"
}

# Install Node.js dependencies
install_node_dependencies() {
    print_header "Installing Node.js Dependencies"
    
    print_info "Installing frontend dependencies..."
    npm install
    
    if [ $? -eq 0 ]; then
        print_success "Node.js dependencies installed successfully"
    else
        print_error "Failed to install Node.js dependencies"
        exit 1
    fi
}

# Install additional required packages
install_additional_packages() {
    print_header "Installing Additional Packages"
    
    # Install Tauri CLI if not present
    if ! npm list -g @tauri-apps/cli >/dev/null 2>&1; then
        print_info "Installing Tauri CLI globally..."
        npm install -g @tauri-apps/cli
    else
        print_success "Tauri CLI already installed"
    fi
    
    # Install tailwindcss-animate if not in package.json
    print_info "Installing additional UI dependencies..."
    npm install tailwindcss-animate @radix-ui/react-slot
}

# Setup Rust dependencies
setup_rust_dependencies() {
    print_header "Setting up Rust Dependencies"
    
    print_info "Installing Rust dependencies..."
    cd src-tauri
    cargo fetch
    cd ..
    
    print_success "Rust dependencies fetched successfully"
}

# Initialize database
setup_database() {
    print_header "Setting up Local Database"
    
    # Create data directory
    mkdir -p data
    print_success "Data directory created"
    
    # The database will be initialized automatically when the app starts
    print_success "Database setup completed"
}

# Download recommended Ollama model
setup_ollama_models() {
    print_header "Setting up Ollama Models"
    
    if command_exists ollama; then
        # Check if any models are available
        models=$(ollama list 2>/dev/null | grep -v "NAME" | wc -l)
        
        if [ "$models" -eq 0 ]; then
            print_info "No models found. Downloading recommended model..."
            print_info "This may take several minutes depending on your internet connection..."
            
            if ollama pull llama2:7b; then
                print_success "Successfully downloaded llama2:7b model"
            else
                print_warning "Failed to download model. You can download manually later with:"
                print_info "ollama pull llama2:7b"
                print_info "Or try other models like: ollama pull mistral"
            fi
        else
            print_success "Found $models Ollama model(s) already installed"
        fi
    else
        print_warning "Ollama not available. Models will need to be downloaded separately."
    fi
}

# Create startup scripts
create_scripts() {
    print_header "Creating Helper Scripts"
    
    # Create scripts directory
    mkdir -p scripts
    
    # Test Ollama connection script
    cat > scripts/test-ollama.js << 'EOF'
const http = require('http');

const options = {
  hostname: 'localhost',
  port: 11434,
  path: '/api/tags',
  method: 'GET'
};

const req = http.request(options, (res) => {
  if (res.statusCode === 200) {
    console.log('✅ Ollama connection successful');
    process.exit(0);
  } else {
    console.log('❌ Ollama connection failed:', res.statusCode);
    process.exit(1);
  }
});

req.on('error', (err) => {
  console.log('❌ Ollama connection failed:', err.message);
  process.exit(1);
});

req.end();
EOF

    # Test database script
    cat > scripts/test-db.js << 'EOF'
const fs = require('fs');
const path = require('path');

const dbPath = path.join(__dirname, '..', 'data');

if (fs.existsSync(dbPath)) {
  console.log('✅ Database directory exists');
  process.exit(0);
} else {
  console.log('❌ Database directory not found');
  process.exit(1);
}
EOF

    # Database initialization script
    cat > scripts/init-db.js << 'EOF'
const fs = require('fs');
const path = require('path');

const dataDir = path.join(__dirname, '..', 'data');

if (!fs.existsSync(dataDir)) {
  fs.mkdirSync(dataDir, { recursive: true });
  console.log('✅ Database directory created');
} else {
  console.log('✅ Database directory already exists');
}

console.log('Database will be initialized automatically when the application starts');
EOF

    print_success "Helper scripts created"
}

# Run tests
run_tests() {
    print_header "Running Setup Verification"
    
    # Test Node.js setup
    print_info "Testing Node.js setup..."
    if npm run test:db >/dev/null 2>&1; then
        print_success "Database test passed"
    else
        print_warning "Database test failed (this is normal on first run)"
    fi
    
    # Test Ollama connection
    print_info "Testing Ollama connection..."
    if npm run test:ollama >/dev/null 2>&1; then
        print_success "Ollama connection test passed"
    else
        print_warning "Ollama connection test failed (make sure Ollama is running)"
    fi
    
    # Test Rust compilation
    print_info "Testing Rust compilation..."
    cd src-tauri
    if cargo check >/dev/null 2>&1; then
        print_success "Rust compilation test passed"
        cd ..
    else
        print_warning "Rust compilation test failed"
        cd ..
    fi
}

# Create .env file with default settings
create_env_file() {
    print_header "Creating Environment Configuration"
    
    if [ ! -f .env ]; then
        cat > .env << 'EOF'
# ATS Scanner Environment Configuration
OLLAMA_HOST=http://localhost:11434
DATABASE_PATH=./data/ats_scanner.db
LOG_LEVEL=info
MODEL_CACHE_SIZE=2
PROCESSING_TIMEOUT=30000
EOF
        print_success "Environment file (.env) created"
    else
        print_success "Environment file already exists"
    fi
}

# Main setup function
main() {
    print_header "Local ATS Scanner Setup"
    echo "This script will set up the complete Tauri application with all dependencies."
    echo ""
    
    check_requirements
    echo ""
    
    install_node_dependencies
    echo ""
    
    install_additional_packages
    echo ""
    
    setup_rust_dependencies
    echo ""
    
    setup_database
    echo ""
    
    setup_ollama_models
    echo ""
    
    create_scripts
    echo ""
    
    create_env_file
    echo ""
    
    run_tests
    echo ""
    
    print_header "Setup Complete!"
    echo ""
    print_success "🎉 ATS Scanner has been set up successfully!"
    echo ""
    print_info "Next steps:"
    echo "1. Make sure Ollama is running: ollama serve"
    echo "2. Start the development server: npm run dev"
    echo "3. The application will open in a new window"
    echo ""
    print_info "Useful commands:"
    echo "• npm run dev          - Start development server"
    echo "• npm run build        - Build for production"
    echo "• npm run test:ollama  - Test Ollama connection"
    echo "• npm run test:db      - Test database setup"
    echo ""
    print_info "For help and documentation:"
    echo "• Check the README.md file"
    echo "• Visit the project repository"
    echo "• Report issues on GitHub"
    echo ""
    print_warning "Note: Make sure to download at least one Ollama model if you haven't already:"
    echo "ollama pull llama2:7b"
    echo ""
}

# Run main function
main "$@"