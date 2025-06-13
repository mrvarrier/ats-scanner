/**
 * Professional ATS Scoring Service
 * Based on industry standards from Jobscan, ResumeWorded, and Fortune 500 ATS systems
 */
class ProfessionalScoringService {
  
  /**
   * Calculate comprehensive ATS score using weighted categories
   * Industry standard: Keyword matching (40%), Experience (25%), Skills (20%), Format (15%)
   */
  static calculateATSScore(analysis) {
    const weights = {
      keywords: 0.40,      // 40% - Most important for ATS
      experience: 0.25,    // 25% - Years and relevance
      skills: 0.20,        // 20% - Technical skills match
      format: 0.15         // 15% - ATS readability
    };
    
    const scores = {
      keywords: this.calculateKeywordScore(analysis),
      experience: this.calculateExperienceScore(analysis),
      skills: this.calculateSkillsScore(analysis),
      format: this.calculateFormatScore(analysis)
    };
    
    // Calculate weighted overall score
    let overallScore = 0;
    for (const [category, weight] of Object.entries(weights)) {
      overallScore += scores[category] * weight;
    }
    
    // Apply industry-standard adjustments
    const adjustedScore = this.applyIndustryAdjustments(overallScore, analysis);
    
    return {
      overall_score: Math.round(adjustedScore),
      category_scores: scores,
      weights: weights,
      match_level: this.getMatchLevel(adjustedScore),
      ats_compatibility: this.calculateATSCompatibility(scores),
      scoring_breakdown: this.generateScoringBreakdown(scores, weights)
    };
  }
  
  /**
   * Keyword Score (40% weight) - Based on Jobscan methodology
   */
  static calculateKeywordScore(analysis) {
    if (!analysis.keyword_optimization) return 30; // Default low score if no keyword analysis
    
    const keywordData = analysis.keyword_optimization;
    const matchRate = keywordData.total_matched_keywords / Math.max(keywordData.total_job_keywords, 1);
    
    let keywordScore = 0;
    
    // Base keyword match score (0-60 points)
    keywordScore += matchRate * 60;
    
    // Critical keywords bonus (0-25 points)
    const criticalMatchRate = keywordData.critical_keywords_matched.length / 
                             Math.max(keywordData.critical_keywords_missing.length + keywordData.critical_keywords_matched.length, 1);
    keywordScore += criticalMatchRate * 25;
    
    // Keyword density bonus (0-15 points)
    const densityScore = Math.min(keywordData.keyword_density / 100 * 15, 15);
    keywordScore += densityScore;
    
    // Penalty for keyword stuffing (over 3% density)
    if (keywordData.keyword_density > 3) {
      keywordScore -= (keywordData.keyword_density - 3) * 5;
    }
    
    return Math.max(0, Math.min(100, keywordScore));
  }
  
  /**
   * Experience Score (25% weight) - Fortune 500 ATS standards
   */
  static calculateExperienceScore(analysis) {
    if (!analysis.experience_analysis) return 40;
    
    const expData = analysis.experience_analysis;
    let experienceScore = 0;
    
    // Years of experience match (0-40 points)
    const requiredYears = this.parseYears(expData.required_years);
    const candidateYears = this.parseYears(expData.total_years_experience);
    
    if (candidateYears >= requiredYears) {
      experienceScore += 40; // Full points for meeting requirement
      if (candidateYears > requiredYears * 1.5) {
        experienceScore += 10; // Bonus for exceeding significantly
      }
    } else {
      const matchRatio = candidateYears / Math.max(requiredYears, 1);
      experienceScore += matchRatio * 30; // Partial credit
    }
    
    // Level match (0-25 points)
    const levelMatch = this.calculateLevelMatch(expData.required_level, expData.current_level);
    experienceScore += levelMatch;
    
    // Career progression (0-20 points)
    if (expData.career_progression === 'clear') {
      experienceScore += 20;
    } else if (expData.career_progression === 'moderate') {
      experienceScore += 12;
    } else {
      experienceScore += 5;
    }
    
    // Industry relevance (0-15 points)
    if (analysis.industry_alignment) {
      const industryExp = analysis.industry_alignment.industry_experience;
      if (industryExp === 'direct') experienceScore += 15;
      else if (industryExp === 'related') experienceScore += 8;
      else experienceScore += 2;
    }
    
    return Math.max(0, Math.min(100, experienceScore));
  }
  
  /**
   * Skills Score (20% weight) - Technical skills assessment
   */
  static calculateSkillsScore(analysis) {
    if (!analysis.hard_skills || analysis.hard_skills.length === 0) return 25;
    
    const hardSkills = analysis.hard_skills;
    const totalSkills = hardSkills.length;
    const foundSkills = hardSkills.filter(skill => skill.found_in_resume).length;
    const requiredSkills = hardSkills.filter(skill => skill.required_for_job).length;
    const foundRequiredSkills = hardSkills.filter(skill => skill.found_in_resume && skill.required_for_job).length;
    
    let skillsScore = 0;
    
    // Required skills match (0-60 points) - Most important
    if (requiredSkills > 0) {
      skillsScore += (foundRequiredSkills / requiredSkills) * 60;
    } else {
      skillsScore += 30; // Default if no required skills specified
    }
    
    // Overall skills match (0-25 points)
    skillsScore += (foundSkills / totalSkills) * 25;
    
    // Skill category diversity bonus (0-15 points)
    const categories = new Set(hardSkills.filter(s => s.found_in_resume).map(s => s.skill_category));
    const categoryBonus = Math.min(categories.size * 3, 15);
    skillsScore += categoryBonus;
    
    return Math.max(0, Math.min(100, skillsScore));
  }
  
  /**
   * Format Score (15% weight) - ATS readability
   */
  static calculateFormatScore(analysis) {
    if (!analysis.resume_structure) return 50;
    
    const structure = analysis.resume_structure;
    let formatScore = 0;
    
    // Essential sections (0-50 points)
    const essentialSections = [
      'has_contact_info',
      'has_work_experience', 
      'has_skills_section'
    ];
    
    essentialSections.forEach(section => {
      if (structure[section]) formatScore += 16.67; // 50/3 points each
    });
    
    // Additional beneficial sections (0-30 points)
    const beneficialSections = [
      'has_professional_summary',
      'has_education_section'
    ];
    
    beneficialSections.forEach(section => {
      if (structure[section]) formatScore += 15; // 30/2 points each
    });
    
    // Chronological format (0-20 points)
    if (structure.chronological_format) {
      formatScore += 20;
    }
    
    // ATS compatibility check
    if (analysis.ats_compatibility) {
      const atsData = analysis.ats_compatibility;
      
      // Deduct points for format issues
      if (atsData.format_issues && atsData.format_issues.length > 0) {
        formatScore -= atsData.format_issues.length * 5;
      }
      
      // Deduct points for parsing concerns
      if (atsData.parsing_concerns && atsData.parsing_concerns.length > 0) {
        formatScore -= atsData.parsing_concerns.length * 3;
      }
    }
    
    return Math.max(0, Math.min(100, formatScore));
  }
  
  /**
   * Apply industry-standard adjustments based on professional ATS behavior
   */
  static applyIndustryAdjustments(score, analysis) {
    let adjustedScore = score;
    
    // Contact information penalty (critical for ATS)
    if (analysis.contact_analysis) {
      const contact = analysis.contact_analysis;
      if (!contact.has_email) adjustedScore -= 15; // Major penalty
      if (!contact.has_phone) adjustedScore -= 10;
      
      // Small bonus for complete contact info
      if (contact.has_email && contact.has_phone && contact.has_linkedin) {
        adjustedScore += 5;
      }
    }
    
    // Job title alignment bonus/penalty
    if (analysis.job_title_analysis) {
      const titleMatch = analysis.job_title_analysis.title_match;
      if (titleMatch === 'exact') adjustedScore += 8;
      else if (titleMatch === 'similar') adjustedScore += 3;
      else adjustedScore -= 5;
    }
    
    // Industry standards: Most scores fall between 30-75%
    if (adjustedScore > 80) {
      // Only exceptional candidates score above 80%
      const isExceptional = this.isExceptionalCandidate(analysis);
      if (!isExceptional) {
        adjustedScore = Math.min(adjustedScore, 75);
      }
    }
    
    // Apply realistic floor - even poor resumes typically score 15-25%
    adjustedScore = Math.max(adjustedScore, 15);
    
    return adjustedScore;
  }
  
  /**
   * Determine if candidate is exceptional (80%+ worthy)
   */
  static isExceptionalCandidate(analysis) {
    const criteria = [];
    
    // Perfect keyword match
    if (analysis.keyword_optimization) {
      const keywordMatch = analysis.keyword_optimization.total_matched_keywords / 
                          Math.max(analysis.keyword_optimization.total_job_keywords, 1);
      if (keywordMatch >= 0.9) criteria.push('keywords');
    }
    
    // Exceeds experience requirements
    if (analysis.experience_analysis) {
      if (analysis.experience_analysis.experience_match === 'exceeds') {
        criteria.push('experience');
      }
    }
    
    // Perfect skills match
    if (analysis.hard_skills) {
      const requiredFound = analysis.hard_skills.filter(s => s.required_for_job && s.found_in_resume).length;
      const totalRequired = analysis.hard_skills.filter(s => s.required_for_job).length;
      if (totalRequired > 0 && requiredFound === totalRequired) {
        criteria.push('skills');
      }
    }
    
    // Excellent format
    if (analysis.ats_compatibility && analysis.ats_compatibility.compatibility_score >= 90) {
      criteria.push('format');
    }
    
    // Need at least 3 exceptional criteria
    return criteria.length >= 3;
  }
  
  /**
   * Calculate level match score
   */
  static calculateLevelMatch(required, current) {
    const levels = { 'entry': 1, 'mid': 2, 'senior': 3, 'executive': 4 };
    const reqLevel = levels[required] || 2;
    const currLevel = levels[current] || 2;
    
    if (currLevel >= reqLevel) {
      return 25; // Full points for meeting or exceeding
    } else {
      const ratio = currLevel / reqLevel;
      return ratio * 15; // Partial credit
    }
  }
  
  /**
   * Parse years from various formats
   */
  static parseYears(yearString) {
    if (!yearString) return 0;
    
    // Handle "X years Y months" format
    const yearMatch = yearString.match(/(\d+)\s*year/i);
    const monthMatch = yearString.match(/(\d+)\s*month/i);
    
    let years = yearMatch ? parseInt(yearMatch[1]) : 0;
    const months = monthMatch ? parseInt(monthMatch[1]) : 0;
    
    years += months / 12;
    
    // Handle simple number format
    if (years === 0) {
      const numMatch = yearString.match(/(\d+(?:\.\d+)?)/);
      if (numMatch) {
        years = parseFloat(numMatch[1]);
      }
    }
    
    return years;
  }
  
  /**
   * Calculate ATS compatibility score
   */
  static calculateATSCompatibility(scores) {
    const avgScore = (scores.keywords + scores.experience + scores.skills + scores.format) / 4;
    
    // ATS systems are particularly sensitive to keywords and format
    const atsWeightedScore = (scores.keywords * 0.5) + (scores.format * 0.3) + (avgScore * 0.2);
    
    return {
      likely_to_pass_screening: atsWeightedScore >= 60,
      compatibility_score: Math.round(atsWeightedScore),
      critical_issues: this.identifyCriticalIssues(scores)
    };
  }
  
  /**
   * Identify critical issues that would cause ATS rejection
   */
  static identifyCriticalIssues(scores) {
    const issues = [];
    
    if (scores.keywords < 40) issues.push("Low keyword match - resume may not appear in searches");
    if (scores.format < 50) issues.push("Format issues may prevent proper parsing");
    if (scores.skills < 30) issues.push("Missing too many required technical skills");
    if (scores.experience < 40) issues.push("Experience level doesn't meet job requirements");
    
    return issues;
  }
  
  /**
   * Get match level based on industry standards
   */
  static getMatchLevel(score) {
    if (score >= 75) return 'Excellent';  // Top 10% of candidates
    if (score >= 60) return 'Good';       // Top 25% of candidates  
    if (score >= 45) return 'Fair';       // Average candidates
    return 'Poor';                        // Below average
  }
  
  /**
   * Generate detailed scoring breakdown
   */
  static generateScoringBreakdown(scores, weights) {
    return {
      keyword_analysis: {
        score: scores.keywords,
        weight: weights.keywords,
        contribution: Math.round(scores.keywords * weights.keywords),
        importance: "Critical - ATS systems primarily match on keywords"
      },
      experience_analysis: {
        score: scores.experience,
        weight: weights.experience, 
        contribution: Math.round(scores.experience * weights.experience),
        importance: "High - Years and relevance determine qualification level"
      },
      skills_analysis: {
        score: scores.skills,
        weight: weights.skills,
        contribution: Math.round(scores.skills * weights.skills),
        importance: "High - Technical skills must match job requirements"
      },
      format_analysis: {
        score: scores.format,
        weight: weights.format,
        contribution: Math.round(scores.format * weights.format),
        importance: "Medium - Poor formatting prevents ATS from parsing correctly"
      }
    };
  }
  
  /**
   * Generate improvement recommendations based on score breakdown
   */
  static generateRecommendations(analysis, scoringResult) {
    const recommendations = [];
    const scores = scoringResult.category_scores;
    
    // Keyword recommendations (highest priority)
    if (scores.keywords < 60) {
      recommendations.push({
        category: "keywords",
        priority: "high",
        suggestion: `Increase keyword match from ${Math.round(scores.keywords)}% to 70%+. Add missing critical keywords: ${analysis.keyword_optimization?.critical_keywords_missing?.slice(0, 3).join(', ') || 'N/A'}`,
        impact: "high"
      });
    }
    
    // Experience recommendations
    if (scores.experience < 50) {
      recommendations.push({
        category: "experience", 
        priority: "high",
        suggestion: "Highlight relevant experience more prominently. Quantify achievements with specific metrics and results.",
        impact: "high"
      });
    }
    
    // Skills recommendations
    if (scores.skills < 40) {
      const missingSkills = analysis.hard_skills?.filter(s => s.required_for_job && !s.found_in_resume)?.slice(0, 3);
      recommendations.push({
        category: "skills",
        priority: "high", 
        suggestion: `Add missing required skills: ${missingSkills?.map(s => s.skill).join(', ') || 'Review job requirements'}`,
        impact: "high"
      });
    }
    
    // Format recommendations
    if (scores.format < 60) {
      recommendations.push({
        category: "format",
        priority: "medium",
        suggestion: "Improve ATS compatibility - use standard section headings, avoid graphics/tables, ensure chronological format",
        impact: "medium"
      });
    }
    
    return recommendations;
  }
}

module.exports = ProfessionalScoringService;