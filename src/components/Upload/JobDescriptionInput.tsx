import React, { useState } from 'react';

interface JobDescriptionInputProps {
  value: string;
  onChange: (value: string) => void;
  jobTitle: string;
  onJobTitleChange: (title: string) => void;
  company: string;
  onCompanyChange: (company: string) => void;
}

const JobDescriptionInput: React.FC<JobDescriptionInputProps> = ({
  value,
  onChange,
  jobTitle,
  onJobTitleChange,
  company,
  onCompanyChange
}) => {
  const [charCount, setCharCount] = useState(value.length);

  const handleDescriptionChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const newValue = e.target.value;
    onChange(newValue);
    setCharCount(newValue.length);
  };

  return (
    <div className="card space-y-4">
      <h3 className="text-lg font-medium text-gray-900">Job Details</h3>
      
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label htmlFor="jobTitle" className="block text-sm font-medium text-gray-700 mb-1">
            Job Title <span className="text-gray-400">(optional)</span>
          </label>
          <input
            type="text"
            id="jobTitle"
            value={jobTitle}
            onChange={(e) => onJobTitleChange(e.target.value)}
            placeholder="e.g., Senior Software Engineer"
            className="input-field"
          />
        </div>
        
        <div>
          <label htmlFor="company" className="block text-sm font-medium text-gray-700 mb-1">
            Company <span className="text-gray-400">(optional)</span>
          </label>
          <input
            type="text"
            id="company"
            value={company}
            onChange={(e) => onCompanyChange(e.target.value)}
            placeholder="e.g., Tech Corp Inc."
            className="input-field"
          />
        </div>
      </div>
      
      <div>
        <label htmlFor="jobDescription" className="block text-sm font-medium text-gray-700 mb-1">
          Job Description *
        </label>
        <textarea
          id="jobDescription"
          value={value}
          onChange={handleDescriptionChange}
          placeholder="Paste the job description here. Include requirements, responsibilities, qualifications, and any other relevant details..."
          className="input-field min-h-[200px] resize-y"
          rows={8}
        />
        
        <div className="flex justify-between items-center mt-2">
          <p className="text-sm text-gray-500">
            Characters: {charCount.toLocaleString()}
          </p>
          
          {charCount < 100 && (
            <p className="text-sm text-warning">
              Add more details for better analysis
            </p>
          )}
          
          {charCount > 10000 && (
            <p className="text-sm text-warning">
              Very long descriptions may affect processing speed
            </p>
          )}
        </div>
      </div>
      
      <div className="bg-blue-50 rounded-lg p-4">
        <h4 className="font-medium text-blue-900 mb-2">💡 Tips for better analysis:</h4>
        <ul className="text-sm text-blue-800 space-y-1">
          <li>• Include specific skills and technologies mentioned</li>
          <li>• Copy requirements and qualifications sections</li>
          <li>• Include experience level and education requirements</li>
          <li>• Add any industry-specific terminology used</li>
        </ul>
      </div>
    </div>
  );
};

export default JobDescriptionInput;