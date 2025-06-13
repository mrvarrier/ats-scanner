import React from 'react';
import { getScoreColor, getMatchLevel } from '../../utils/helpers';

interface ScoreCircleProps {
  score: number;
  size?: 'sm' | 'md' | 'lg';
  showLabel?: boolean;
}

const ScoreCircle: React.FC<ScoreCircleProps> = ({ score, size = 'md', showLabel = true }) => {
  const sizeClasses = {
    sm: 'w-16 h-16 text-xs',
    md: 'w-24 h-24 text-lg',
    lg: 'w-32 h-32 text-2xl'
  };

  const strokeWidth = size === 'lg' ? 8 : size === 'md' ? 6 : 4;
  const radius = size === 'lg' ? 58 : size === 'md' ? 42 : 26;
  const circumference = 2 * Math.PI * radius;
  const strokeDasharray = circumference;
  const strokeDashoffset = circumference - (score / 100) * circumference;

  return (
    <div className="flex flex-col items-center">
      <div className={`relative ${sizeClasses[size]} flex items-center justify-center`}>
        <svg className="transform -rotate-90 w-full h-full">
          <circle
            cx="50%"
            cy="50%"
            r={radius}
            stroke="currentColor"
            className="text-gray-200"
            strokeWidth={strokeWidth}
            fill="transparent"
          />
          <circle
            cx="50%"
            cy="50%"
            r={radius}
            stroke="currentColor"
            className={getScoreColor(score)}
            strokeWidth={strokeWidth}
            fill="transparent"
            strokeDasharray={strokeDasharray}
            strokeDashoffset={strokeDashoffset}
            strokeLinecap="round"
            style={{
              transition: 'stroke-dashoffset 0.5s ease-in-out',
            }}
          />
        </svg>
        <div className={`absolute inset-0 flex items-center justify-center ${sizeClasses[size]} font-bold ${getScoreColor(score)}`}>
          {score}%
        </div>
      </div>
      {showLabel && (
        <div className="mt-2 text-center">
          <p className={`font-medium ${getScoreColor(score)}`}>
            {getMatchLevel(score)} Match
          </p>
        </div>
      )}
    </div>
  );
};

export default ScoreCircle;