export interface Resume {
  id: number;
  filename: string;
  original_name: string;
  file_path: string;
  upload_date: string;
  file_size: number;
  extracted_text: string;
  last_scan_id?: number;
  last_score?: number;
  last_scan_date?: string;
  last_job_title?: string;
  last_company?: string;
}

export interface JobDescription {
  id: number;
  title?: string;
  description: string;
  company?: string;
  date_added: string;
  reuse_count: number;
}

export interface Scan {
  id: number;
  resume_id: number;
  job_description_id: number;
  model_used: string;
  overall_score: number;
  analysis_json: string;
  scan_date: string;
  resume_name?: string;
  job_title?: string;
  company?: string;
  analysis?: AnalysisResult;
  skills?: Skill[];
}

export interface Skill {
  id: number;
  scan_id: number;
  skill_name: string;
  skill_type: 'found' | 'missing' | 'recommended';
  found_in_resume: boolean;
  priority_level: 'high' | 'medium' | 'low';
  recommendation?: string;
}

export interface ContactAnalysis {
  has_phone: boolean;
  has_email: boolean;
  has_location: boolean;
  has_linkedin: boolean;
  contact_score: number;
}

export interface JobTitleAnalysis {
  current_title: string;
  target_title: string;
  title_match: 'exact' | 'similar' | 'different';
  title_score: number;
}

export interface HardSkill {
  skill: string;
  found_in_resume: boolean;
  evidence?: string;
  required_for_job: boolean;
  skill_category: string;
}

export interface SoftSkill {
  skill: string;
  found_in_resume: boolean;
  evidence?: string;
  required_for_job: boolean;
}

export interface KeywordOptimization {
  critical_keywords_matched: string[];
  critical_keywords_missing: string[];
  total_job_keywords: number;
  total_matched_keywords: number;
  keyword_density: number;
  ats_keywords_score: number;
}

export interface ResumeStructure {
  has_professional_summary: boolean;
  has_skills_section: boolean;
  has_work_experience: boolean;
  has_education_section: boolean;
  has_contact_info: boolean;
  chronological_format: boolean;
  section_organization: 'excellent' | 'good' | 'poor';
  structure_score: number;
}

export interface MeasurableResults {
  has_quantified_achievements: boolean;
  number_of_metrics: number;
  examples: string[];
  results_score: number;
}

export interface IndustryAlignment {
  industry_experience: 'direct' | 'related' | 'unrelated';
  company_size_match: 'startup' | 'small' | 'medium' | 'large' | 'enterprise';
  industry_keywords: string[];
  alignment_score: number;
}

export interface ATSCompatibility {
  likely_to_pass_screening: boolean;
  compatibility_score: number;
  format_issues: string[];
  parsing_concerns: string[];
}

export interface AnalysisResult {
  overall_score: number;
  match_level: 'Excellent' | 'Good' | 'Fair' | 'Poor';
  contact_analysis?: ContactAnalysis;
  job_title_analysis?: JobTitleAnalysis;
  hard_skills?: HardSkill[];
  soft_skills?: SoftSkill[];
  experience_analysis: ExperienceAnalysis;
  education_analysis?: EducationAnalysis;
  keyword_optimization?: KeywordOptimization;
  resume_structure?: ResumeStructure;
  measurable_results?: MeasurableResults;
  industry_alignment?: IndustryAlignment;
  recommendations?: Recommendation[];
  ats_compatibility?: ATSCompatibility;
  // Legacy support
  found_skills?: FoundSkill[];
  missing_skills?: MissingSkill[];
  keyword_analysis?: KeywordAnalysis;
  error?: string;
  raw_response?: string;
}

export interface ScoringAdjustments {
  missingCriticalSkills: number;
  experienceGap: number;
  educationMismatch: number;
  formatIssues: number;
  keywordDensity: number;
  totalPenalty: number;
}

export interface ScoringBreakdown {
  hard_skills_score: number;
  experience_score: number;
  education_score: number;
  industry_score: number;
  format_score: number;
  soft_skills_score: number;
}

export interface FoundSkill {
  skill: string;
  evidence: string;
  relevance?: 'high' | 'medium' | 'low';
  match_type?: 'exact' | 'partial' | 'synonym';
  importance?: 'critical' | 'important' | 'nice-to-have';
}

export interface MissingCriticalSkill {
  skill: string;
  category: 'technical' | 'certification' | 'experience';
  impact: 'high' | 'medium' | 'low';
  frequency_in_job: number;
}

export interface MissingSkill {
  skill: string;
  priority: 'high' | 'medium' | 'low';
  impact: string;
}

export interface ExperienceAnalysis {
  total_years_experience: string;
  required_years: string;
  experience_match: 'exceeds' | 'meets' | 'below';
  relevant_experience: string;
  career_progression: 'clear' | 'moderate' | 'unclear';
  current_level: 'entry' | 'mid' | 'senior' | 'executive';
  required_level: 'entry' | 'mid' | 'senior' | 'executive';
  // Legacy support
  years_required?: string;
  years_candidate?: string;
  level_match?: 'junior' | 'mid' | 'senior';
  industry_relevance?: 'high' | 'medium' | 'low';
}

export interface EducationAnalysis {
  highest_degree: string;
  required_degree: string;
  degree_match: 'exceeds' | 'meets' | 'below';
  relevant_major: boolean;
  certifications_found: string[];
  certifications_required: string[];
  education_score: number;
  // Legacy support
  degree_required?: string;
  degree_candidate?: string;
  certifications_missing?: string[];
}

export interface KeywordAnalysis {
  total_keywords_in_job?: number;
  matched_keywords: string[];
  missing_keywords: string[];
  keyword_match_percentage?: number;
  keyword_density_score?: number;
  ats_score: number;
}

export interface FormatAnalysis {
  has_contact_info: boolean;
  has_summary: boolean;
  has_skills_section: boolean;
  has_experience_section: boolean;
  has_education_section: boolean;
  chronological_order: boolean;
  consistent_formatting: boolean;
  ats_friendly_score: number;
}

export interface RedFlag {
  issue: string;
  severity: 'critical' | 'major' | 'minor';
  impact: string;
}

export interface ImprovementPriority {
  priority: number;
  category: 'skills' | 'experience' | 'education' | 'format';
  action: string;
  impact: string;
  urgency: 'immediate' | 'soon' | 'later';
}

export interface Recommendation {
  priority: 'high' | 'medium' | 'low';
  category: 'skills' | 'experience' | 'keywords' | 'format';
  suggestion: string;
  impact?: 'high' | 'medium' | 'low';
  example?: string;
}

export interface CompetitiveAnalysis {
  estimated_ranking: 'top 5%' | 'top 10%' | 'top 25%' | 'top 50%' | 'bottom 50%';
  likely_to_pass_ats?: boolean;
  recruiter_attention_score?: number;
  interview_likelihood?: 'high' | 'medium' | 'low' | 'very low';
  salary_competitiveness?: 'above market' | 'at market' | 'below market';
  strengths: string[];
  weaknesses: string[];
  unique_value: string;
}

export interface OllamaModel {
  name: string;
  description: string;
  speed: 'fast' | 'slower';
}

export interface ModelResponse {
  available: boolean;
  models: any[];
  recommended: OllamaModel[];
  error?: string;
}