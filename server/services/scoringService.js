class ScoringService {
  
  /**
   * Calibrates and validates the overall score based on professional ATS standards
   * Professional ATS systems are typically very strict, with most candidates scoring 20-60%
   */
  static calibrateScore(analysis) {
    if (!analysis || typeof analysis.overall_score !== 'number') {
      return analysis;
    }

    let calibratedScore = analysis.overall_score;
    
    // Apply professional ATS scoring penalties
    const penalties = this.calculatePenalties(analysis);
    calibratedScore = Math.max(0, calibratedScore - penalties.totalPenalty);
    
    // Apply scoring floor and ceiling based on professional standards
    calibratedScore = this.applyScoringBounds(calibratedScore, analysis);
    
    // Update the analysis with calibrated score
    const calibratedAnalysis = {
      ...analysis,
      original_score: analysis.overall_score,
      overall_score: Math.round(calibratedScore),
      scoring_adjustments: penalties,
      match_level: this.getMatchLevel(calibratedScore)
    };
    
    return calibratedAnalysis;
  }
  
  /**
   * Calculate penalties based on professional ATS criteria
   */
  static calculatePenalties(analysis) {
    const penalties = {
      missingCriticalSkills: 0,
      experienceGap: 0,
      educationMismatch: 0,
      formatIssues: 0,
      keywordDensity: 0,
      totalPenalty: 0
    };
    
    // Critical skills penalty (heavily weighted)
    if (analysis.missing_critical_skills) {
      const criticalSkillsCount = analysis.missing_critical_skills.filter(
        skill => skill.impact === 'high'
      ).length;
      penalties.missingCriticalSkills = criticalSkillsCount * 15; // 15 points per critical skill
    }
    
    // Missing skills penalty
    if (analysis.missing_skills) {
      const highPriorityMissing = analysis.missing_skills.filter(
        skill => skill.priority === 'high'
      ).length;
      penalties.missingCriticalSkills += highPriorityMissing * 10; // 10 points per high priority skill
    }
    
    // Experience gap penalty
    if (analysis.experience_analysis) {
      const expGap = this.parseExperienceGap(analysis.experience_analysis.experience_gap);
      if (expGap > 0) {
        penalties.experienceGap = Math.min(expGap * 5, 25); // 5 points per year gap, max 25
      }
      
      // Level mismatch penalty
      if (analysis.experience_analysis.level_required && analysis.experience_analysis.level_candidate) {
        const levelMismatch = this.calculateLevelMismatch(
          analysis.experience_analysis.level_required,
          analysis.experience_analysis.level_candidate
        );
        penalties.experienceGap += levelMismatch;
      }
    }
    
    // Education mismatch penalty
    if (analysis.education_analysis) {
      if (analysis.education_analysis.degree_match === 'missing') {
        penalties.educationMismatch = 20;
      } else if (analysis.education_analysis.degree_match === 'related') {
        penalties.educationMismatch = 10;
      }
      
      // Missing certifications penalty
      const missingCerts = analysis.education_analysis.certifications_missing?.length || 0;
      penalties.educationMismatch += missingCerts * 5;
    }
    
    // Format issues penalty
    if (analysis.format_analysis) {
      const formatScore = analysis.format_analysis.ats_friendly_score || 50;
      if (formatScore < 70) {
        penalties.formatIssues = (70 - formatScore) * 0.5; // 0.5 points per point below 70
      }
    }
    
    // Keyword density penalty
    if (analysis.keyword_analysis) {
      const keywordMatchRate = analysis.keyword_analysis.keyword_match_percentage || 0;
      if (keywordMatchRate < 50) {
        penalties.keywordDensity = (50 - keywordMatchRate) * 0.8; // 0.8 points per percentage below 50%
      }
    }
    
    penalties.totalPenalty = penalties.missingCriticalSkills + 
                           penalties.experienceGap + 
                           penalties.educationMismatch + 
                           penalties.formatIssues + 
                           penalties.keywordDensity;
    
    return penalties;
  }
  
  /**
   * Apply professional scoring bounds
   */
  static applyScoringBounds(score, analysis) {
    // Professional ATS systems rarely give scores above 80% unless exceptional
    if (score > 80) {
      // Only allow 80%+ for truly exceptional matches
      const exceptionalCriteria = this.isExceptionalMatch(analysis);
      if (!exceptionalCriteria) {
        score = Math.min(score, 75);
      }
    }
    
    // Most professional systems score between 20-60% for typical candidates
    if (score > 60 && score <= 80) {
      // Good matches typically score 45-60%
      const hasMinorIssues = this.hasMinorIssues(analysis);
      if (hasMinorIssues) {
        score = Math.min(score, 55);
      }
    }
    
    return score;
  }
  
  /**
   * Check if this is an exceptional match (80%+ worthy)
   */
  static isExceptionalMatch(analysis) {
    const criteria = {
      perfectSkillsMatch: false,
      perfectExperienceMatch: false,
      perfectEducationMatch: false,
      excellentFormat: false,
      highKeywordDensity: false
    };
    
    // Perfect skills match
    const missingCritical = (analysis.missing_critical_skills?.length || 0) + 
                           (analysis.missing_skills?.filter(s => s.priority === 'high').length || 0);
    criteria.perfectSkillsMatch = missingCritical === 0;
    
    // Perfect experience match
    criteria.perfectExperienceMatch = analysis.experience_analysis?.industry_match === 'exact' &&
                                     !analysis.experience_analysis?.experience_gap;
    
    // Perfect education match
    criteria.perfectEducationMatch = analysis.education_analysis?.degree_match === 'exact' &&
                                    (analysis.education_analysis?.certifications_missing?.length || 0) === 0;
    
    // Excellent format
    criteria.excellentFormat = (analysis.format_analysis?.ats_friendly_score || 0) >= 90;
    
    // High keyword density
    criteria.highKeywordDensity = (analysis.keyword_analysis?.keyword_match_percentage || 0) >= 80;
    
    // Need at least 4 out of 5 criteria for exceptional score
    const meetsCriteria = Object.values(criteria).filter(Boolean).length;
    return meetsCriteria >= 4;
  }
  
  /**
   * Check for minor issues that would cap the score
   */
  static hasMinorIssues(analysis) {
    const issues = [];
    
    // Minor skill gaps
    const minorSkillGaps = (analysis.missing_skills?.filter(s => s.priority === 'medium').length || 0);
    if (minorSkillGaps > 2) issues.push('multiple_minor_skill_gaps');
    
    // Format issues
    const formatScore = analysis.format_analysis?.ats_friendly_score || 100;
    if (formatScore < 80) issues.push('format_issues');
    
    // Keyword density
    const keywordMatch = analysis.keyword_analysis?.keyword_match_percentage || 100;
    if (keywordMatch < 65) issues.push('keyword_density_low');
    
    return issues.length > 0;
  }
  
  /**
   * Parse experience gap from text
   */
  static parseExperienceGap(gapText) {
    if (!gapText) return 0;
    
    const match = gapText.match(/(\d+)/);
    return match ? parseInt(match[1]) : 0;
  }
  
  /**
   * Calculate level mismatch penalty
   */
  static calculateLevelMismatch(required, candidate) {
    const levels = { entry: 1, mid: 2, senior: 3, executive: 4 };
    const requiredLevel = levels[required] || 2;
    const candidateLevel = levels[candidate] || 2;
    
    const gap = requiredLevel - candidateLevel;
    if (gap > 0) {
      return gap * 10; // 10 points per level below required
    }
    
    return 0; // No penalty for being above required level
  }
  
  /**
   * Get match level based on calibrated score
   */
  static getMatchLevel(score) {
    if (score >= 75) return 'Excellent';
    if (score >= 55) return 'Good';
    if (score >= 35) return 'Fair';
    return 'Poor';
  }
  
  /**
   * Generate score explanation
   */
  static generateScoreExplanation(analysis) {
    const score = analysis.overall_score;
    const adjustments = analysis.scoring_adjustments;
    
    let explanation = `Your resume scored ${score}% using professional ATS standards. `;
    
    if (score >= 60) {
      explanation += "This is a strong score that should pass most ATS systems and get recruiter attention.";
    } else if (score >= 40) {
      explanation += "This score suggests your resume may pass some ATS systems but needs improvement for better results.";
    } else {
      explanation += "This score indicates your resume is likely to be filtered out by most ATS systems.";
    }
    
    if (adjustments?.totalPenalty > 10) {
      explanation += ` Key issues identified: `;
      const issues = [];
      if (adjustments.missingCriticalSkills > 10) issues.push("missing critical skills");
      if (adjustments.experienceGap > 5) issues.push("experience gaps");
      if (adjustments.educationMismatch > 5) issues.push("education mismatch");
      if (adjustments.formatIssues > 5) issues.push("format problems");
      if (adjustments.keywordDensity > 5) issues.push("low keyword density");
      
      explanation += issues.join(", ") + ".";
    }
    
    return explanation;
  }
}

module.exports = ScoringService;