import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, act, waitFor } from './utils';
import { BulletPointAnalyzer } from '../components/ui/BulletPointAnalyzer';
import { IndustryKeywordManager } from '../components/ui/IndustryKeywordManager';
import { FormatHealthDashboard } from '../components/ui/FormatHealthDashboard';
import { AchievementSuggestions } from '../components/ui/AchievementSuggestions';
import { invoke } from '@tauri-apps/api/tauri';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/tauri');
const mockInvoke = vi.mocked(invoke);

describe('New Components', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockInvoke.mockResolvedValue({ success: false, data: null });
  });

  it('BulletPointAnalyzer renders without crashing with empty text', async () => {
    await act(async () => {
      render(<BulletPointAnalyzer text="" />);
    });
    // Should not crash and not render anything for empty text
    expect(screen.queryByRole('article')).not.toBeInTheDocument();
  });

  it('IndustryKeywordManager renders without crashing with empty content', async () => {
    await act(async () => {
      render(<IndustryKeywordManager resumeContent="" jobDescription="" />);
    });

    expect(screen.getByText('Industry Keyword Analysis')).toBeInTheDocument();
    expect(screen.getByText('Target Industry')).toBeInTheDocument();
    expect(screen.getByDisplayValue('Technology')).toBeInTheDocument();
  });

  it('FormatHealthDashboard renders without crashing with empty content', async () => {
    await act(async () => {
      render(<FormatHealthDashboard resumeContent="" />);
    });

    expect(screen.getByText('Format Health Dashboard')).toBeInTheDocument();
    expect(
      screen.getByText('Upload a resume to analyze format health')
    ).toBeInTheDocument();
  });

  it('AchievementSuggestions renders without crashing with empty content', async () => {
    await act(async () => {
      render(<AchievementSuggestions resumeContent="" />);
    });

    expect(screen.getByText('Achievement Suggestions')).toBeInTheDocument();
    expect(
      screen.getByText(
        'Upload a resume to get contextual achievement suggestions'
      )
    ).toBeInTheDocument();
  });

  it('BulletPointAnalyzer renders with content and shows loading state', async () => {
    vi.useFakeTimers();
    
    // Mock a slow API response to capture loading state
    mockInvoke.mockImplementation(() => 
      new Promise(resolve => setTimeout(() => resolve({ success: true, data: null }), 1000))
    );
    
    await act(async () => {
      render(<BulletPointAnalyzer text="Managed a team of 5 developers" />);
    });

    // Advance timers to trigger the debounced analysis
    await act(async () => {
      vi.advanceTimersByTime(500);
    });

    // Should show loading state after debounce delay
    expect(screen.getByText('Analyzing bullet point...')).toBeInTheDocument();
    
    vi.useRealTimers();
  });

  it('Components handle mock API failures gracefully', async () => {
    // Mock API calls to fail quickly
    mockInvoke.mockRejectedValue(new Error('API Error'));

    await act(async () => {
      render(
        <div>
          <BulletPointAnalyzer text="Some bullet point" />
          <IndustryKeywordManager
            resumeContent="Some content"
            jobDescription="Some job"
          />
          <FormatHealthDashboard resumeContent="Some content" />
          <AchievementSuggestions resumeContent="Some content" />
        </div>
      );
    });

    // Components should not crash even when API calls fail
    expect(screen.getByText('Industry Keyword Analysis')).toBeInTheDocument();
    expect(screen.getByText('Format Health Dashboard')).toBeInTheDocument();
    expect(screen.getByText('Achievement Suggestions')).toBeInTheDocument();
  });
});
