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
    const prompt = this.createAnalysisPrompt(resumeText, jobDescription);
    
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

  createAnalysisPrompt(resumeText, jobDescription) {
    return `You are an expert ATS (Applicant Tracking System) analyzer. Carefully analyze this resume against the job description. Be thorough and accurate.

INSTRUCTIONS:
1. Read the ENTIRE resume text carefully - don't miss any skills or information
2. Look for skills in various forms (acronyms, full names, variations)
3. Check job descriptions, project descriptions, and skill sections
4. Be realistic with scoring - most good resumes score 60-85%
5. Only mark skills as "missing" if they are truly absent from the resume
6. Consider synonyms and related skills (e.g., "JavaScript" and "JS", "Machine Learning" and "ML")

RESUME TEXT:
${resumeText}

JOB DESCRIPTION:
${jobDescription}

ANALYSIS REQUIREMENTS:

For HARD SKILLS:
- Search thoroughly through ALL sections of the resume
- Look for exact matches, abbreviations, and common variations
- Check project descriptions, work experience, and skills sections
- Include evidence quotes when skills are found
- Only mark as "missing" if truly not present in any form

For EXPERIENCE:
- Calculate years based on actual dates in resume
- Consider all relevant experience, not just exact job titles
- Look at project experience and internships too

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
    "has_phone": <search for phone numbers>,
    "has_email": <search for email addresses>,
    "has_location": <search for city, state, or address>,
    "has_linkedin": <search for linkedin.com links>,
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
    "total_years_experience": "<calculate from actual dates>",
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