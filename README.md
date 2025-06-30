# Local ATS Scanner

A completely local, privacy-first ATS (Applicant Tracking System) scanner that uses Ollama models to analyze and optimize resumes. Built with Tauri, React, and Rust for cross-platform compatibility and high performance.

## ✨ Features

- **🔒 Privacy First**: All processing happens locally - your data never leaves your computer
- **🤖 AI-Powered**: Uses local Ollama models for intelligent resume analysis
- **📄 Multi-Format Support**: Analyze PDF, DOCX, and TXT resume files
- **📊 Detailed Scoring**: Comprehensive ATS compatibility scores with category breakdowns
- **🎯 Smart Recommendations**: Actionable suggestions to improve your resume
- **⚡ Real-Time Analysis**: Fast local processing with instant feedback
- **🔄 Batch Processing**: Analyze multiple resumes against multiple job descriptions
- **💾 Local Storage**: SQLite database for all your analysis history
- **🌙 Dark Mode**: Beautiful interface with light and dark themes
- **🚀 Cross-Platform**: Runs on Windows, macOS, and Linux

## 🔧 Prerequisites

Before running the setup, ensure you have the following installed:

### Required Dependencies

1. **Node.js 18+**
   - Download from [nodejs.org](https://nodejs.org)
   - Verify installation: `node --version`

2. **Rust**
   - Install via rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
   - Verify installation: `rustc --version`

3. **Ollama**
   - Download from [ollama.ai](https://ollama.ai)
   - Start the service: `ollama serve`
   - Download a model: `ollama pull llama2:7b`

### System Requirements

- **Memory**: 8GB RAM recommended (4GB minimum)
- **Storage**: 10GB free space (additional space for AI models)
- **Processor**: Dual-core 2.0GHz minimum (quad-core recommended)

## 🚀 Quick Setup

1. **Clone or download this repository**
   ```bash
   git clone <repository-url>
   cd ats-scanner
   ```

2. **Run the automated setup script**
   ```bash
   chmod +x setup.sh
   ./setup.sh
   ```

3. **Start the application**
   ```bash
   npm run dev
   ```

The setup script will:
- ✅ Check all system requirements
- ✅ Install Node.js and Rust dependencies
- ✅ Set up the local SQLite database
- ✅ Configure Ollama integration
- ✅ Download a recommended AI model (if none exists)
- ✅ Run verification tests

## 📱 Usage

### First Time Setup

1. **Ensure Ollama is running**
   ```bash
   ollama serve
   ```

2. **Download an AI model** (if not done during setup)
   ```bash
   ollama pull llama2:7b
   # Or try other models:
   # ollama pull mistral
   # ollama pull codellama
   ```

3. **Start the application**
   ```bash
   npm run dev
   ```

### Analyzing a Resume

1. **Navigate to the Analysis tab**
2. **Select your AI model** from the dropdown
3. **Upload your resume** (PDF, DOCX, or TXT)
4. **Paste the job description** you're targeting
5. **Click "Analyze"** and wait for results
6. **Review your scores** and recommendations

### Key Features

- **Dashboard**: Overview of your analysis history and statistics
- **Analysis**: Upload and analyze resumes against job descriptions
- **Optimization**: AI-powered resume improvement suggestions
- **Job Library**: Manage and organize job descriptions
- **Settings**: Configure models, preferences, and system settings

## 🏗️ Project Structure

```
ats-scanner/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Main application entry
│   │   ├── commands.rs     # Tauri commands
│   │   ├── database.rs     # SQLite database operations
│   │   ├── document.rs     # Document parsing (PDF, DOCX, TXT)
│   │   ├── ollama.rs       # Ollama API integration
│   │   ├── scoring.rs      # ATS scoring algorithms
│   │   ├── models.rs       # Data models
│   │   └── utils.rs        # Utility functions
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
├── src/                    # React frontend
│   ├── components/         # React components
│   ├── pages/             # Main application pages
│   ├── store/             # Zustand state management
│   └── lib/               # Utility functions
├── scripts/               # Helper scripts
├── data/                  # Local database storage
├── setup.sh              # Automated setup script
├── package.json          # Node.js dependencies
└── README.md             # This file
```

## 🔧 Development

### Available Scripts

```bash
# Development
npm run dev          # Start development server with hot reload
npm run build        # Build for production
npm run preview      # Preview production build

# Database
npm run db:init      # Initialize database schema
npm run test:db      # Test database connection

# Ollama
npm run test:ollama  # Test Ollama connection

# Tauri
npm run tauri dev    # Start Tauri development mode
npm run tauri build  # Build Tauri application
```

### Adding New Features

1. **Backend (Rust)**: Add new Tauri commands in `src-tauri/src/commands.rs`
2. **Frontend (React)**: Create components in `src/components/`
3. **Database**: Extend schema in `src-tauri/src/database.rs`
4. **AI Integration**: Modify prompts in `src-tauri/src/ollama.rs`

## 🛠️ Configuration

### Environment Variables

Create a `.env` file in the root directory:

```env
OLLAMA_HOST=http://localhost:11434
DATABASE_PATH=./data/ats_scanner.db
LOG_LEVEL=info
MODEL_CACHE_SIZE=2
PROCESSING_TIMEOUT=30000
```

### Ollama Models

Recommended models for different use cases:

- **General Use**: `llama2:7b` (balanced performance and accuracy)
- **Fast Analysis**: `mistral:7b` (faster processing)
- **Technical Resumes**: `codellama:7b` (better for technical content)
- **High Accuracy**: `llama2:13b` (requires more memory)

Download models with:
```bash
ollama pull <model-name>
```

## 🔍 Troubleshooting

### Common Issues

**Ollama Connection Failed**
- Ensure Ollama is running: `ollama serve`
- Check if port 11434 is available
- Verify firewall settings

**Model Not Found**
- Download a model: `ollama pull llama2:7b`
- Check available models: `ollama list`

**Database Errors**
- Check data directory permissions
- Reinitialize database: `npm run db:init`

**Build Failures**
- Update Rust: `rustup update`
- Clean dependencies: `cargo clean && npm clean-install`

### Performance Optimization

1. **For faster analysis**: Use smaller models (7B parameters)
2. **For better accuracy**: Use larger models (13B+ parameters)
3. **For low memory**: Adjust `MODEL_CACHE_SIZE` in settings
4. **For slow processing**: Check system resources and model size

## 📊 Technical Details

### Architecture

- **Frontend**: React 18 with TypeScript and Tailwind CSS
- **Backend**: Rust with Tauri framework
- **Database**: SQLite with sqlx for type-safe queries
- **AI Integration**: Local Ollama API for model communication
- **Document Parsing**: Native Rust libraries for PDF/DOCX/TXT
- **State Management**: Zustand for React state
- **UI Components**: Radix UI with custom styling

### Security Features

- **Local Processing**: All data stays on your machine
- **File Validation**: Secure document parsing with malware protection
- **Data Encryption**: Sensitive information encrypted at rest
- **Sandboxed Execution**: Isolated document processing environment
- **No Telemetry**: No data collection or external communications

### Performance Characteristics

- **Startup Time**: < 5 seconds on modern hardware
- **Document Parsing**: < 3 seconds for typical resumes
- **AI Analysis**: 5-15 seconds depending on model and content
- **Memory Usage**: 200MB base + model size (typically 2-8GB)
- **Storage**: < 10MB per 1000 analyses in database

## 🤝 Contributing

We welcome contributions! Here's how to get started:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes** following the existing code style
4. **Add tests** for new functionality
5. **Commit your changes**: `git commit -m 'Add amazing feature'`
6. **Push to the branch**: `git push origin feature/amazing-feature`
7. **Open a Pull Request**

### Development Guidelines

- Follow Rust and TypeScript best practices
- Add tests for new features
- Update documentation for API changes
- Ensure cross-platform compatibility
- Maintain privacy-first principles

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Ollama** - For providing excellent local AI model infrastructure
- **Tauri** - For the amazing cross-platform application framework
- **Radix UI** - For accessible UI components
- **Tailwind CSS** - For the utility-first CSS framework
- **The Rust Community** - For excellent documentation and libraries
- **Open Source Contributors** - For making this project possible

## 📞 Support

- **Issues**: Report bugs on [GitHub Issues](https://github.com/your-org/ats-scanner/issues)
- **Discussions**: Join conversations on [GitHub Discussions](https://github.com/your-org/ats-scanner/discussions)
- **Documentation**: Check the [Wiki](https://github.com/your-org/ats-scanner/wiki)

---

**Note**: This is a privacy-first application. All resume analysis happens locally on your machine. No data is sent to external servers or third parties.