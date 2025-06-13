# Personal ATS Scanner

A modern, minimalist personal ATS (Applicant Tracking System) scanner that analyzes how well your resume matches job descriptions using local AI models via Ollama.

## Features

- **Resume Analysis**: Upload PDF or Word documents and get detailed compatibility analysis
- **AI-Powered Insights**: Uses Mistral and Qwen2.5 models for comprehensive resume evaluation
- **Skills Gap Analysis**: Identifies missing skills and provides actionable recommendations
- **ATS Optimization**: Keyword analysis and formatting suggestions for better ATS compatibility
- **Privacy-First**: All processing happens locally - your data never leaves your machine
- **Easy Setup**: One-command installation and setup

## Quick Start

1. **Clone and Setup**
   ```bash
   git clone <repository-url>
   cd personal-ats-scanner
   chmod +x setup.sh && ./setup.sh
   ```

2. **Start the Application**
   ```bash
   npm run dev
   ```

3. **Open in Browser**
   Navigate to `http://localhost:3000`

## Prerequisites

- **Node.js 18+**: Download from [nodejs.org](https://nodejs.org/)
- **Ollama**: Download from [ollama.ai](https://ollama.ai)

## Detailed Setup

### 1. Install Ollama and Models

```bash
# Install Ollama (visit ollama.ai for platform-specific instructions)

# Pull required models
ollama pull mistral:latest
ollama pull qwen2.5:14b

# Start Ollama service
ollama serve
```

### 2. Install Dependencies

```bash
npm install
```

### 3. Initialize Database

```bash
npm run setup
```

## Usage

### Dashboard

- **Resume History**: View all uploaded resumes with their analysis scores
- **New Scan**: Upload a new resume and job description for analysis

### Analysis Process

1. **Upload Resume**: Drag and drop PDF or Word documents
2. **Add Job Description**: Paste the complete job posting
3. **Select AI Model**: Choose between fast (Mistral) or detailed (Qwen2.5) analysis
4. **Review Results**: Get comprehensive insights and recommendations

### Analysis Results Include

- **Overall Match Score**: 0-100% compatibility rating
- **Skills Analysis**: Found vs. missing skills with evidence
- **Experience Matching**: Years and level assessment
- **Keyword Optimization**: ATS-friendly keyword analysis
- **Actionable Recommendations**: Specific improvement suggestions
- **Competitive Analysis**: Market positioning insights

## API Endpoints

- `GET /api/resumes` - List all resumes
- `POST /api/resumes` - Upload new resume
- `POST /api/scans` - Create new analysis
- `GET /api/scans/:id` - Get analysis results
- `GET /api/analyze/models` - Get available AI models
- `GET /api/health` - Server health check

## File Structure

```
personal-ats-scanner/
├── src/                    # Frontend React application
│   ├── components/         # Reusable React components
│   ├── pages/             # Main application pages
│   ├── types/             # TypeScript type definitions
│   └── utils/             # Helper functions and API calls
├── server/                # Backend Express.js server
│   ├── routes/            # API route handlers
│   ├── services/          # Business logic services
│   └── utils/             # Server utilities
├── database/              # SQLite database files
├── uploads/               # File upload storage
└── setup.sh              # Installation script
```

## Tech Stack

- **Frontend**: React 18, TypeScript, Tailwind CSS, Vite
- **Backend**: Node.js, Express.js, SQLite
- **AI**: Ollama (Mistral, Qwen2.5 models)
- **File Processing**: pdf-parse, mammoth.js

## Configuration

Environment variables (create `.env` from `.env.example`):

```env
PORT=3001
DATABASE_PATH=./database/ats-scanner.db
OLLAMA_BASE_URL=http://localhost:11434
MAX_FILE_SIZE=10485760
UPLOAD_DIR=./uploads
```

## Troubleshooting

### Common Issues

**"Ollama service not available"**
- Ensure Ollama is installed and running: `ollama serve`
- Check models are installed: `ollama list`

**"Database initialization failed"**
- Ensure write permissions in the database directory
- Delete existing database file and re-run setup

**"File upload failed"**
- Check file size (max 10MB)
- Ensure file is PDF or Word format (.pdf, .doc, .docx)

**"Analysis timeout"**
- Large documents may take longer with detailed models
- Try using Mistral model for faster processing
- Ensure Ollama has sufficient system resources

### Debug Mode

Set environment variable for detailed logging:
```bash
NODE_ENV=development npm run dev
```

### Model Performance

- **mistral:latest**: Faster analysis, good for quick scans
- **qwen2.5:14b**: More detailed analysis, better insights (requires more RAM)

## Development

### Running in Development

```bash
# Start both frontend and backend
npm run dev

# Start only backend
npm run server

# Start only frontend
npm run client
```

### Building for Production

```bash
npm run build
```

## Privacy & Security

- All data processing happens locally on your machine
- Resume content never leaves your device
- No external API calls except to local Ollama instance
- SQLite database stored locally

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Support

For issues, questions, or feature requests, please create an issue in the GitHub repository.

---

**Built with ❤️ for job seekers who value privacy and detailed insights.**