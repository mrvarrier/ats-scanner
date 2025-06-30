import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, fireEvent, waitFor } from './utils'
import userEvent from '@testing-library/user-event'
import { invoke } from '@tauri-apps/api/tauri'
import AnalysisPage from '../pages/AnalysisPage'
import { 
  mockAnalysisResult, 
  mockJobDescription, 
  mockOllamaModels,
  createMockFile,
  createDataTransfer
} from './utils'

// Mock Tauri invoke
vi.mock('@tauri-apps/api/tauri')
const mockInvoke = vi.mocked(invoke)

describe('AnalysisPage', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders the analysis page with initial elements', () => {
    render(<AnalysisPage />)
    
    expect(screen.getByText('Resume Analysis')).toBeInTheDocument()
    expect(screen.getByText('Upload Resume')).toBeInTheDocument()
    expect(screen.getByText('Select Job Description')).toBeInTheDocument()
    expect(screen.getByText('Choose AI Model')).toBeInTheDocument()
  })

  it('loads job descriptions and models on mount', async () => {
    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels })

    render(<AnalysisPage />)

    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith('get_job_descriptions')
      expect(mockInvoke).toHaveBeenCalledWith('get_ollama_models')
    })
  })

  it('handles file upload via drag and drop', async () => {
    const user = userEvent.setup()
    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels })
      .mockResolvedValueOnce({ 
        success: true, 
        data: { 
          filename: 'test.pdf', 
          content: 'Resume content',
          file_type: 'pdf',
          size: 1024
        }
      })

    render(<AnalysisPage />)

    const dropzone = screen.getByText(/drag & drop your resume/i).closest('[data-testid="dropzone"]')
    expect(dropzone).toBeInTheDocument()

    if (dropzone) {
      const file = createMockFile('resume.pdf', 'Resume content', 'application/pdf')
      const dataTransfer = createDataTransfer([file])

      fireEvent.drop(dropzone, { dataTransfer })

      await waitFor(() => {
        expect(mockInvoke).toHaveBeenCalledWith('parse_document', expect.objectContaining({
          filename: 'resume.pdf'
        }))
      })
    }
  })

  it('prevents analysis when requirements are not met', async () => {
    render(<AnalysisPage />)

    const analyzeButton = screen.getByText('Analyze Resume')
    expect(analyzeButton).toBeDisabled()
  })

  it('enables analysis when all requirements are met', async () => {
    const user = userEvent.setup()
    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels })
      .mockResolvedValueOnce({ 
        success: true, 
        data: { 
          filename: 'test.pdf', 
          content: 'Resume content',
          file_type: 'pdf'
        }
      })

    render(<AnalysisPage />)

    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('Senior Software Engineer')).toBeInTheDocument()
    })

    // Upload a file
    const dropzone = screen.getByText(/drag & drop your resume/i).closest('[data-testid="dropzone"]')
    if (dropzone) {
      const file = createMockFile('resume.pdf', 'Resume content', 'application/pdf')
      const dataTransfer = createDataTransfer([file])
      fireEvent.drop(dropzone, { dataTransfer })
    }

    // Select job description and model
    await user.selectOptions(screen.getByLabelText(/job description/i), mockJobDescription.id)
    await user.selectOptions(screen.getByLabelText(/ai model/i), mockOllamaModels[0].name)

    await waitFor(() => {
      const analyzeButton = screen.getByText('Analyze Resume')
      expect(analyzeButton).not.toBeDisabled()
    })
  })

  it('performs analysis and displays results', async () => {
    const user = userEvent.setup()
    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels })
      .mockResolvedValueOnce({ 
        success: true, 
        data: { 
          filename: 'test.pdf', 
          content: 'Resume content',
          file_type: 'pdf'
        }
      })
      .mockResolvedValueOnce({ success: true, data: mockAnalysisResult })

    render(<AnalysisPage />)

    // Wait for initial load
    await waitFor(() => {
      expect(screen.getByText('Senior Software Engineer')).toBeInTheDocument()
    })

    // Upload file and select options
    const dropzone = screen.getByText(/drag & drop your resume/i).closest('[data-testid="dropzone"]')
    if (dropzone) {
      const file = createMockFile('resume.pdf', 'Resume content', 'application/pdf')
      const dataTransfer = createDataTransfer([file])
      fireEvent.drop(dropzone, { dataTransfer })
    }

    await user.selectOptions(screen.getByLabelText(/job description/i), mockJobDescription.id)
    await user.selectOptions(screen.getByLabelText(/ai model/i), mockOllamaModels[0].name)

    // Click analyze button
    await waitFor(() => {
      const analyzeButton = screen.getByText('Analyze Resume')
      expect(analyzeButton).not.toBeDisabled()
    })

    await user.click(screen.getByText('Analyze Resume'))

    // Verify analysis was called
    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith('analyze_resume', expect.objectContaining({
        resume_content: 'Resume content',
        job_description_id: mockJobDescription.id,
        model_name: mockOllamaModels[0].name
      }))
    })

    // Check results are displayed
    await waitFor(() => {
      expect(screen.getByText('Analysis Results')).toBeInTheDocument()
      expect(screen.getByText('85.5')).toBeInTheDocument() // Overall score
      expect(screen.getByText(/strong candidate/i)).toBeInTheDocument()
    })
  })

  it('handles analysis errors gracefully', async () => {
    const user = userEvent.setup()
    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels })
      .mockResolvedValueOnce({ 
        success: true, 
        data: { 
          filename: 'test.pdf', 
          content: 'Resume content',
          file_type: 'pdf'
        }
      })
      .mockResolvedValueOnce({ 
        success: false, 
        error: 'Failed to connect to Ollama service' 
      })

    render(<AnalysisPage />)

    // Setup and trigger analysis
    await waitFor(() => {
      expect(screen.getByText('Senior Software Engineer')).toBeInTheDocument()
    })

    const dropzone = screen.getByText(/drag & drop your resume/i).closest('[data-testid="dropzone"]')
    if (dropzone) {
      const file = createMockFile('resume.pdf', 'Resume content', 'application/pdf')
      const dataTransfer = createDataTransfer([file])
      fireEvent.drop(dropzone, { dataTransfer })
    }

    await user.selectOptions(screen.getByLabelText(/job description/i), mockJobDescription.id)
    await user.selectOptions(screen.getByLabelText(/ai model/i), mockOllamaModels[0].name)

    await waitFor(() => {
      const analyzeButton = screen.getByText('Analyze Resume')
      expect(analyzeButton).not.toBeDisabled()
    })

    await user.click(screen.getByText('Analyze Resume'))

    // Check error message is displayed
    await waitFor(() => {
      expect(screen.getByText(/failed to connect to ollama/i)).toBeInTheDocument()
    })
  })

  it('displays loading state during analysis', async () => {
    const user = userEvent.setup()
    let resolveAnalysis: (value: any) => void
    const analysisPromise = new Promise(resolve => {
      resolveAnalysis = resolve
    })

    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels })
      .mockResolvedValueOnce({ 
        success: true, 
        data: { 
          filename: 'test.pdf', 
          content: 'Resume content',
          file_type: 'pdf'
        }
      })
      .mockImplementationOnce(() => analysisPromise)

    render(<AnalysisPage />)

    // Setup and trigger analysis
    await waitFor(() => {
      expect(screen.getByText('Senior Software Engineer')).toBeInTheDocument()
    })

    const dropzone = screen.getByText(/drag & drop your resume/i).closest('[data-testid="dropzone"]')
    if (dropzone) {
      const file = createMockFile('resume.pdf', 'Resume content', 'application/pdf')
      const dataTransfer = createDataTransfer([file])
      fireEvent.drop(dropzone, { dataTransfer })
    }

    await user.selectOptions(screen.getByLabelText(/job description/i), mockJobDescription.id)
    await user.selectOptions(screen.getByLabelText(/ai model/i), mockOllamaModels[0].name)

    const analyzeButton = await screen.findByText('Analyze Resume')
    await user.click(analyzeButton)

    // Check loading state
    expect(screen.getByText(/analyzing/i)).toBeInTheDocument()
    expect(screen.getByText('Analyze Resume')).toBeDisabled()

    // Resolve the promise
    resolveAnalysis!({ success: true, data: mockAnalysisResult })

    // Wait for loading to finish
    await waitFor(() => {
      expect(screen.queryByText(/analyzing/i)).not.toBeInTheDocument()
    })
  })

  it('handles file parsing errors', async () => {
    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels })
      .mockResolvedValueOnce({ 
        success: false, 
        error: 'Unsupported file type' 
      })

    render(<AnalysisPage />)

    const dropzone = screen.getByText(/drag & drop your resume/i).closest('[data-testid="dropzone"]')
    if (dropzone) {
      const file = createMockFile('resume.xyz', 'Invalid content', 'application/unknown')
      const dataTransfer = createDataTransfer([file])
      fireEvent.drop(dropzone, { dataTransfer })

      await waitFor(() => {
        expect(screen.getByText(/unsupported file type/i)).toBeInTheDocument()
      })
    }
  })
})