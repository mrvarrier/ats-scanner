import React from 'react';
import { render, RenderOptions } from '@testing-library/react';
import { vi } from 'vitest';
import { AllTheProviders } from './test-providers';
// Note: Commenting out router import since we don't have it installed yet
// import { BrowserRouter } from 'react-router-dom'

// Mock data for testing
export const mockAnalysisResult = {
  overall_score: 85.5,
  category_scores: {
    skills: 90.0,
    experience: 80.0,
    education: 85.0,
    keywords: 88.0,
    format: 92.0,
  },
  detailed_feedback:
    'Strong candidate with relevant experience. Good technical skills alignment.',
  missing_keywords: ['Docker', 'Kubernetes'],
  recommendations: [
    'Add more details about project leadership experience',
    'Include specific metrics and achievements',
    'Consider adding Docker and Kubernetes experience',
  ],
  processing_time_ms: 2500,
};

export const mockJobDescription = {
  id: 'job-123',
  title: 'Senior Software Engineer',
  company: 'Tech Corp',
  content:
    'We are looking for a Senior Software Engineer with 5+ years of experience in Python, React, and cloud technologies.',
  requirements: 'Python, React, AWS, 5+ years experience',
  created_at: '2024-01-15T10:00:00Z',
  updated_at: '2024-01-15T10:00:00Z',
};

export const mockResume = {
  id: 'resume-123',
  filename: 'john_doe_resume.pdf',
  content:
    'John Doe\nSoftware Engineer\n5 years Python experience\nReact development\nAWS certified',
  file_type: 'pdf',
  created_at: '2024-01-15T09:00:00Z',
  updated_at: '2024-01-15T09:00:00Z',
};

export const mockAnalysis = {
  id: 'analysis-123',
  resume_id: 'resume-123',
  job_description_id: 'job-123',
  model_used: 'llama2',
  overall_score: 85.5,
  skills_score: 90.0,
  experience_score: 80.0,
  education_score: 85.0,
  keywords_score: 88.0,
  format_score: 92.0,
  detailed_feedback: 'Strong candidate with relevant experience',
  missing_keywords: 'Docker,Kubernetes',
  recommendations: 'Add more cloud experience',
  processing_time_ms: 2500,
  created_at: '2024-01-15T11:00:00Z',
};

export const mockOllamaModels = [
  { name: 'llama2', size: '7B' },
  { name: 'codellama', size: '13B' },
  { name: 'mistral', size: '7B' },
];

const customRender = (
  ui: React.ReactElement,
  options?: Omit<RenderOptions, 'wrapper'>
) => render(ui, { wrapper: AllTheProviders, ...options });

export {
  screen,
  fireEvent,
  waitFor,
  within,
  getByRole,
  getByText,
  getByLabelText,
  getByPlaceholderText,
  queryByRole,
  queryByText,
  findByRole,
  findByText,
  act,
  cleanup,
  renderHook,
  waitForElementToBeRemoved,
} from '@testing-library/react';
export { customRender as render };

// Enhanced helpers are declared above with export const

// Helper to create a file for testing
export const createMockFile = (
  name: string,
  content: string,
  type: string = 'text/plain'
) => {
  const blob = new Blob([content], { type });
  const file = new File([blob], name, { type });
  return file;
};

// Helper to simulate drag and drop
export const createDataTransfer = (files: File[]) => {
  const dt = new DataTransfer();
  files.forEach(file => dt.items.add(file));
  return dt;
};

// Enhanced helper to render components with proper act() wrapping
export const renderWithAct = async (
  ui: React.ReactElement,
  options?: Omit<RenderOptions, 'wrapper'>
) => {
  let renderResult: ReturnType<typeof render>;
  await act(async () => {
    renderResult = render(ui, { wrapper: AllTheProviders, ...options });
  });
  return renderResult!;
};

// Helper to advance timers with proper act() wrapping
export const advanceTimersWithAct = async (ms: number) => {
  await act(async () => {
    vi.advanceTimersByTime(ms);
  });
};

// Helper to wait for async state updates with act()
export const waitForAsyncState = async (callback: () => void) => {
  await act(async () => {
    await new Promise((resolve) => setTimeout(resolve, 0));
  });
  await waitFor(callback);
};
