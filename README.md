# ATS Scanner

A completely local, privacy-first ATS (Applicant Tracking System) scanner that uses advanced AI models to analyze and optimize resumes. Built with Tauri, React, and Rust for professional cross-platform distribution with automatic updates.

## ✨ Features

### Core Functionality
- **🔒 Privacy First**: All processing happens locally - your data never leaves your computer
- **🤖 Advanced AI Analysis**: Uses local Ollama models for intelligent resume analysis
- **📄 Multi-Format Support**: Analyze PDF, DOCX, and TXT resume files
- **📊 Comprehensive Scoring**: Detailed ATS compatibility scores with category breakdowns
- **🎯 Smart Recommendations**: Actionable suggestions to improve your resume
- **⚡ Real-Time Optimization**: Live feedback as you edit your resume
- **🔄 Batch Processing**: Analyze multiple resumes against multiple job descriptions

### Advanced AI Features
- **🧠 ML-Powered Insights**: Machine learning predictions for interview probability and salary estimates
- **🎨 Smart Optimization Engine**: Context-aware suggestions for resume improvement
- **📈 Competitive Analysis**: Compare your resume against industry benchmarks
- **🎯 ATS Simulation**: Simulate how different ATS systems will parse your resume
- **📝 Achievement Analysis**: XYZ method suggestions for quantifying accomplishments
- **🔍 Semantic Analysis**: Deep understanding of resume content and job requirements
- **📊 Format Compatibility**: Comprehensive format checking for ATS systems

### User Experience
- **💾 Local Storage**: SQLite database for all your analysis history
- **🌙 Dark Mode**: Beautiful interface with light and dark themes
- **🚀 Cross-Platform**: Native apps for macOS, Windows, and Linux
- **🔄 Automatic Updates**: Built-in update system with notifications
- **📱 Modern UI**: Clean, responsive design with accessibility features
- **⚡ High Performance**: Optimized for speed and low resource usage

## 📦 Installation

### For End Users (Recommended)

Download the latest version for your platform:

- **🍎 macOS**: [Download DMG](https://github.com/your-username/ats-scanner/releases/latest/download/ATS%20Scanner_1.0.0_aarch64.dmg)
- **🪟 Windows**: [Download MSI](https://github.com/your-username/ats-scanner/releases/latest/download/ATS%20Scanner_1.0.0_x64.msi)
- **🐧 Linux**: [Download AppImage](https://github.com/your-username/ats-scanner/releases/latest/download/ats-scanner_1.0.0_amd64.AppImage)

**Installation Instructions:**
- **macOS**: Open the DMG and drag ATS Scanner to Applications
- **Windows**: Run the MSI installer and follow the setup wizard
- **Linux**: Make the AppImage executable and run: `chmod +x filename.AppImage && ./filename.AppImage`

### System Requirements

- **Memory**: 8GB RAM recommended (4GB minimum for basic usage)
- **Storage**: 5GB free space (additional space for AI models)
- **Processor**: Dual-core 2.0GHz minimum (quad-core recommended)
- **Operating System**: 
  - macOS 10.15+ (Catalina or later)
  - Windows 10+ (64-bit)
  - Linux (most modern distributions)

### Prerequisites for AI Analysis

**Ollama (Required):**
1. Download from [ollama.ai](https://ollama.ai)
2. Install and start the service: `ollama serve`
3. Download a model: `ollama pull llama2:7b`

**Recommended Models:**
- **General Use**: `llama2:7b` (balanced performance)
- **Fast Analysis**: `mistral:7b` (faster processing)
- **Technical Resumes**: `codellama:7b` (better for tech content)
- **High Accuracy**: `llama2:13b` (requires more memory)

## 🔧 Development Setup

For developers who want to build from source:

### Required Dependencies

1. **Node.js 18+**
   - Download from [nodejs.org](https://nodejs.org)
   - Verify: `node --version`

2. **Rust**
   - Install: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
   - Verify: `rustc --version`

3. **Tauri CLI**
   - Install: `npm install -g @tauri-apps/cli`

### Development Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-username/ats-scanner.git
   cd ats-scanner
   ```

2. **Install dependencies**
   ```bash
   npm ci
   ```

3. **Start development server**
   ```bash
   npm run dev
   ```

4. **Build for production**
   ```bash
   npm run build
   ```

### Development Scripts

```bash
# Development
npm run dev              # Start development server with hot reload
npm run build            # Build for production
npm run build:mac        # Build for macOS
npm run build:windows    # Build for Windows
npm run build:linux      # Build for Linux

# Database
npm run db:init          # Initialize database schema
npm run test:db          # Test database connection

# Testing
npm run test             # Run unit tests
npm run test:e2e         # Run end-to-end tests
npm run test:coverage    # Generate coverage report

# Ollama Integration
npm run test:ollama      # Test Ollama connection
```

## 📱 How to Use

### Getting Started

1. **Install ATS Scanner** using one of the installers above
2. **Install Ollama** from [ollama.ai](https://ollama.ai)
3. **Download an AI model**: `ollama pull llama2:7b`
4. **Start Ollama**: `ollama serve`
5. **Launch ATS Scanner** from your Applications/Start Menu

### Analyzing Your Resume

1. **Open the Analysis tab**
2. **Select your AI model** from the dropdown
3. **Upload your resume** (supports PDF, DOCX, and TXT files)
4. **Paste the job description** you're targeting
5. **Click "Analyze Resume"** and wait for results
6. **Review detailed scores and recommendations**

### Understanding Your Results

**📊 ATS Compatibility Score**: Overall rating of how well your resume will perform in ATS systems

**🎯 Analysis Categories**:
- **Keyword Matching**: How well your resume matches job requirements
- **Format Compatibility**: Technical formatting for ATS parsing
- **Content Quality**: Overall strength of your resume content
- **Achievement Quantification**: Use of numbers and metrics

**💡 AI Recommendations**:
- Specific suggestions for improvement
- Keywords to add or optimize
- Formatting fixes for better ATS compatibility
- Achievement enhancement suggestions

### Advanced Features

**🧠 ML Insights**:
- Interview probability predictions
- Salary range estimates
- Career growth recommendations
- Competitive positioning analysis

**⚡ Real-time Optimization**:
- Live feedback as you edit
- Contextual suggestions
- Format issue detection
- Achievement analyzer

**📈 Competitive Analysis**:
- Industry benchmark comparisons
- Skills gap identification
- Market positioning insights

**🎯 ATS Simulation**:
- Test against different ATS systems
- Format compatibility checking
- Parsing accuracy assessment

## 🔄 Automatic Updates

ATS Scanner includes a built-in update system that keeps your app current with the latest features and improvements.

### How Updates Work

1. **Automatic Checking**: App checks for updates when started and periodically
2. **User Notification**: You'll see a notification when a new version is available
3. **One-Click Download**: Download updates directly through the app
4. **Secure Installation**: Updates are cryptographically signed for security
5. **Data Preservation**: All your resume data and analysis history is preserved

### Update Notifications

- **Non-intrusive**: Updates are offered, never forced
- **Release Notes**: See what's new before updating
- **Progress Tracking**: Watch download and installation progress
- **Rollback Protection**: Automatic backups before major updates

## 🏗️ Technical Architecture

### Project Structure
```
ats-scanner/
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs              # Application entry point
│   │   ├── commands.rs          # Tauri API commands
│   │   ├── database.rs          # SQLite operations
│   │   ├── document.rs          # PDF/DOCX/TXT parsing
│   │   ├── ollama.rs            # AI model integration
│   │   ├── scoring.rs           # ATS scoring algorithms
│   │   ├── ml_insights.rs       # ML predictions engine
│   │   ├── smart_optimizer.rs   # Optimization algorithms
│   │   ├── competitive_analyzer.rs # Benchmarking
│   │   ├── achievement_analyzer.rs # XYZ method analysis
│   │   ├── semantic_analyzer.rs # Content understanding
│   │   ├── ats_simulator.rs     # ATS system simulation
│   │   ├── realtime_optimizer.rs # Live optimization
│   │   ├── format_checker.rs    # Format validation
│   │   └── models.rs            # Data structures
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # App configuration
├── src/                         # React frontend
│   ├── components/              # UI components
│   │   ├── ui/                  # Base UI components
│   │   ├── layout/              # Layout components
│   │   └── pages/               # Page components
│   ├── hooks/                   # Custom React hooks
│   ├── store/                   # State management
│   ├── types/                   # TypeScript definitions
│   └── lib/                     # Utility functions
├── .github/workflows/           # CI/CD automation
├── scripts/                     # Build and setup scripts
├── e2e/                        # End-to-end tests
└── dist/                       # Built frontend assets
```

### Technology Stack

**Frontend**:
- React 18 with TypeScript
- Tailwind CSS for styling
- Radix UI for components
- Zustand for state management
- Vite for build tooling

**Backend**:
- Rust with Tauri framework
- SQLite with sqlx for database
- reqwest for HTTP client
- tokio for async runtime
- serde for serialization

**AI Integration**:
- Local Ollama API integration
- Support for multiple model types
- Streaming response handling
- Model performance optimization

**Distribution**:
- GitHub Actions for CI/CD
- Cross-platform builds (macOS, Windows, Linux)
- Automatic update system with digital signatures
- Professional installers for each platform

## ⚙️ Configuration & Settings

### In-App Settings

ATS Scanner provides a comprehensive settings interface:

**🤖 AI Model Configuration**:
- Select and manage Ollama models
- Adjust analysis parameters
- Configure performance vs accuracy trade-offs

**🎨 Interface Preferences**:
- Light/dark theme toggle
- Language selection
- Accessibility options
- Result display preferences

**🔒 Privacy Settings**:
- Data retention policies
- Analysis history management
- Export/import preferences
- Update notification settings

**📊 Analysis Parameters**:
- Scoring algorithm weights
- Keyword matching sensitivity
- Format checking strictness
- Industry-specific optimizations

### Advanced Configuration

For developers and power users:

**Environment Variables** (development):
```env
OLLAMA_HOST=http://localhost:11434
DATABASE_PATH=./data/ats_scanner.db
LOG_LEVEL=info
MODEL_CACHE_SIZE=2
PROCESSING_TIMEOUT=30000
```

**Model Recommendations**:
- **Balanced**: `llama2:7b` (recommended for most users)
- **Speed**: `mistral:7b` (faster analysis, good quality)
- **Accuracy**: `llama2:13b` (best results, requires more RAM)
- **Technical**: `codellama:7b` (optimized for technical resumes)
- **Multilingual**: `aya:8b` (supports multiple languages)

## 🔍 Troubleshooting

### Installation Issues

**macOS "App is damaged" error**:
- Right-click the app → Open → Open anyway
- Or run: `xattr -cr "/Applications/ATS Scanner.app"`

**Windows "Unknown publisher" warning**:
- Click "More info" → "Run anyway"
- Or right-click installer → Properties → Unblock

**Linux AppImage won't run**:
- Make executable: `chmod +x ats-scanner_1.0.0_amd64.AppImage`
- Install FUSE if needed: `sudo apt install fuse`

### Runtime Issues

**❌ "Ollama Connection Failed"**:
- Ensure Ollama is running: `ollama serve`
- Check if port 11434 is available: `lsof -i :11434`
- Verify firewall settings allow localhost connections
- Try restarting Ollama service

**❌ "Model Not Found"**:
- Download a model: `ollama pull llama2:7b`
- Check available models: `ollama list`
- Verify model name spelling in app settings

**❌ "Analysis Failed"**:
- Check Ollama logs for errors
- Try a different/smaller model
- Ensure sufficient system memory
- Restart both Ollama and ATS Scanner

**❌ "Database Errors"**:
- Check app data directory permissions
- Close other instances of the app
- Contact support if data corruption is suspected

### Performance Optimization

**🚀 For faster analysis**:
- Use smaller models (7B parameters)
- Close other memory-intensive applications  
- Enable hardware acceleration if available

**🎯 For better accuracy**:
- Use larger models (13B+ parameters)
- Ensure adequate system memory (16GB+ recommended)
- Allow longer processing timeouts

**💾 For lower memory usage**:
- Use quantized models (Q4_K_M variants)
- Reduce model cache size in settings
- Process one resume at a time

### Getting Help

**📚 Self-Help Resources**:
- Check the [Issues](https://github.com/your-username/ats-scanner/issues) page
- Search existing questions and solutions
- Review the [Discussions](https://github.com/your-username/ats-scanner/discussions) forum

**🐛 Reporting Bugs**:
1. Check if issue already exists
2. Include your OS and app version
3. Describe steps to reproduce
4. Include relevant error messages or logs

**💡 Feature Requests**:
- Use GitHub Discussions for feature ideas
- Describe your use case and expected behavior
- Check if similar requests already exist

## 🔒 Privacy & Security

### Privacy-First Design

**🔐 Complete Local Processing**:
- All resume analysis happens on your computer
- No data is ever sent to external servers
- Your resumes and job descriptions stay private
- Analysis results stored locally in encrypted database

**🛡️ Security Features**:
- Sandboxed document processing environment
- Secure file validation and parsing
- Encrypted local data storage
- No telemetry or tracking
- Regular security updates via automatic update system

**📊 Data Handling**:
- All user data stored in standard OS locations
- Easy backup and migration options
- Complete data export functionality
- User-controlled data retention policies

### What Data is Stored Locally

- Resume files and extracted text
- Job descriptions and analysis history
- ATS compatibility scores and recommendations  
- User preferences and settings
- AI model cache and optimization data

**Note**: No personal data ever leaves your computer. All AI processing uses your local Ollama installation.

## 📊 Performance & System Impact

### Resource Usage

**Memory Requirements**:
- **App Base**: ~200MB RAM
- **AI Models**: 2-8GB RAM (depending on model size)
- **Total Recommended**: 8GB+ system RAM

**Storage Requirements**:
- **App Installation**: ~50MB
- **User Data**: <10MB per 1000 analyses
- **AI Models**: 2-8GB per model (stored by Ollama)

**Performance Benchmarks**:
- **App Startup**: <5 seconds on modern hardware
- **Document Parsing**: 1-3 seconds for typical resumes
- **AI Analysis**: 5-15 seconds (varies by model and content length)
- **Database Operations**: <100ms for most queries

### Optimization Tips

**⚡ Maximum Performance**:
- Use SSD storage for better I/O
- Close unnecessary applications during analysis
- Use GPU acceleration if available (some models)
- Keep Ollama models on fast storage

**🔋 Battery Optimization** (laptops):
- Use smaller, efficient models (7B parameters)
- Reduce analysis frequency
- Enable power management in OS settings

## 🤝 Contributing

We welcome contributions from the community! Whether you're fixing bugs, adding features, or improving documentation, your help makes ATS Scanner better for everyone.

### How to Contribute

1. **🍴 Fork the repository** on GitHub
2. **🌿 Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **💻 Make your changes** following our coding standards
4. **🧪 Add tests** for new functionality
5. **📝 Update documentation** as needed
6. **✅ Test thoroughly** on multiple platforms if possible
7. **📤 Commit your changes**: `git commit -m 'Add amazing feature'`
8. **🚀 Push to your branch**: `git push origin feature/amazing-feature`
9. **🔄 Open a Pull Request** with a clear description

### Development Guidelines

**📋 Code Standards**:
- Follow Rust and TypeScript best practices
- Use meaningful variable and function names
- Add comprehensive comments for complex logic
- Ensure code is properly formatted (use `cargo fmt` and `prettier`)

**🧪 Testing Requirements**:
- Add unit tests for new backend functions
- Add component tests for new UI features
- Test cross-platform compatibility when possible
- Verify accessibility standards are met

**📚 Documentation**:
- Update README.md for user-facing changes
- Add inline code documentation
- Update API documentation for new commands
- Include examples for new features

**🔒 Privacy & Security**:
- Maintain privacy-first principles
- No external data transmission
- Secure file handling practices
- Follow security best practices

### Types of Contributions Welcome

**🐛 Bug Fixes**:
- Runtime error fixes
- UI/UX improvements
- Performance optimizations
- Cross-platform compatibility issues

**✨ New Features**:
- Additional AI analysis capabilities
- New export formats
- Enhanced optimization algorithms
- Improved user interface components

**📖 Documentation**:
- User guides and tutorials
- API documentation
- Code examples
- Troubleshooting guides

**🌐 Localization**:
- Interface translations
- Locale-specific optimizations
- Cultural adaptation of recommendations

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

**What this means for you**:
- ✅ Free to use for personal and commercial purposes
- ✅ Modify and distribute the source code
- ✅ Create derivative works
- ✅ No warranty or liability from the authors

## 🙏 Acknowledgments

**🙏 Special Thanks To**:
- **[Ollama](https://ollama.ai)** - For providing excellent local AI model infrastructure
- **[Tauri](https://tauri.app)** - For the amazing cross-platform application framework  
- **[Radix UI](https://radix-ui.com)** - For accessible, unstyled UI components
- **[Tailwind CSS](https://tailwindcss.com)** - For the utility-first CSS framework
- **The Rust Community** - For exceptional documentation and libraries
- **The React Community** - For building amazing developer tools
- **Open Source Contributors** - For making privacy-focused software possible

**🔬 Research & Inspiration**:
- Academic research on ATS systems and resume parsing
- Job seekers who provided feedback and testing
- HR professionals who shared insights on ATS functionality
- The broader open-source community for privacy-first software

## 📞 Support & Community

### Getting Help

**📚 Documentation**:
- **README** (you are here) - Complete setup and usage guide
- **[PACKAGING_GUIDE.md](PACKAGING_GUIDE.md)** - Distribution and building guide
- **[DISTRIBUTION.md](DISTRIBUTION.md)** - Advanced distribution information

**💬 Community Support**:
- **[GitHub Discussions](https://github.com/your-username/ats-scanner/discussions)** - Ask questions, share tips
- **[GitHub Issues](https://github.com/your-username/ats-scanner/issues)** - Report bugs, request features

### Stay Updated

**📡 Release Information**:
- Watch this repository for release notifications
- Check the [Releases page](https://github.com/your-username/ats-scanner/releases) for latest versions
- Enable automatic updates in the app for seamless upgrades

**🌟 Show Your Support**:
- ⭐ Star this repository if ATS Scanner helps you
- 🐛 Report bugs to help improve the app
- 💡 Suggest features that would benefit job seekers
- 🔄 Share with others who could benefit from privacy-first resume analysis

---

## 🎯 Project Mission

**ATS Scanner exists to democratize access to professional resume optimization while protecting user privacy.**

In a world where applicant tracking systems filter millions of resumes, job seekers deserve:
- 🔒 **Privacy**: Your personal information stays on your computer
- 🤖 **AI-Powered Insights**: Advanced analysis using state-of-the-art models
- 💰 **Zero Cost**: No subscriptions, no hidden fees, no data harvesting
- 🌍 **Universal Access**: Works offline, cross-platform, for everyone

**Your resume is personal. Your analysis should be too.**

---

*Built with ❤️ for job seekers everywhere*  
*Privacy-first • Local processing • Open source*