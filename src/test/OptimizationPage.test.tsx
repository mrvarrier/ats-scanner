import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from './utils';
import userEvent from '@testing-library/user-event';
import { invoke } from '@tauri-apps/api/tauri';
import OptimizationPage from '../pages/OptimizationPage';
import { mockJobDescription, mockOllamaModels } from './utils';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/tauri');
const mockInvoke = vi.mocked(invoke);

const mockOptimizationResult = {
  optimized_content:
    'John Doe\nSenior Software Engineer\n\nEXPERIENCE\n• 5+ years Python development with Django and Flask\n• Extensive React.js frontend development\n• AWS cloud architecture and deployment experience',
  changes_made: [
    {
      section: 'Experience',
      change_type: 'enhancement',
      original: 'Python experience',
      optimized: '5+ years Python development with Django and Flask',
      impact_score: 85.0,
    },
  ],
  before_score: 72.5,
  after_score: 88.2,
  improvement_percentage: 21.7,
};

describe('OptimizationPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the optimization page with initial elements', () => {
    render(<OptimizationPage />);

    expect(screen.getByText('Resume Optimization')).toBeInTheDocument();
    expect(screen.getByText('Original Resume')).toBeInTheDocument();
    expect(screen.getByText('Optimized Resume')).toBeInTheDocument();
    expect(
      screen.getByPlaceholderText(/paste your resume content/i)
    ).toBeInTheDocument();
  });

  it('loads job descriptions and models on mount', async () => {
    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels });

    render(<OptimizationPage />);

    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith('get_job_descriptions');
      expect(mockInvoke).toHaveBeenCalledWith('get_ollama_models');
    });
  });

  it('allows user to input resume content', async () => {
    const user = userEvent.setup();
    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels });

    render(<OptimizationPage />);

    const textarea = screen.getByPlaceholderText(/paste your resume content/i);
    await user.type(textarea, 'John Doe\nSoftware Engineer\nPython experience');

    expect(textarea).toHaveValue(
      'John Doe\nSoftware Engineer\nPython experience'
    );
  });

  it('enables optimization when all requirements are met', async () => {
    const user = userEvent.setup();
    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels });

    render(<OptimizationPage />);

    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('Senior Software Engineer')).toBeInTheDocument();
    });

    // Fill in resume content
    const textarea = screen.getByPlaceholderText(/paste your resume content/i);
    await user.type(textarea, 'John Doe\nSoftware Engineer');

    // Select job description and model
    await user.selectOptions(
      screen.getByLabelText(/job description/i),
      mockJobDescription.id
    );
    await user.selectOptions(
      screen.getByLabelText(/ai model/i),
      mockOllamaModels[0].name
    );

    await waitFor(() => {
      const optimizeButton = screen.getByText('Optimize Resume');
      expect(optimizeButton).not.toBeDisabled();
    });
  });

  it('performs optimization and displays results', async () => {
    const user = userEvent.setup();
    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels })
      .mockResolvedValueOnce({ success: true, data: mockOptimizationResult });

    render(<OptimizationPage />);

    // Wait for initial load
    await waitFor(() => {
      expect(screen.getByText('Senior Software Engineer')).toBeInTheDocument();
    });

    // Fill in form
    const textarea = screen.getByPlaceholderText(/paste your resume content/i);
    await user.type(textarea, 'John Doe\nSoftware Engineer\nPython experience');

    await user.selectOptions(
      screen.getByLabelText(/job description/i),
      mockJobDescription.id
    );
    await user.selectOptions(
      screen.getByLabelText(/ai model/i),
      mockOllamaModels[0].name
    );

    // Click optimize button
    await waitFor(() => {
      const optimizeButton = screen.getByText('Optimize Resume');
      expect(optimizeButton).not.toBeDisabled();
    });

    await user.click(screen.getByText('Optimize Resume'));

    // Verify optimization was called
    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith(
        'optimize_resume',
        expect.objectContaining({
          resume_content: 'John Doe\nSoftware Engineer\nPython experience',
          job_description_id: mockJobDescription.id,
          model_name: mockOllamaModels[0].name,
          optimization_level: 'moderate',
        })
      );
    });

    // Check optimized content is displayed
    await waitFor(() => {
      const optimizedTextarea = screen.getByDisplayValue(
        /5\+ years Python development/i
      );
      expect(optimizedTextarea).toBeInTheDocument();
    });
  });

  it('displays improvement metrics after optimization', async () => {
    const user = userEvent.setup();
    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels })
      .mockResolvedValueOnce({ success: true, data: mockOptimizationResult });

    render(<OptimizationPage />);

    // Setup and trigger optimization
    await waitFor(() => {
      expect(screen.getByText('Senior Software Engineer')).toBeInTheDocument();
    });

    const textarea = screen.getByPlaceholderText(/paste your resume content/i);
    await user.type(textarea, 'John Doe\nSoftware Engineer\nPython experience');

    await user.selectOptions(
      screen.getByLabelText(/job description/i),
      mockJobDescription.id
    );
    await user.selectOptions(
      screen.getByLabelText(/ai model/i),
      mockOllamaModels[0].name
    );

    const optimizeButton = await screen.findByText('Optimize Resume');
    await user.click(optimizeButton);

    // Check improvement metrics
    await waitFor(() => {
      expect(screen.getByText(/21\.7%/)).toBeInTheDocument(); // Improvement percentage
      expect(screen.getByText(/72\.5/)).toBeInTheDocument(); // Before score
      expect(screen.getByText(/88\.2/)).toBeInTheDocument(); // After score
    });
  });

  it('shows changes made during optimization', async () => {
    const user = userEvent.setup();
    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels })
      .mockResolvedValueOnce({ success: true, data: mockOptimizationResult });

    render(<OptimizationPage />);

    // Setup and trigger optimization
    await waitFor(() => {
      expect(screen.getByText('Senior Software Engineer')).toBeInTheDocument();
    });

    const textarea = screen.getByPlaceholderText(/paste your resume content/i);
    await user.type(textarea, 'John Doe\nSoftware Engineer');

    await user.selectOptions(
      screen.getByLabelText(/job description/i),
      mockJobDescription.id
    );
    await user.selectOptions(
      screen.getByLabelText(/ai model/i),
      mockOllamaModels[0].name
    );

    const optimizeButton = await screen.findByText('Optimize Resume');
    await user.click(optimizeButton);

    // Check changes are displayed
    await waitFor(() => {
      expect(screen.getByText('Changes Made')).toBeInTheDocument();
      expect(screen.getByText('Experience')).toBeInTheDocument();
      expect(screen.getByText('enhancement')).toBeInTheDocument();
    });
  });

  it('allows users to change optimization level', async () => {
    const user = userEvent.setup();
    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels });

    render(<OptimizationPage />);

    // Find optimization level select
    const optimizationSelect = screen.getByLabelText(/optimization level/i);

    // Change to aggressive
    await user.selectOptions(optimizationSelect, 'aggressive');
    expect(screen.getByText('Aggressive')).toBeInTheDocument();

    // Change to conservative
    await user.selectOptions(optimizationSelect, 'conservative');
    expect(screen.getByText('Conservative')).toBeInTheDocument();
  });

  it('provides real-time analysis as user types', async () => {
    const user = userEvent.setup();
    vi.useFakeTimers();

    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels })
      .mockResolvedValueOnce({
        success: true,
        data: { overall_score: 75.0, category_scores: { skills: 80 } },
      });

    render(<OptimizationPage />);

    const textarea = screen.getByPlaceholderText(/paste your resume content/i);
    await user.type(textarea, 'John Doe\nSoftware Engineer');

    // Fast-forward timers to trigger debounced analysis
    vi.advanceTimersByTime(1000);

    await waitFor(() => {
      expect(screen.getByText(/current score/i)).toBeInTheDocument();
    });

    vi.useRealTimers();
  });

  it('handles optimization errors gracefully', async () => {
    const user = userEvent.setup();
    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels })
      .mockResolvedValueOnce({
        success: false,
        error: 'Model not available',
      });

    render(<OptimizationPage />);

    // Setup and trigger optimization
    await waitFor(() => {
      expect(screen.getByText('Senior Software Engineer')).toBeInTheDocument();
    });

    const textarea = screen.getByPlaceholderText(/paste your resume content/i);
    await user.type(textarea, 'John Doe\nSoftware Engineer');

    await user.selectOptions(
      screen.getByLabelText(/job description/i),
      mockJobDescription.id
    );
    await user.selectOptions(
      screen.getByLabelText(/ai model/i),
      mockOllamaModels[0].name
    );

    const optimizeButton = await screen.findByText('Optimize Resume');
    await user.click(optimizeButton);

    // Check error message
    await waitFor(() => {
      expect(screen.getByText(/model not available/i)).toBeInTheDocument();
    });
  });

  it('allows copying optimized content', async () => {
    const user = userEvent.setup();
    // Mock clipboard API
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
    });

    mockInvoke
      .mockResolvedValueOnce({ success: true, data: [mockJobDescription] })
      .mockResolvedValueOnce({ success: true, data: mockOllamaModels })
      .mockResolvedValueOnce({ success: true, data: mockOptimizationResult });

    render(<OptimizationPage />);

    // Setup and trigger optimization
    await waitFor(() => {
      expect(screen.getByText('Senior Software Engineer')).toBeInTheDocument();
    });

    const textarea = screen.getByPlaceholderText(/paste your resume content/i);
    await user.type(textarea, 'John Doe\nSoftware Engineer');

    await user.selectOptions(
      screen.getByLabelText(/job description/i),
      mockJobDescription.id
    );
    await user.selectOptions(
      screen.getByLabelText(/ai model/i),
      mockOllamaModels[0].name
    );

    const optimizeButton = await screen.findByText('Optimize Resume');
    await user.click(optimizeButton);

    // Wait for optimization to complete
    await waitFor(() => {
      expect(
        screen.getByDisplayValue(/5\+ years Python development/i)
      ).toBeInTheDocument();
    });

    // Click copy button
    const copyButton = screen.getByText(/copy optimized/i);
    await user.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith(
      mockOptimizationResult.optimized_content
    );
  });
});
