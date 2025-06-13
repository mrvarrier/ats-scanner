import React, { useState } from 'react';
import { Resume } from '../../types';

interface ResumeParsingDisplayProps {
  resume: Resume;
}

const ResumeParsingDisplay: React.FC<ResumeParsingDisplayProps> = ({ resume }) => {
  const [activeSection, setActiveSection] = useState<'parsed' | 'raw'>('parsed');

  // Function to extract contact information from resume text
  const extractContactInfo = (text: string) => {
    // Improved email regex
    const emailRegex = /\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b/g;
    
    // Improved phone regex - matches various formats
    const phoneRegex = /(\+?1[-.\s]?)?\(?([0-9]{3})\)?[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})\b/g;
    
    // Improved LinkedIn regex
    const linkedinRegex = /(linkedin\.com\/in\/[A-Za-z0-9._-]+|in\.linkedin\.com\/[A-Za-z0-9._-]+)/gi;
    
    // Location regex - more specific patterns
    const locationRegex = /\b([A-Z][a-z]+,\s*[A-Z]{2}|[A-Z][a-z]+,\s*[A-Z][a-z]+|\b(Remote|Work From Home)\b)/g;
    
    const emails = text.match(emailRegex) || [];
    const phones = text.match(phoneRegex) || [];
    const linkedins = text.match(linkedinRegex) || [];
    const locations = text.match(locationRegex) || [];
    
    // Filter out false positive locations
    const filteredLocations = locations.filter(loc => 
      !loc.toLowerCase().includes('university') &&
      !loc.toLowerCase().includes('college') &&
      !loc.toLowerCase().includes('school') &&
      !loc.toLowerCase().includes('institute')
    );
    
    return { 
      emails, 
      phones: phones.map(p => p.trim()), 
      linkedins,
      locations: filteredLocations
    };
  };

  // Function to extract sections from resume text
  const extractSections = (text: string) => {
    const sections: { [key: string]: string } = {};
    
    // Common section headers
    const sectionHeaders = [
      'contact information',
      'summary',
      'professional summary',
      'objective',
      'skills',
      'technical skills',
      'core competencies',
      'experience',
      'work experience',
      'professional experience',
      'employment',
      'education',
      'certifications',
      'projects',
      'achievements',
      'awards'
    ];
    
    const lines = text.split('\n');
    let currentSection = 'header';
    let currentContent = [];
    
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i].trim();
      if (!line) continue;
      
      // Check if this line is a section header
      const isHeader = sectionHeaders.some(header => 
        line.toLowerCase().includes(header.toLowerCase()) && 
        line.length < 50 && 
        !line.includes('@') && 
        !line.includes('(') &&
        (line.toUpperCase() === line || line.toLowerCase() === line || 
         line.charAt(0).toUpperCase() + line.slice(1).toLowerCase() === line)
      );
      
      if (isHeader) {
        // Save previous section
        if (currentContent.length > 0) {
          sections[currentSection] = currentContent.join('\n');
        }
        
        // Start new section
        currentSection = line.toLowerCase().replace(/[^a-z0-9]/g, '_');
        currentContent = [];
      } else {
        currentContent.push(line);
      }
    }
    
    // Save last section
    if (currentContent.length > 0) {
      sections[currentSection] = currentContent.join('\n');
    }
    
    return sections;
  };

  // Function to extract dates and calculate experience
  const extractExperience = (text: string) => {
    const dateRegex = /\b(19|20)\d{2}\b/g;
    const dates = text.match(dateRegex) || [];
    const years = dates.map(d => parseInt(d)).sort();
    
    // Extract and parse date ranges
    const dateRanges = extractDateRanges(text);
    const totalMonths = calculateTotalExperience(dateRanges);
    
    return {
      dates: dates,
      dateRanges: dateRanges,
      estimatedYears: Math.floor(totalMonths / 12),
      estimatedMonths: totalMonths % 12,
      totalMonths: totalMonths,
      startYear: years[0] || null,
      endYear: years[years.length - 1] || null
    };
  };

  // Function to extract date ranges from work experience
  const extractDateRanges = (text: string) => {
    const ranges = [];
    const lines = text.split('\n');
    
    // Common date range patterns
    const patterns = [
      // Jan 2023 - Dec 2024, January 2023 - December 2024
      /\b(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec|January|February|March|April|May|June|July|August|September|October|November|December)\s+(\d{4})\s*[-–—]\s*(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec|January|February|March|April|May|June|July|August|September|October|November|December)\s+(\d{4})\b/gi,
      // Jan 2023 - Present, January 2023 - Current
      /\b(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec|January|February|March|April|May|June|July|August|September|October|November|December)\s+(\d{4})\s*[-–—]\s*(Present|Current|Now)\b/gi,
      // 01/2023 - 12/2024, 2023 - 2024
      /\b(\d{1,2})\/(\d{4})\s*[-–—]\s*(\d{1,2})\/(\d{4})\b/g,
      /\b(\d{4})\s*[-–—]\s*(\d{4})\b/g,
      // 2023 - Present
      /\b(\d{4})\s*[-–—]\s*(Present|Current|Now)\b/gi
    ];
    
    for (const line of lines) {
      for (const pattern of patterns) {
        const matches = [...line.matchAll(pattern)];
        for (const match of matches) {
          try {
            const range = parseDateRange(match);
            if (range) {
              ranges.push(range);
            }
          } catch (e) {
            // Skip invalid date ranges
          }
        }
      }
    }
    
    return ranges;
  };

  // Function to parse individual date range match
  const parseDateRange = (match: RegExpMatchArray) => {
    const fullMatch = match[0];
    
    // Handle different patterns
    if (fullMatch.includes('Present') || fullMatch.includes('Current') || fullMatch.includes('Now')) {
      // Extract start date and use current date as end
      const startPart = fullMatch.split(/[-–—]/)[0].trim();
      const startDate = parseDate(startPart);
      const endDate = new Date();
      
      return {
        start: startDate,
        end: endDate,
        text: fullMatch
      };
    } else {
      // Extract both start and end dates
      const parts = fullMatch.split(/[-–—]/);
      if (parts.length === 2) {
        const startDate = parseDate(parts[0].trim());
        const endDate = parseDate(parts[1].trim());
        
        return {
          start: startDate,
          end: endDate,
          text: fullMatch
        };
      }
    }
    
    return null;
  };

  // Function to parse various date formats
  const parseDate = (dateStr: string) => {
    const monthNames = {
      'jan': 0, 'january': 0, 'feb': 1, 'february': 1, 'mar': 2, 'march': 2,
      'apr': 3, 'april': 3, 'may': 4, 'jun': 5, 'june': 5, 'jul': 6, 'july': 6,
      'aug': 7, 'august': 7, 'sep': 8, 'september': 8, 'oct': 9, 'october': 9,
      'nov': 10, 'november': 10, 'dec': 11, 'december': 11
    };
    
    // Month Year format (e.g., "Jan 2023", "January 2023")
    const monthYearMatch = dateStr.match(/\b(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec|January|February|March|April|May|June|July|August|September|October|November|December)\s+(\d{4})\b/i);
    if (monthYearMatch) {
      const month = monthNames[monthYearMatch[1].toLowerCase()];
      const year = parseInt(monthYearMatch[2]);
      return new Date(year, month, 1);
    }
    
    // MM/YYYY format
    const mmYyyyMatch = dateStr.match(/\b(\d{1,2})\/(\d{4})\b/);
    if (mmYyyyMatch) {
      const month = parseInt(mmYyyyMatch[1]) - 1; // JavaScript months are 0-indexed
      const year = parseInt(mmYyyyMatch[2]);
      return new Date(year, month, 1);
    }
    
    // Year only format
    const yearMatch = dateStr.match(/\b(\d{4})\b/);
    if (yearMatch) {
      const year = parseInt(yearMatch[1]);
      return new Date(year, 0, 1); // January 1st of that year
    }
    
    return null;
  };

  // Function to calculate total experience in months
  const calculateTotalExperience = (ranges: any[]) => {
    if (ranges.length === 0) return 0;
    
    // Sort ranges by start date
    const sortedRanges = ranges
      .filter(r => r.start && r.end)
      .sort((a, b) => a.start.getTime() - b.start.getTime());
    
    if (sortedRanges.length === 0) return 0;
    
    // Merge overlapping ranges and calculate total months
    const mergedRanges = [];
    let currentRange = { ...sortedRanges[0] };
    
    for (let i = 1; i < sortedRanges.length; i++) {
      const range = sortedRanges[i];
      
      // If ranges overlap or are adjacent, merge them
      if (range.start <= currentRange.end) {
        currentRange.end = new Date(Math.max(currentRange.end.getTime(), range.end.getTime()));
      } else {
        mergedRanges.push(currentRange);
        currentRange = { ...range };
      }
    }
    mergedRanges.push(currentRange);
    
    // Calculate total months across all ranges
    let totalMonths = 0;
    for (const range of mergedRanges) {
      const months = (range.end.getFullYear() - range.start.getFullYear()) * 12 + 
                    (range.end.getMonth() - range.start.getMonth());
      totalMonths += Math.max(0, months);
    }
    
    return totalMonths;
  };

  // Function to extract and format work experience entries
  const extractFormattedExperience = (text: string) => {
    const lines = text.split('\n');
    const experiences = [];
    let currentJob = null;
    let collectingJob = false;
    
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i].trim();
      if (!line) continue;
      
      // Look for potential job titles (lines with common job keywords)
      const jobKeywords = ['Engineer', 'Manager', 'Developer', 'Analyst', 'Director', 'Lead', 'Senior', 'Junior', 'Specialist', 'Coordinator', 'Associate', 'Consultant', 'Administrator', 'Supervisor', 'Executive', 'Officer', 'Representative', 'Technician', 'Designer', 'Architect', 'Intern'];
      const hasJobKeyword = jobKeywords.some(keyword => line.includes(keyword));
      
      // Look for company patterns (often followed by location or dates)
      const nextLine = i + 1 < lines.length ? lines[i + 1].trim() : '';
      const lineAfter = i + 2 < lines.length ? lines[i + 2].trim() : '';
      
      // Check if this might be a job title
      if (hasJobKeyword && line.length > 5 && line.length < 100 && !line.includes('@')) {
        // Save previous job if exists
        if (currentJob) {
          experiences.push(currentJob);
        }
        
        // Start new job
        currentJob = {
          title: line,
          company: '',
          duration: '',
          location: '',
          description: []
        };
        collectingJob = true;
        
        // Look for company in next lines
        if (nextLine && !nextLine.match(/\b(19|20)\d{2}\b/) && nextLine.length < 100) {
          currentJob.company = nextLine;
          i++; // Skip the company line
          
          // Check if line after company has dates
          if (lineAfter && lineAfter.match(/\b(19|20)\d{2}\b/)) {
            currentJob.duration = lineAfter;
            i++; // Skip the duration line
          }
        }
      }
      // Look for date ranges that might be duration
      else if (line.match(/\b(19|20)\d{2}\b/) && currentJob && !currentJob.duration) {
        currentJob.duration = line;
      }
      // Look for location indicators (more specific)
      else if (currentJob && !currentJob.location && !line.match(/\b(19|20)\d{2}\b/) && line.length < 50) {
        // Check for common location patterns
        const locationPatterns = [
          /\b(Remote|Work From Home|WFH)\b/i,
          /\b[A-Z][a-z]+,\s*[A-Z]{2}\b/, // City, ST format
          /\b[A-Z][a-z]+,\s*[A-Z][a-z]+\b/, // City, State format
          /\b(San Francisco|Los Angeles|New York|Chicago|Boston|Seattle|Austin|Denver|Miami|Atlanta|Dallas|Houston|Phoenix|Portland|Washington DC|Washington D\.C\.)\b/i
        ];
        
        const isLocation = locationPatterns.some(pattern => pattern.test(line)) && 
                          !line.toLowerCase().includes('university') && 
                          !line.toLowerCase().includes('college') &&
                          !line.toLowerCase().includes('school') &&
                          !hasJobKeyword;
        
        if (isLocation) {
          currentJob.location = line;
        }
      }
      // Collect job description bullets
      else if (collectingJob && currentJob && line.length > 10) {
        // Check if this might be the start of a new job
        const looksLikeNewJob = hasJobKeyword && !line.startsWith('•') && !line.startsWith('-') && !line.startsWith('*');
        if (looksLikeNewJob) {
          // This might be a new job, step back
          i--;
          collectingJob = false;
        } else {
          currentJob.description.push(line);
        }
      }
    }
    
    // Add the last job
    if (currentJob) {
      experiences.push(currentJob);
    }
    
    return experiences;
  };

  // Function to extract potential job titles
  const extractJobTitles = (text: string) => {
    const lines = text.split('\n');
    const titles = [];
    
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i].trim();
      
      // Look for lines that might be job titles
      if (line.length > 5 && line.length < 100 && 
          !line.includes('@') && 
          !line.includes('(') &&
          !line.toLowerCase().includes('university') &&
          !line.toLowerCase().includes('school') &&
          (line.includes('Manager') || line.includes('Engineer') || 
           line.includes('Developer') || line.includes('Analyst') ||
           line.includes('Director') || line.includes('Lead') ||
           line.includes('Senior') || line.includes('Junior'))) {
        titles.push(line);
      }
    }
    
    return titles;
  };

  const contactInfo = extractContactInfo(resume.extracted_text);
  const sections = extractSections(resume.extracted_text);
  const experience = extractExperience(resume.extracted_text);
  const formattedExperience = extractFormattedExperience(resume.extracted_text);
  const jobTitles = extractJobTitles(resume.extracted_text);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h2 className="text-xl font-bold text-gray-900 mb-2">Resume Parsing Analysis</h2>
        <p className="text-gray-600">
          See exactly how your resume was parsed and what information was extracted.
        </p>
      </div>

      {/* Tab Navigation */}
      <div className="border-b border-gray-200">
        <nav className="-mb-px flex space-x-8">
          <button
            onClick={() => setActiveSection('parsed')}
            className={`py-2 px-1 border-b-2 font-medium text-sm ${
              activeSection === 'parsed'
                ? 'border-primary text-primary'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            Parsed Sections
          </button>
          <button
            onClick={() => setActiveSection('raw')}
            className={`py-2 px-1 border-b-2 font-medium text-sm ${
              activeSection === 'raw'
                ? 'border-primary text-primary'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            Raw Text
          </button>
        </nav>
      </div>

      {activeSection === 'parsed' ? (
        <div className="space-y-6">
          {/* File Information */}
          <div className="card">
            <h3 className="text-lg font-medium text-gray-900 mb-4">File Information</h3>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div>
                <span className="text-gray-600 text-sm">Original Name:</span>
                <p className="font-medium">{resume.original_name}</p>
              </div>
              <div>
                <span className="text-gray-600 text-sm">File Size:</span>
                <p className="font-medium">{Math.round(resume.file_size / 1024)} KB</p>
              </div>
              <div>
                <span className="text-gray-600 text-sm">Upload Date:</span>
                <p className="font-medium">{new Date(resume.upload_date).toLocaleDateString()}</p>
              </div>
              <div>
                <span className="text-gray-600 text-sm">Text Length:</span>
                <p className="font-medium">{resume.extracted_text.length} characters</p>
              </div>
            </div>
          </div>

          {/* Contact Information */}
          <div className="card">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Extracted Contact Information</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
              <div>
                <h4 className="font-medium text-gray-900 mb-2">Email Addresses</h4>
                {contactInfo.emails.length > 0 ? (
                  <ul className="space-y-1">
                    {contactInfo.emails.map((email, index) => (
                      <li key={index} className="text-sm bg-green-50 text-green-800 px-2 py-1 rounded">
                        {email}
                      </li>
                    ))}
                  </ul>
                ) : (
                  <p className="text-sm text-red-600">No email found</p>
                )}
              </div>
              <div>
                <h4 className="font-medium text-gray-900 mb-2">Phone Numbers</h4>
                {contactInfo.phones.length > 0 ? (
                  <ul className="space-y-1">
                    {contactInfo.phones.map((phone, index) => (
                      <li key={index} className="text-sm bg-green-50 text-green-800 px-2 py-1 rounded">
                        {phone}
                      </li>
                    ))}
                  </ul>
                ) : (
                  <p className="text-sm text-red-600">No phone found</p>
                )}
              </div>
              <div>
                <h4 className="font-medium text-gray-900 mb-2">LinkedIn</h4>
                {contactInfo.linkedins.length > 0 ? (
                  <ul className="space-y-1">
                    {contactInfo.linkedins.map((linkedin, index) => (
                      <li key={index} className="text-sm bg-green-50 text-green-800 px-2 py-1 rounded">
                        {linkedin}
                      </li>
                    ))}
                  </ul>
                ) : (
                  <p className="text-sm text-red-600">No LinkedIn found</p>
                )}
              </div>
              <div>
                <h4 className="font-medium text-gray-900 mb-2">Location</h4>
                {contactInfo.locations.length > 0 ? (
                  <ul className="space-y-1">
                    {contactInfo.locations.map((location, index) => (
                      <li key={index} className="text-sm bg-green-50 text-green-800 px-2 py-1 rounded">
                        {location}
                      </li>
                    ))}
                  </ul>
                ) : (
                  <p className="text-sm text-red-600">No location found</p>
                )}
              </div>
            </div>
          </div>

          {/* Work Experience Breakdown */}
          <div className="card">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Work Experience Breakdown</h3>
            
            {/* Summary Stats */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6 p-4 bg-gray-50 rounded-lg">
              <div>
                <span className="text-gray-600 text-sm">Total Jobs Found:</span>
                <p className="font-medium">{formattedExperience.length}</p>
              </div>
              <div>
                <span className="text-gray-600 text-sm">Date Ranges Found:</span>
                <p className="font-medium">{experience.dateRanges?.length || 0}</p>
              </div>
              <div>
                <span className="text-gray-600 text-sm">Total Experience:</span>
                <p className="font-medium">
                  {experience.estimatedYears > 0 || experience.estimatedMonths > 0 
                    ? `${experience.estimatedYears} year${experience.estimatedYears !== 1 ? 's' : ''} ${experience.estimatedMonths} month${experience.estimatedMonths !== 1 ? 's' : ''}`
                    : 'Not calculated'}
                </p>
              </div>
              <div>
                <span className="text-gray-600 text-sm">Date Range:</span>
                <p className="font-medium">
                  {experience.startYear && experience.endYear 
                    ? `${experience.startYear} - ${experience.endYear}` 
                    : 'Not found'}
                </p>
              </div>
            </div>

            {/* Date Ranges Detected */}
            {experience.dateRanges && experience.dateRanges.length > 0 && (
              <div className="mb-6 p-4 bg-blue-50 rounded-lg">
                <h4 className="font-medium text-blue-900 mb-2">Detected Employment Periods:</h4>
                <div className="space-y-2">
                  {experience.dateRanges.map((range: any, index: number) => (
                    <div key={index} className="flex items-center justify-between text-sm">
                      <span className="text-blue-800 font-mono">{range.text}</span>
                      <span className="text-blue-600">
                        {range.start && range.end ? `${Math.round(((range.end.getTime() - range.start.getTime()) / (1000 * 60 * 60 * 24 * 30.44)))} months` : 'Invalid range'}
                      </span>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* Individual Job Entries */}
            <div className="space-y-4">
              {formattedExperience.length > 0 ? (
                formattedExperience.map((job, index) => (
                  <div key={index} className="border border-gray-200 rounded-lg p-4 bg-white">
                    <div className="mb-3">
                      <h4 className="text-lg font-semibold text-gray-900">{job.title}</h4>
                      {job.company && (
                        <p className="text-md text-gray-700 font-medium">{job.company}</p>
                      )}
                      <div className="flex flex-wrap gap-4 mt-2 text-sm text-gray-600">
                        {job.duration && (
                          <span className="flex items-center">
                            <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                            </svg>
                            {job.duration}
                          </span>
                        )}
                        {job.location && (
                          <span className="flex items-center">
                            <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
                            </svg>
                            {job.location}
                          </span>
                        )}
                      </div>
                    </div>
                    
                    {job.description.length > 0 && (
                      <div>
                        <h5 className="text-sm font-medium text-gray-900 mb-2">Key Responsibilities & Achievements:</h5>
                        <ul className="space-y-1">
                          {job.description.map((desc, descIndex) => (
                            <li key={descIndex} className="text-sm text-gray-700 flex items-start">
                              <span className="text-gray-400 mr-2 mt-1">•</span>
                              <span>{desc}</span>
                            </li>
                          ))}
                        </ul>
                      </div>
                    )}
                  </div>
                ))
              ) : (
                <div className="text-center py-8 text-gray-500">
                  <svg className="w-12 h-12 mx-auto mb-4 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2-2v2m8 0V6a2 2 0 012 2v6a2 2 0 01-2 2H8a2 2 0 01-2-2V8a2 2 0 012-2h8zM8 14v.01M12 14v.01M16 14v.01" />
                  </svg>
                  <p className="text-lg font-medium">No work experience detected</p>
                  <p className="text-sm">The resume parser couldn't identify clear job entries. Check the raw text or parsed sections to see how the experience information appears.</p>
                </div>
              )}
            </div>
          </div>

          {/* Job Titles */}
          <div className="card">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Potential Job Titles Found</h3>
            {jobTitles.length > 0 ? (
              <div className="flex flex-wrap gap-2">
                {jobTitles.map((title, index) => (
                  <span key={index} className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm">
                    {title}
                  </span>
                ))}
              </div>
            ) : (
              <p className="text-gray-500 italic">No job titles detected</p>
            )}
          </div>

          {/* Parsed Sections */}
          <div className="card">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Parsed Resume Sections</h3>
            <div className="space-y-4">
              {Object.entries(sections).map(([sectionName, content]) => (
                <div key={sectionName} className="border border-gray-200 rounded-lg p-4">
                  <h4 className="font-medium text-gray-900 mb-2 capitalize">
                    {sectionName.replace(/_/g, ' ')}
                  </h4>
                  <div className="bg-gray-50 rounded p-3">
                    <pre className="text-sm text-gray-700 whitespace-pre-wrap font-mono">
                      {content.substring(0, 500)}{content.length > 500 ? '...' : ''}
                    </pre>
                  </div>
                  {content.length > 500 && (
                    <p className="text-xs text-gray-500 mt-2">
                      Showing first 500 characters of {content.length} total
                    </p>
                  )}
                </div>
              ))}
            </div>
          </div>
        </div>
      ) : (
        <div className="card">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Raw Extracted Text</h3>
          <div className="bg-gray-50 rounded-lg p-4 max-h-96 overflow-y-auto">
            <pre className="text-sm text-gray-700 whitespace-pre-wrap font-mono">
              {resume.extracted_text}
            </pre>
          </div>
          <p className="text-sm text-gray-500 mt-2">
            Total characters: {resume.extracted_text.length}
          </p>
        </div>
      )}
    </div>
  );
};

export default ResumeParsingDisplay;