import { describe, it, expect } from 'vitest';
import { render, screen } from './utils';
import React from 'react';

// Simple component for testing
const TestComponent: React.FC<{ text: string }> = ({ text }) => {
  return <div data-testid="test-component">{text}</div>;
};

describe('Test Setup', () => {
  it('should render a simple component', () => {
    render(<TestComponent text="Hello World" />);

    expect(screen.getByTestId('test-component')).toBeInTheDocument();
    expect(screen.getByText('Hello World')).toBeInTheDocument();
  });

  it('should have working mock utilities', () => {
    const mockData = {
      overall_score: 85.5,
      category_scores: {
        skills: 90.0,
        experience: 80.0,
      },
    };

    expect(mockData.overall_score).toBe(85.5);
    expect(mockData.category_scores.skills).toBe(90.0);
  });
});
