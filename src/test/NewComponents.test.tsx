import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from './utils';
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

  it('BulletPointAnalyzer renders without crashing with empty text', () => {
    render(<BulletPointAnalyzer text="" />);
    // Should not crash and not render anything for empty text
    expect(screen.queryByRole('article')).not.toBeInTheDocument();
  });

  it('IndustryKeywordManager renders without crashing with empty content', () => {
    render(<IndustryKeywordManager resumeContent="" jobDescription="" />);

    expect(screen.getByText('Industry Keyword Analysis')).toBeInTheDocument();
    expect(screen.getByLabelText('Target Industry')).toBeInTheDocument();
  });

  it('FormatHealthDashboard renders without crashing with empty content', () => {
    render(<FormatHealthDashboard resumeContent="" />);

    expect(screen.getByText('Format Health Dashboard')).toBeInTheDocument();
    expect(
      screen.getByText('Upload a resume to analyze format health')
    ).toBeInTheDocument();
  });

  it('AchievementSuggestions renders without crashing with empty content', () => {
    render(<AchievementSuggestions resumeContent="" />);

    expect(screen.getByText('Achievement Suggestions')).toBeInTheDocument();
    expect(
      screen.getByText(
        'Upload a resume to get contextual achievement suggestions'
      )
    ).toBeInTheDocument();
  });

  it('BulletPointAnalyzer renders with content and shows loading state', () => {
    render(<BulletPointAnalyzer text="Managed a team of 5 developers" />);

    // Should show loading state initially
    expect(screen.getByText('Analyzing bullet point...')).toBeInTheDocument();
  });

  it('Components handle mock API failures gracefully', async () => {
    // Mock API calls to fail
    mockInvoke.mockRejectedValue(new Error('API Error'));

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

    // Components should not crash even when API calls fail
    expect(screen.getByText('Industry Keyword Analysis')).toBeInTheDocument();
    expect(screen.getByText('Format Health Dashboard')).toBeInTheDocument();
    expect(screen.getByText('Achievement Suggestions')).toBeInTheDocument();
  });
});
