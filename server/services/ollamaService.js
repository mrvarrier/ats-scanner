const axios = require('axios');
const ScoringService = require('./scoringService');

class OllamaService {
  constructor() {
    this.baseURL = 'http://localhost:11434';
    this.availableModels = ['mistral:latest', 'qwen2.5:14b'];
  }

  async checkConnection() {
    try {
      const response = await axios.get(`${this.baseURL}/api/tags`);
      return response.status === 200;
    } catch (error) {
      console.error('Ollama connection error:', error.message);
      return false;
    }
  }

  async getAvailableModels() {
    try {
      const response = await axios.get(`${this.baseURL}/api/tags`);
      return response.data.models || [];
    } catch (error) {
      console.error('Error fetching models:', error.message);
      return [];
    }
  }

  async analyzeResume(resumeText, jobDescription, model = 'mistral:latest') {
    const prompt = this.createAnalysisPrompt(resumeText, jobDescription, model);
    
    try {
      const response = await axios.post(`${this.baseURL}/api/generate`, {
        model: model,
        prompt: prompt,
        stream: false,
        options: {
          temperature: 0.3,
          top_p: 0.9
        }
      }, {
        timeout: 300000 // 5 minutes timeout
      });

      return this.parseAnalysisResponse(response.data.response);
    } catch (error) {
      console.error('Analysis error:', error.message);
      throw new Error(`Analysis failed: ${error.message}`);
    }
  }

  createAnalysisPrompt(resumeText, jobDescription, model = 'mistral:latest') {
    // Create model-specific prompts to handle different AI behaviors
    const isMistral = model.includes('mistral');
    
    const baseInstructions = `You are an expert ATS (Applicant Tracking System) analyzer. Carefully analyze this resume against the job description. Be thorough and accurate.

RESUME TEXT:
${resumeText}

JOB DESCRIPTION:
${jobDescription}`;

    const strictInstructions = `
CRITICAL INSTRUCTIONS - READ CAREFULLY:
1. ONLY extract information that is explicitly written in the resume text above
2. DO NOT infer, assume, or add information that is not clearly stated
3. If you cannot find something in the resume, mark it as NOT FOUND
4. Look for skills in various forms (acronyms, full names, variations) but only if they actually exist in the text
5. Be realistic with scoring - most good resumes score 60-85%
6. Consider synonyms and related skills (e.g., "JavaScript" and "JS") but only if they appear in the resume

STRICT RULES FOR MISTRAL:
- If no location is mentioned in the resume, has_location = false
- If no phone number appears in the resume, has_phone = false  
- If no LinkedIn URL appears in the resume, has_linkedin = false
- Only mark skills as found if you can quote the exact text where they appear
- Do not hallucinate or make up information
- Example: If resume says "Python, JavaScript" then Python=found, JavaScript=found, but React=NOT found unless explicitly mentioned

VALIDATION CHECKLIST:
- Before marking has_location=true, find the actual city/state text in resume
- Before marking has_phone=true, find the actual phone number in resume
- Before marking has_linkedin=true, find the actual linkedin.com URL in resume
- Before marking skill as found, copy the exact quote from resume as evidence

ANALYSIS REQUIREMENTS:`;

    const lenientInstructions = `
INSTRUCTIONS:
1. Read the ENTIRE resume text carefully - don't miss any skills or information
2. Look for skills in various forms (acronyms, full names, variations)
3. Check job descriptions, project descriptions, and skill sections
4. Be realistic with scoring - most good resumes score 60-85%
5. Only mark skills as "missing" if they are truly absent from the resume
6. Consider synonyms and related skills (e.g., "JavaScript" and "JS", "Machine Learning" and "ML")

ANALYSIS REQUIREMENTS:`;

    const analysisRequirements = `

For HARD SKILLS:
- Search thoroughly through ALL sections of the resume
- Look for exact matches, abbreviations, and common variations
- Check project descriptions, work experience, and skills sections
- Include evidence quotes when skills are found
- Only mark as "missing" if truly not present in any form

For EXPERIENCE:
- Find date ranges in format "Month Year - Month Year" (e.g., "May 2023 - August 2025")
- Parse "Present", "Current", "Now" as current date
- Calculate total months across all employment periods
- Handle overlapping periods by merging them
- Convert to years and months (e.g., "2 years 3 months" for May 2023 - August 2025)
- Consider all relevant experience, not just exact job titles
- Look at project experience and internships too
- Example: "May 2023 - August 2025" = 27 months = 2 years 3 months

For ATS SCORING:
- Most professional resumes should score 60-85%
- Only deduct points for genuine gaps or issues
- Consider that ATS systems look for keyword matches primarily
- Factor in formatting, structure, and completeness

Return JSON format:
{
  "overall_score": <realistic 0-100, most good resumes 60-85>,
  "match_level": "<Excellent 85+/Good 70-84/Fair 50-69/Poor <50>",
  "contact_analysis": {
    "has_phone": <true only if phone number like (123) 456-7890 or 123-456-7890 exists in resume>,
    "has_email": <true only if email address like name@domain.com exists in resume>,
    "has_location": <true only if city/state like "San Francisco, CA" or address exists in resume>,
    "has_linkedin": <true only if linkedin.com/in/username URL exists in resume>,
    "contact_score": <0-100>
  },
  "job_title_analysis": {
    "current_title": "<most recent title from resume>",
    "target_title": "<title from job posting>",
    "title_match": "<exact/similar/different>",
    "title_score": <0-100>
  },
  "hard_skills": [
    {
      "skill": "<skill from job description>",
      "found_in_resume": <thoroughly check entire resume>,
      "evidence": "<exact quote where found, or empty if not found>",
      "required_for_job": <is this required or nice-to-have>,
      "skill_category": "<programming/tools/frameworks/databases/etc>"
    }
  ],
  "experience_analysis": {
    "total_years_experience": "<calculate precisely from date ranges, e.g. '2 years 3 months'>",
    "required_years": "<from job posting>",
    "experience_match": "<exceeds/meets/below>",
    "current_level": "<entry/mid/senior/executive>",
    "required_level": "<entry/mid/senior/executive>"
  },
  "keyword_optimization": {
    "critical_keywords_matched": ["<keywords found in resume>"],
    "critical_keywords_missing": ["<keywords truly missing>"],
    "total_job_keywords": <count important keywords in job>,
    "total_matched_keywords": <count found in resume>,
    "keyword_density": <percentage of job keywords found>,
    "ats_keywords_score": <0-100 based on keyword match>
  },
  "resume_structure": {
    "has_professional_summary": <check for summary/objective section>,
    "has_skills_section": <check for dedicated skills section>,
    "has_work_experience": <check for work/employment history>,
    "has_education_section": <check for education/degree info>,
    "has_contact_info": <check for email/phone>,
    "chronological_format": <is experience in reverse chronological order>,
    "structure_score": <0-100 based on organization>
  },
  "recommendations": [
    {
      "category": "<skills/experience/format/keywords>",
      "priority": "<high/medium/low>",
      "suggestion": "<specific, actionable advice>",
      "impact": "<high/medium/low>"
    }
  ],
  "ats_compatibility": {
    "likely_to_pass_screening": <realistic assessment>,
    "compatibility_score": <0-100, be realistic>,
    "format_issues": ["<actual formatting problems>"],
    "parsing_concerns": ["<real parsing issues>"]
  }
}

IMPORTANT: Be thorough and accurate. Don't mark skills as missing if they exist in the resume in any form.`;

    return baseInstructions + (isMistral ? strictInstructions : lenientInstructions) + analysisRequirements;
  }

  parseAnalysisResponse(response) {
    try {
      // Try to extract JSON from the response
      const jsonMatch = response.match(/\{[\s\S]*\}/);
      if (jsonMatch) {
        return JSON.parse(jsonMatch[0]);
      }
      
      // If no JSON found, create a basic structure
      return {
        overall_score: 50,
        match_level: "Fair",
        error: "Failed to parse AI response",
        raw_response: response
      };
    } catch (error) {
      console.error('Error parsing analysis response:', error);
      return {
        overall_score: 0,
        match_level: "Poor",
        error: "Analysis parsing failed",
        raw_response: response
      };
    }
  }
}

module.exports = new OllamaService();