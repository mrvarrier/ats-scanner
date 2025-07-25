// Shared type definitions for API responses and common data structures

export interface CommandResult<T> {
  success: boolean;
  data?: T;
  error?: string;
}

export interface DocumentInfo {
  id: string;
  filename: string;
  content: string;
  file_type: string;
  size: number;
  word_count: number;
  character_count: number;
}

export interface OllamaModel {
  name: string;
  size: number;
  digest: string;
  modified_at: string;
}

export interface CategoryScores {
  skills: number;
  experience: number;
  education: number;
  keywords: number;
  format: number;
}

export interface AnalysisResult {
  overall_score: number;
  category_scores: CategoryScores;
  detailed_feedback: string;
  missing_keywords: string[];
  recommendations: string[];
  processing_time_ms: number;
}

// Achievement Analysis Types
export interface AchievementAnalysis {
  bullet_points: BulletAnalysis[];
  xyz_formula_usage: number;
  achievement_density: number;
  quantification_score: number;
  action_verb_strength: number;
  overall_achievement_score: number;
  suggestions: AchievementSuggestion[];
}

export interface BulletAnalysis {
  text: string;
  section: string;
  has_xyz_structure: boolean;
  action_verb: string;
  quantification: string;
  impact: string;
  strength_score: number;
  suggestions: string[];
}

export interface AchievementSuggestion {
  bullet_point: string;
  suggestion_type: string;
  original: string;
  improved: string;
  explanation: string;
  impact_score: number;
}

// ML Insights Types
export interface MLInsights {
  success_prediction: SuccessPrediction;
  career_path_suggestions: CareerPathSuggestions;
  salary_prediction: SalaryPrediction;
  ml_recommendations: MLRecommendation[];
  feature_analysis: FeatureAnalysis;
  confidence_score: number;
}

export interface SuccessPrediction {
  application_success_probability: number;
  interview_likelihood: number;
  hiring_probability: number;
  factors_analysis: PredictionFactor[];
  risk_factors: string[];
  success_factors: string[];
}

export interface CareerPathSuggestions {
  current_level: string;
  suggested_roles: SuggestedRole[];
  skill_gaps: SkillGap[];
  growth_trajectory: GrowthPath[];
  industry_alignment: number;
}

export interface SalaryPrediction {
  predicted_salary_range: SalaryRange;
  market_percentile: number;
  factors_affecting_salary: SalaryFactor[];
  improvement_potential: number;
  location_adjustments: LocationAdjustment[];
}

export interface SuggestedRole {
  title: string;
  match_score: number;
  requirements_met: number;
  missing_skills: string[];
  salary_range: SalaryRange;
  growth_potential: number;
}

export interface SalaryRange {
  min: number;
  max: number;
  median: number;
}

export interface SkillGap {
  skill: string;
  current_level: number;
  required_level: number;
  learning_resources: string[];
}

export interface GrowthPath {
  role: string;
  timeline: string;
  requirements: string[];
  salary_increase: number;
}

export interface PredictionFactor {
  factor: string;
  impact: number;
  explanation: string;
}

export interface SalaryFactor {
  factor: string;
  impact_percentage: number;
  description: string;
}

export interface LocationAdjustment {
  location: string;
  adjustment_factor: number;
  cost_of_living: number;
}

export interface MLRecommendation {
  category: string;
  priority: string;
  recommendation: string;
  expected_outcome: string;
  implementation_difficulty: string;
  impact_score: number;
}

export interface FeatureAnalysis {
  feature_importance: FeatureImportance[];
  model_accuracy: number;
  prediction_confidence: number;
}

export interface FeatureImportance {
  feature: string;
  importance: number;
  description: string;
}

// Individual ML Command Response Types
export interface ApplicationSuccessResponse {
  success_prediction: number;
  confidence_metrics: ConfidenceMetrics;
  generated_at: string;
}

export interface ConfidenceMetrics {
  overall_confidence: number;
  prediction_reliability: number;
  data_quality_score: number;
  model_performance_score: number;
  feature_completeness: number;
}

export interface CareerPathSuggestionsResponse {
  career_path_suggestions: CareerPathSuggestion[];
  skill_demand_forecast: SkillDemandForecast[];
  generated_at: string;
}

export interface CareerPathSuggestion {
  role: string;
  title: string;
  match_score: number;
  confidence: number;
  description: string;
  requirements: string[];
  salary_range: SalaryRange;
  growth_potential: number;
}

export interface SkillDemandForecast {
  skill: string;
  demand: number;
  trend: string;
  growth_rate: number;
  learning_priority: string;
}

export interface SalaryPredictionMLResponse {
  salary_prediction: SalaryPredictionML;
  confidence_metrics: ConfidenceMetrics;
  generated_at: string;
}

export interface SalaryPredictionML {
  predicted_range: SalaryRange;
  base_prediction: number;
  market_percentile: number;
  confidence_interval: number;
  factors_analysis: SalaryFactorAnalysis[];
}

export interface SalaryFactorAnalysis {
  factor: string;
  impact_weight: number;
  contribution_percentage: number;
  description: string;
}

export interface MLRecommendationsResponse {
  recommendations: MLRecommendationItem[];
  optimization_prioritization: OptimizationPriority[];
  confidence_metrics: ConfidenceMetrics;
  generated_at: string;
}

export interface MLRecommendationItem {
  category: string;
  priority: 'High' | 'Medium' | 'Low';
  recommendation: string;
  expected_outcome: string;
  implementation_difficulty: string;
  impact_score: number;
  confidence: number;
  difficulty: string;
}

export interface OptimizationPriority {
  area: string;
  category: string;
  description: string;
  priority_score: number;
  roi_estimate: number;
  effort_required: string;
}

// User Preferences Types
export interface UserPreferences {
  theme: 'Light' | 'Dark' | 'HighContrast';
  auto_connect_on_startup: boolean;
  default_model?: string;
  analysis_history_retention_days: number;
  notification_preferences: NotificationPreferences;
  privacy_settings: PrivacySettings;
  performance_settings: PerformanceSettings;
}

export interface NotificationPreferences {
  enable_analysis_complete: boolean;
  enable_error_notifications: boolean;
  enable_update_notifications: boolean;
}

export interface PrivacySettings {
  store_analysis_history: boolean;
  anonymous_usage_statistics: boolean;
  auto_clear_uploaded_files: boolean;
}

export interface PerformanceSettings {
  max_concurrent_analyses: number;
  cache_analysis_results: boolean;
  auto_optimize_memory: boolean;
}

// Error Types
export interface AppError {
  code: string;
  message: string;
  details?: Record<string, unknown>;
  timestamp: string;
}

// Analysis Request Types
export interface AnalysisRequest {
  resume_content: string;
  job_description: string;
  model_name: string;
}

export interface OptimizationRequest {
  resume_content: string;
  job_description: string;
  model_name: string;
  optimization_level: 'Basic' | 'Intermediate' | 'Advanced';
}

// Analysis Database Record
export interface Analysis {
  id: string;
  resume_id: string;
  job_description_id: string;
  model_used: string;
  overall_score: number;
  skills_score: number;
  experience_score: number;
  education_score: number;
  keywords_score: number;
  format_score: number;
  detailed_feedback: string;
  missing_keywords: string;
  recommendations: string;
  processing_time_ms: number;
  created_at: string;
}

// Semantic Analysis Types
export interface SemanticAnalysisResult {
  overall_score: number;
  keyword_relevance_score: number;
  contextual_understanding_score: number;
  semantic_matches: SemanticMatch[];
  conceptual_gaps: ConceptualGap[];
  industry_alignment_score: number;
  recommendations: string[];
  processed_at: string;
}

export interface SemanticMatch {
  keyword: string;
  context: string;
  relevance_score: number;
  match_type: 'exact' | 'semantic' | 'contextual';
  section: string;
}

export interface ConceptualGap {
  concept: string;
  importance_score: number;
  suggested_keywords: string[];
  explanation: string;
}

// Format Compatibility Types
export interface FormatCompatibilityReport {
  overall_compatibility_score: number;
  ats_compatibility_rating: 'excellent' | 'good' | 'fair' | 'poor';
  format_issues: FormatIssue[];
  recommendations: FormatRecommendation[];
  parsing_analysis: ParsingAnalysis;
  section_detection: SectionDetection;
}

export interface FormatIssue {
  issue_type: string;
  severity: 'critical' | 'major' | 'minor';
  description: string;
  location: string;
  suggestion: string;
}

export interface FormatRecommendation {
  category: string;
  priority: 'high' | 'medium' | 'low';
  description: string;
  expected_impact: string;
}

export interface SectionDetection {
  detected_sections: string[];
  missing_standard_sections: string[];
  section_order_score: number;
  header_formatting_score: number;
}

// Industry Analysis Types
export interface IndustryAnalysisResult {
  detected_industry: string;
  confidence_score: number;
  industry_keywords: IndustryKeywordMatch[];
  role_level_assessment: RoleLevelAssessment;
  required_certifications: CertificationCheck[];
  industry_trends: TrendAnalysis[];
  domain_expertise_score: number;
  industry_specific_recommendations: string[];
}

export interface IndustryKeywordMatch {
  keyword: string;
  category: string;
  found: boolean;
  frequency: number;
  context: string[];
  weight: number;
  synonyms_found: string[];
}

export interface RoleLevelAssessment {
  detected_level: string;
  confidence: number;
  experience_indicators: ExperienceIndicator[];
  leadership_indicators: LeadershipIndicator[];
  years_of_experience_estimate?: number;
  seniority_signals: SenioritySignal[];
}

export interface ExperienceIndicator {
  indicator_type: string;
  description: string;
  weight: number;
  context: string;
}

export interface LeadershipIndicator {
  indicator_type: string;
  description: string;
  team_size?: number;
  scope: string;
  context: string;
}

export interface SenioritySignal {
  signal_type: string;
  description: string;
  strength: number;
  context: string;
}

export interface CertificationCheck {
  certification_name: string;
  found: boolean;
  importance: number;
  expiry_status?: string;
  alternatives: string[];
  recommendation_reason: string;
}

export interface TrendAnalysis {
  trend_name: string;
  trend_type: string; // emerging, declining, stable, growing
  relevance_score: number;
  found_in_resume: boolean;
  importance_for_role: number;
  learning_resources: string[];
}

// ATS Testing Dashboard Types
export interface ValidationReport {
  overall_accuracy: number;
  per_ats_accuracy: Record<string, number>;
  format_detection_accuracy: number;
  parsing_simulation_accuracy: number;
  keyword_extraction_accuracy: number;
  improvement_suggestions: ImprovementSuggestion[];
  test_results: TestResult[];
  benchmark_comparison: BenchmarkComparison;
  confidence_score: number;
}

export interface ImprovementSuggestion {
  category: string;
  description: string;
  priority: string;
  implementation_effort: string;
  expected_improvement: number;
}

export interface TestResult {
  test_id: string;
  resume_type: string;
  test_category: string;
  expected_result: TestExpectation;
  actual_result: TestOutcome;
  accuracy_score: number;
  issues_found: string[];
  recommendations: string[];
  execution_time_ms: number;
}

export interface TestExpectation {
  format_score_range: [number, number];
  critical_issues_count: number;
  parsing_success_rate: number;
  keyword_detection_rate: number;
  ats_compatibility_scores: Record<string, number>;
}

export interface TestOutcome {
  format_score: number;
  critical_issues_count: number;
  parsing_success_rate: number;
  keyword_detection_rate: number;
  ats_compatibility_scores: Record<string, number>;
  processing_time_ms: number;
}

export interface BenchmarkComparison {
  baseline_accuracy: number;
  current_accuracy: number;
  improvement_percentage: number;
  performance_trend: string; // "improving", "stable", "declining"
  comparison_details: Record<string, number>;
}

export interface ATSSimulationResult {
  overall_ats_score: number;
  system_simulations: Record<string, ATSSystemResult>;
  parsing_analysis: ParsingAnalysis;
  keyword_extraction: KeywordExtractionResult;
  format_analysis: FormatAnalysis;
  optimization_recommendations: ATSOptimizationRecommendation[];
  compatibility_issues: CompatibilityIssue[];
}

export interface ATSSystemResult {
  system_name: string;
  compatibility_score: number;
  parsing_success_rate: number;
  extracted_sections: Record<string, ExtractionQuality>;
  keyword_detection_rate: number;
  format_compliance: FormatCompliance;
  specific_issues: string[];
  recommendations: string[];
}

export interface ParsingAnalysis {
  structure_clarity: number;
  section_detection: Record<string, boolean>;
  contact_info_extraction: ContactExtractionResult;
  work_experience_parsing: ExperienceParsingResult;
  education_parsing: EducationParsingResult;
  skills_parsing: SkillsParsingResult;
  formatting_issues: FormattingIssue[];
}

export interface KeywordExtractionResult {
  extraction_accuracy: number;
  keywords_found: ExtractedKeyword[];
  missed_keywords: string[];
  context_preservation: number;
  semantic_understanding: number;
}

export interface FormatAnalysis {
  file_format_compatibility: Record<string, boolean>;
  layout_complexity: number;
  font_compatibility: FontCompatibility;
  graphics_elements: GraphicsAnalysis;
  table_usage: TableAnalysis;
  line_spacing: number;
  margin_analysis: MarginAnalysis;
}

export interface ATSOptimizationRecommendation {
  category: string;
  priority: string; // critical, high, medium, low
  title: string;
  description: string;
  implementation_steps: string[];
  expected_improvement: number;
  affected_systems: string[];
  examples: string[];
}

export interface CompatibilityIssue {
  severity: string; // critical, major, minor, warning
  issue_type: string;
  description: string;
  affected_systems: string[];
  impact_score: number;
  resolution_difficulty: string; // easy, medium, hard
  fix_suggestions: string[];
}

export interface ExtractionQuality {
  accuracy: number;
  completeness: number;
  structure_preservation: number;
  issues: string[];
}

export interface FormatCompliance {
  meets_standards: boolean;
  compliance_score: number;
  violations: string[];
  recommendations: string[];
}

export interface ContactExtractionResult {
  email_detected: boolean;
  phone_detected: boolean;
  address_detected: boolean;
  linkedin_detected: boolean;
  extraction_confidence: number;
  formatting_issues: string[];
}

export interface ExperienceParsingResult {
  jobs_detected: number;
  date_parsing_accuracy: number;
  title_extraction_accuracy: number;
  company_extraction_accuracy: number;
  description_parsing_quality: number;
  chronological_order_detected: boolean;
  parsing_issues: string[];
}

export interface EducationParsingResult {
  institutions_detected: number;
  degree_extraction_accuracy: number;
  date_parsing_accuracy: number;
  gpa_detection: boolean;
  certification_detection: number;
  parsing_issues: string[];
}

export interface SkillsParsingResult {
  skills_detected: number;
  categorization_accuracy: number;
  technical_skills_ratio: number;
  soft_skills_ratio: number;
  skill_context_preservation: number;
  parsing_issues: string[];
}

export interface FormattingIssue {
  issue_type: string;
  description: string;
  severity: string;
  line_number?: number;
  suggestion: string;
}

export interface ExtractedKeyword {
  keyword: string;
  confidence: number;
  context: string;
  section: string;
  importance: number;
}

export interface FontCompatibility {
  standard_fonts_used: boolean;
  font_consistency: number;
  readability_score: number;
  problematic_fonts: string[];
}

export interface GraphicsAnalysis {
  has_graphics: boolean;
  graphics_compatibility: number;
  alt_text_present: boolean;
  graphics_impact: string; // positive, negative, neutral
  recommendations: string[];
}

export interface TableAnalysis {
  tables_detected: number;
  table_compatibility: number;
  parsing_difficulty: number;
  alternative_suggestions: string[];
}

export interface MarginAnalysis {
  margin_consistency: number;
  standard_margins: boolean;
  readability_impact: number;
  recommendations: string[];
}

// Comprehensive Analysis Types (Enhanced Multi-dimensional Analysis)
export interface EnhancedAnalysisResult {
  base_analysis: AnalysisResult;
  semantic_analysis: SemanticAnalysisResult;
  industry_analysis: IndustryAnalysisResult;
  ats_compatibility: ATSCompatibilityResult;
  scoring_breakdown: ScoringBreakdown;
  optimization_suggestions: OptimizationSuggestion[];
  benchmarks_comparison: BenchmarkAnalysis;
}

export interface ATSCompatibilityResult {
  overall_compatibility_score: number;
  system_specific_scores: Record<string, number>;
  format_issues: FormatIssue[];
  parsing_warnings: string[];
  ats_optimization_suggestions: string[];
}

export interface ScoringBreakdown {
  weighted_scores: Record<string, WeightedScore>;
  industry_adjustments: Record<string, number>;
  role_level_multipliers: Record<string, number>;
  final_calculations: FinalCalculations;
}

export interface WeightedScore {
  raw_score: number;
  weight: number;
  adjusted_score: number;
  explanation: string;
}

export interface FinalCalculations {
  base_score: number;
  industry_bonus: number;
  role_level_bonus: number;
  semantic_bonus: number;
  ats_penalty: number;
  final_score: number;
}

export interface OptimizationSuggestion {
  category: string;
  priority: string; // "high", "medium", "low"
  title: string;
  description: string;
  expected_impact: number;
  implementation_difficulty: string; // "easy", "medium", "hard"
  specific_actions: string[];
  ats_systems_helped: string[];
}

export interface BenchmarkAnalysis {
  industry_benchmark: number;
  role_level_benchmark: number;
  percentile_ranking: number;
  peer_comparison: PeerComparison;
  improvement_potential: number;
}

export interface PeerComparison {
  above_average_areas: string[];
  below_average_areas: string[];
  standout_strengths: string[];
  critical_gaps: string[];
}

// ==================== Competitive Analysis Types ====================

export interface CompetitiveAnalysis {
  market_position: MarketPosition;
  competitor_comparison: CompetitorComparison;
  skill_competitiveness: SkillCompetitiveness;
  salary_insights: SalaryInsights;
  hiring_probability: HiringProbability;
  market_trends: MarketTrends;
  competitive_intelligence: CompetitiveIntelligence;
  strategic_recommendations: StrategicRecommendations;
  generated_at: string;
}

export interface MarketPosition {
  percentile_ranking: number; // 0-100, where 100 is top 1%
  strength_areas: StrengthArea[];
  improvement_areas: ImprovementArea[];
  market_demand_score: number;
  competitive_advantages: CompetitiveAdvantage[];
  positioning_statement: string;
  market_segment: MarketSegment;
}

export interface StrengthArea {
  area: string;
  score: number;
  market_percentile: number;
  relative_to_competition: string; // "significantly above", "above", "at par", "below"
  supporting_evidence: string[];
  leverage_opportunities: string[];
}

export interface ImprovementArea {
  area: string;
  current_score: number;
  target_score: number;
  market_impact: number;
  improvement_timeline: string;
  required_actions: string[];
  success_metrics: string[];
}

export interface CompetitiveAdvantage {
  advantage: string;
  strength_level: string; // "strong", "moderate", "weak"
  market_rarity: number; // percentage of candidates who have this
  value_to_employers: number;
  sustainability: string; // how long this advantage will last
  amplification_strategies: string[];
}

export interface MarketSegment {
  segment_name: string;
  target_companies: string[];
  typical_requirements: string[];
  competitive_landscape: string;
  growth_potential: number;
  entry_barriers: string[];
}

export interface CompetitorComparison {
  peer_analysis: PeerAnalysis;
  benchmark_comparisons: CompetitiveBenchmarkComparison[];
  competitive_gaps: CompetitiveGap[];
  differentiation_opportunities: DifferentiationOpportunity[];
  market_positioning_matrix: PositioningMatrix;
}

export interface PeerAnalysis {
  peer_group_definition: string;
  sample_size: number;
  user_ranking: number; // position within peer group
  peer_group_stats: PeerGroupStats;
  performance_vs_peers: PerformanceMetric[];
}

export interface PeerGroupStats {
  average_score: number;
  median_score: number;
  top_quartile_threshold: number;
  bottom_quartile_threshold: number;
  standard_deviation: number;
}

export interface PerformanceMetric {
  metric_name: string;
  user_score: number;
  peer_average: number;
  peer_top_10_percent: number;
  relative_performance: string; // "outperforming", "on par", "underperforming"
  improvement_potential: number;
}

export interface CompetitiveBenchmarkComparison {
  benchmark_type: string; // "industry", "role_level", "geographic", "company_size"
  benchmark_value: number;
  user_value: number;
  variance: number; // positive = above benchmark, negative = below
  significance: string; // "highly significant", "significant", "not significant"
  context: string;
}

export interface CompetitiveGap {
  gap_area: string;
  gap_size: string; // "large", "medium", "small"
  market_impact: number;
  urgency: string; // "critical", "high", "medium", "low"
  closing_strategy: string;
  estimated_effort: string;
  expected_roi: number;
}

export interface DifferentiationOpportunity {
  opportunity: string;
  uniqueness_score: number; // how unique this would make the candidate
  market_value: number;
  implementation_difficulty: string;
  success_probability: number;
  strategic_value: string;
}

export interface PositioningMatrix {
  x_axis: string; // e.g., "Technical Skills"
  y_axis: string; // e.g., "Leadership Experience"
  user_position: [number, number];
  competitor_positions: CompetitorPosition[];
  market_quadrants: MarketQuadrant[];
  optimal_positioning: [number, number];
}

export interface CompetitorPosition {
  competitor_id: string;
  position: [number, number];
  market_share: number;
  success_rate: number;
}

export interface MarketQuadrant {
  quadrant_name: string;
  description: string;
  opportunity_level: string;
  competition_level: string;
  strategic_advice: string;
}

export interface SkillCompetitiveness {
  skill_portfolio_analysis: SkillPortfolioAnalysis;
  skill_market_positioning: SkillMarketPosition[];
  skill_gap_competitiveness: SkillGapCompetitive[];
  emerging_skill_opportunities: EmergingSkillOpportunity[];
  skill_combination_advantages: SkillCombinationAdvantage[];
}

export interface SkillPortfolioAnalysis {
  portfolio_strength: number;
  portfolio_diversity: number;
  portfolio_uniqueness: number;
  market_alignment: number;
  future_readiness: number;
  skill_categories: SkillCategory[];
}

export interface SkillCategory {
  category: string;
  skill_count: number;
  average_proficiency: number;
  market_demand: number;
  competitive_strength: string;
  development_priority: string;
}

export interface SkillMarketPosition {
  skill: string;
  user_proficiency: number;
  market_demand: number;
  supply_scarcity: number; // how rare this skill is
  competitive_position: string; // "leader", "strong", "average", "weak"
  market_trends: string;
  monetization_potential: number;
}

export interface SkillGapCompetitive {
  skill: string;
  gap_vs_market: number;
  competitive_impact: number;
  acquisition_priority: string;
  competitive_advantage_potential: number;
  market_window: string; // time window for opportunity
}

export interface EmergingSkillOpportunity {
  skill: string;
  emergence_timeline: string;
  adoption_rate: number;
  first_mover_advantage: number;
  market_potential: number;
  learning_curve: string;
  investment_recommendation: string;
}

export interface SkillCombinationAdvantage {
  skill_combination: string[];
  synergy_score: number;
  market_rarity: number;
  value_multiplier: number; // how much this combination increases value
  target_roles: string[];
  development_strategy: string;
}

export interface SalaryInsights {
  competitive_salary_analysis: CompetitiveSalaryAnalysis;
  negotiation_positioning: NegotiationPositioning;
  compensation_benchmarking: CompensationBenchmarking;
  total_compensation_analysis: TotalCompensationAnalysis;
}

export interface CompetitiveSalaryAnalysis {
  market_percentile: number;
  vs_peer_group: number; // percentage difference from peer average
  salary_potential: SalaryPotential;
  geographic_competitiveness: GeographicSalaryComparison[];
  industry_competitiveness: IndustrySalaryComparison[];
}

export interface SalaryPotential {
  current_estimated: number;
  short_term_potential: number; // 6-12 months
  medium_term_potential: number; // 1-3 years
  long_term_potential: number; // 3-5 years
  ceiling_estimate: number;
  growth_trajectory: string;
}

export interface GeographicSalaryComparison {
  location: string;
  salary_premium: number; // percentage above/below user's location
  cost_of_living_adjusted: number;
  purchasing_power: number;
  career_opportunity_score: number;
  relocation_recommendation: string;
}

export interface IndustrySalaryComparison {
  industry: string;
  salary_premium: number;
  growth_potential: number;
  skill_transferability: number;
  transition_difficulty: string;
  transition_recommendation: string;
}

export interface NegotiationPositioning {
  negotiation_strength: number;
  leverage_factors: LeverageFactor[];
  market_timing: string; // "excellent", "good", "fair", "poor"
  negotiation_strategies: NegotiationStrategy[];
  risk_assessment: NegotiationRiskAssessment;
}

export interface LeverageFactor {
  factor: string;
  strength: number;
  market_rarity: number;
  employer_value: number;
  negotiation_power: string;
}

export interface NegotiationStrategy {
  strategy: string;
  success_probability: number;
  risk_level: string;
  potential_upside: number;
  implementation_tips: string[];
}

export interface NegotiationRiskAssessment {
  overall_risk: string;
  market_risks: string[];
  timing_risks: string[];
  competitive_risks: string[];
  mitigation_strategies: string[];
}

export interface CompensationBenchmarking {
  total_compensation_percentile: number;
  base_salary_comparison: number;
  bonus_potential_comparison: number;
  equity_comparison: number;
  benefits_comparison: number;
  compensation_structure_optimization: OptimizationRecommendation[];
}

export interface OptimizationRecommendation {
  component: string; // "base", "bonus", "equity", "benefits"
  current_positioning: string;
  optimization_opportunity: number;
  negotiation_approach: string;
  market_precedent: string;
}

export interface TotalCompensationAnalysis {
  current_total_comp: number;
  market_total_comp: number;
  optimization_potential: number;
  component_breakdown: Record<string, CompensationComponent>;
  optimization_priorities: string[];
}

export interface CompensationComponent {
  current_value: number;
  market_value: number;
  optimization_potential: number;
  negotiation_difficulty: string;
}

export interface HiringProbability {
  overall_probability: number;
  probability_by_company_type: CompanyTypeProbability[];
  probability_by_role_level: RoleLevelProbability[];
  probability_factors: ProbabilityFactor[];
  success_scenarios: SuccessScenario[];
  improvement_impact: ImprovementImpact[];
}

export interface CompanyTypeProbability {
  company_type: string; // "startup", "mid-size", "enterprise", "faang"
  probability: number;
  match_strength: number;
  key_factors: string[];
  improvement_suggestions: string[];
}

export interface RoleLevelProbability {
  role_level: string;
  probability: number;
  readiness_score: number;
  gap_analysis: string[];
  preparation_timeline: string;
}

export interface ProbabilityFactor {
  factor: string;
  current_impact: number; // positive or negative contribution
  improvement_potential: number;
  controllability: string; // "high", "medium", "low"
  timeline_to_improve: string;
}

export interface SuccessScenario {
  scenario_name: string;
  probability: number;
  required_conditions: string[];
  success_indicators: string[];
  timeline: string;
}

export interface ImprovementImpact {
  improvement: string;
  current_probability: number;
  improved_probability: number;
  probability_increase: number;
  implementation_effort: string;
  roi_score: number;
}

export interface MarketTrends {
  hiring_trends: HiringTrendAnalysis;
  skill_trends: SkillTrendAnalysis;
  compensation_trends: CompensationTrendAnalysis;
  geographic_trends: GeographicTrendAnalysis;
  industry_disruption: IndustryDisruptionAnalysis;
}

export interface HiringTrendAnalysis {
  overall_market_direction: string; // "expanding", "contracting", "stable"
  hiring_velocity_trend: number; // change in hiring speed
  demand_supply_ratio: number;
  seasonal_patterns: SeasonalPattern[];
  future_outlook: string;
}

export interface SeasonalPattern {
  period: string; // "Q1", "Q2", etc.
  hiring_intensity: number;
  opportunity_level: string;
  strategic_timing: string;
}

export interface SkillTrendAnalysis {
  trending_skills: TrendingSkillAnalysis[];
  declining_skills: DecliningSkillAnalysis[];
  skill_evolution_patterns: SkillEvolutionPattern[];
  cross_industry_transfers: CrossIndustryTransfer[];
}

export interface TrendingSkillAnalysis {
  skill: string;
  growth_rate: number;
  adoption_velocity: number;
  market_penetration: number;
  competitive_advantage_window: string;
  learning_investment_recommendation: string;
}

export interface DecliningSkillAnalysis {
  skill: string;
  decline_rate: number;
  obsolescence_timeline: string;
  transition_pathways: string[];
  preservation_strategy?: string;
}

export interface SkillEvolutionPattern {
  base_skill: string;
  evolved_skills: string[];
  evolution_timeline: string;
  transition_difficulty: string;
  market_opportunity: number;
}

export interface CrossIndustryTransfer {
  from_industry: string;
  to_industry: string;
  transferable_skills: string[];
  success_rate: number;
  salary_impact: number;
  transition_timeline: string;
}

export interface CompensationTrendAnalysis {
  salary_growth_trends: SalaryGrowthTrend[];
  compensation_structure_shifts: CompensationShift[];
  regional_compensation_changes: RegionalCompensationChange[];
  total_compensation_evolution: TotalCompensationEvolution;
}

export interface SalaryGrowthTrend {
  skill_category: string;
  annual_growth_rate: number;
  market_drivers: string[];
  sustainability: string;
  projection: number; // 2-year projection
}

export interface CompensationShift {
  shift_type: string; // "base_to_variable", "cash_to_equity", etc.
  trend_strength: number;
  affected_roles: string[];
  strategic_implications: string;
}

export interface RegionalCompensationChange {
  region: string;
  compensation_change: number;
  driving_factors: string[];
  competitive_impact: string;
}

export interface TotalCompensationEvolution {
  traditional_vs_modern: CompensationComparison;
  emerging_benefits: EmergingBenefit[];
  value_proposition_shifts: ValuePropositionShift[];
}

export interface CompensationComparison {
  traditional_weight: number;
  modern_weight: number;
  employee_preference_shift: number;
  employer_adoption_rate: number;
}

export interface EmergingBenefit {
  benefit_type: string;
  adoption_rate: number;
  employee_value_rating: number;
  cost_to_employer: string;
  negotiation_potential: number;
}

export interface ValuePropositionShift {
  from_value: string;
  to_value: string;
  shift_velocity: number;
  adoption_timeline: string;
  strategic_importance: string;
}

export interface GeographicTrendAnalysis {
  regional_growth_patterns: RegionalGrowthPattern[];
  talent_migration_trends: TalentMigrationTrend[];
  cost_of_living_impacts: CostOfLivingImpact[];
  remote_work_implications: RemoteWorkImplication[];
}

export interface RegionalGrowthPattern {
  region: string;
  growth_rate: number;
  driving_industries: string[];
  talent_demand: number;
  competitive_landscape: string;
}

export interface TalentMigrationTrend {
  from_region: string;
  to_region: string;
  migration_volume: number;
  driving_factors: string[];
  impact_on_compensation: number;
}

export interface CostOfLivingImpact {
  region: string;
  cost_index: number;
  purchasing_power_change: number;
  career_value_proposition: string;
}

export interface RemoteWorkImplication {
  work_arrangement: string;
  adoption_rate: number;
  salary_impact: number;
  geographic_arbitrage_opportunity: number;
  competitive_positioning: string;
}

export interface IndustryDisruptionAnalysis {
  disruption_indicators: DisruptionIndicator[];
  transformation_timeline: TransformationTimeline;
  skill_evolution_requirements: SkillEvolutionRequirement[];
  career_resilience_factors: CareerResilienceFactor[];
}

export interface DisruptionIndicator {
  indicator_type: string;
  strength: number;
  timeline: string;
  affected_roles: string[];
  mitigation_strategies: string[];
}

export interface TransformationTimeline {
  current_phase: string;
  next_phase: string;
  transition_timeline: string;
  preparation_requirements: string[];
}

export interface SkillEvolutionRequirement {
  current_skill: string;
  evolved_skill: string;
  transition_urgency: string;
  learning_path: string[];
  market_readiness_timeline: string;
}

export interface CareerResilienceFactor {
  factor: string;
  importance: number;
  current_strength: number;
  development_strategy: string;
  future_relevance: string;
}

export interface CompetitiveIntelligence {
  market_intelligence: MarketIntelligence;
  competitive_landscape: CompetitiveLandscape;
  opportunity_mapping: OpportunityMapping;
  threat_assessment: ThreatAssessment;
}

export interface MarketIntelligence {
  market_size: number;
  growth_rate: number;
  key_players: KeyPlayer[];
  market_dynamics: MarketDynamic[];
  entry_barriers: string[];
}

export interface KeyPlayer {
  player_name: string;
  market_share: number;
  competitive_advantages: string[];
  vulnerabilities: string[];
  strategic_direction: string;
}

export interface MarketDynamic {
  dynamic_type: string;
  impact_level: string;
  timeline: string;
  strategic_implications: string[];
}

export interface CompetitiveLandscape {
  competitive_intensity: number;
  market_concentration: number;
  differentiation_factors: string[];
  competitive_moats: CompetitiveMoat[];
}

export interface CompetitiveMoat {
  moat_type: string;
  strength: number;
  sustainability: string;
  replication_difficulty: string;
}

export interface OpportunityMapping {
  market_gaps: MarketGap[];
  emerging_opportunities: EmergingOpportunity[];
  optimization_opportunities: OptimizationOpportunity[];
}

export interface MarketGap {
  gap_description: string;
  market_size: number;
  difficulty_to_address: string;
  potential_roi: number;
  timeline_to_capture: string;
}

export interface EmergingOpportunity {
  opportunity_description: string;
  market_potential: number;
  competitive_window: string;
  required_capabilities: string[];
  success_probability: number;
}

export interface OptimizationOpportunity {
  area: string;
  current_performance: number;
  optimization_potential: number;
  implementation_complexity: string;
  expected_impact: number;
}

export interface ThreatAssessment {
  competitive_threats: CompetitiveThreat[];
  market_threats: MarketThreat[];
  technology_threats: TechnologyThreat[];
  regulatory_threats: RegulatoryThreat[];
}

export interface CompetitiveThreat {
  threat_source: string;
  threat_level: string;
  probability: number;
  potential_impact: number;
  mitigation_strategies: string[];
}

export interface MarketThreat {
  threat_type: string;
  severity: string;
  timeline: string;
  affected_segments: string[];
  defensive_strategies: string[];
}

export interface TechnologyThreat {
  technology: string;
  disruption_potential: number;
  adoption_timeline: string;
  affected_skills: string[];
  adaptation_strategies: string[];
}

export interface RegulatoryThreat {
  regulation_type: string;
  impact_assessment: string;
  compliance_requirements: string[];
  strategic_response: string[];
}

export interface StrategicRecommendations {
  short_term_strategies: StrategicRecommendation[];
  medium_term_strategies: StrategicRecommendation[];
  long_term_strategies: StrategicRecommendation[];
  contingency_plans: ContingencyPlan[];
}

export interface StrategicRecommendation {
  strategy: string;
  priority: string;
  expected_impact: number;
  implementation_timeline: string;
  resource_requirements: string[];
  success_metrics: string[];
  risk_factors: string[];
}

export interface ContingencyPlan {
  scenario: string;
  probability: number;
  triggers: string[];
  response_strategy: string;
  preparation_requirements: string[];
}

// Extended types for additional competitive analysis responses
export interface MarketPositionAnalysis {
  market_position: MarketPosition;
  competitive_analysis: CompetitiveAnalysis;
  strategic_insights: unknown;
}

export interface SalaryInsightsResponse {
  competitive_salary_analysis: CompetitiveSalaryAnalysis;
  negotiation_positioning: NegotiationPositioning;
  compensation_benchmarking: CompensationBenchmarking;
  total_compensation_analysis: TotalCompensationAnalysis;
}

export interface HiringProbabilityResponse {
  overall_probability: number;
  probability_by_company_type: CompanyTypeProbability[];
  probability_by_role_level: RoleLevelProbability[];
  probability_factors: ProbabilityFactor[];
  success_scenarios: SuccessScenario[];
  improvement_impact: ImprovementImpact[];
}

// Job Description Management Types
export interface JobDescription {
  id: string;
  title: string;
  company: string;
  content: string;
  requirements: string; // JSON string array
  preferred_qualifications?: string; // JSON string array
  salary_range_min?: number;
  salary_range_max?: number;
  salary_currency?: string;
  location: string;
  remote_options: RemoteWorkType;
  employment_type: EmploymentType;
  experience_level: ExperienceLevel;
  posted_date?: string; // ISO string
  application_deadline?: string; // ISO string
  job_url?: string;
  keywords: string; // JSON string array
  industry?: string;
  department?: string;
  status: JobStatus;
  priority: JobPriority;
  notes?: string;
  application_status: ApplicationStatus;
  application_date?: string; // ISO string
  interview_date?: string; // ISO string
  response_deadline?: string; // ISO string
  contact_person?: string;
  contact_email?: string;
  tags: string; // JSON string array
  source: JobSource;
  is_archived: boolean;
  created_at: string; // ISO string
  updated_at: string; // ISO string
}

export type RemoteWorkType = 'OnSite' | 'Remote' | 'Hybrid' | 'Flexible';

export type EmploymentType =
  | 'FullTime'
  | 'PartTime'
  | 'Contract'
  | 'Temporary'
  | 'Internship'
  | 'Freelance';

export type ExperienceLevel =
  | 'EntryLevel'
  | 'Junior'
  | 'MidLevel'
  | 'Senior'
  | 'Lead'
  | 'Principal'
  | 'Executive';

export type JobStatus =
  | 'Draft'
  | 'Active'
  | 'Applied'
  | 'Interviewing'
  | 'Offered'
  | 'Rejected'
  | 'Withdrawn'
  | 'Closed';

export type JobPriority = 'Low' | 'Medium' | 'High' | 'Critical';

export type ApplicationStatus =
  | 'NotApplied'
  | 'Applied'
  | 'ApplicationReviewed'
  | 'PhoneScreen'
  | 'TechnicalInterview'
  | 'OnSiteInterview'
  | 'FinalRound'
  | 'OfferReceived'
  | 'OfferAccepted'
  | 'OfferDeclined'
  | 'Rejected'
  | 'Withdrawn';

export type JobSource =
  | 'Manual'
  | 'LinkedIn'
  | 'Indeed'
  | 'CompanyWebsite'
  | 'Referral'
  | 'Recruiter'
  | 'JobBoard'
  | 'URL';

export interface JobSearchRequest {
  query?: string;
  company?: string;
  location?: string;
  remote_options?: RemoteWorkType[];
  employment_type?: EmploymentType[];
  experience_level?: ExperienceLevel[];
  salary_min?: number;
  salary_max?: number;
  status?: JobStatus[];
  priority?: JobPriority[];
  application_status?: ApplicationStatus[];
  industry?: string;
  tags?: string[];
  posted_after?: string; // ISO string
  posted_before?: string; // ISO string
  include_archived?: boolean;
  limit?: number;
  offset?: number;
  sort_by?: JobSortOption;
  sort_order?: SortOrder;
}

export type JobSortOption =
  | 'CreatedAt'
  | 'UpdatedAt'
  | 'PostedDate'
  | 'ApplicationDeadline'
  | 'Priority'
  | 'Title'
  | 'Company'
  | 'SalaryMin'
  | 'SalaryMax';

export type SortOrder = 'Asc' | 'Desc';

export interface JobSearchResult {
  jobs: JobDescription[];
  total_count: number;
  has_more: boolean;
}

export interface JobAnalytics {
  total_jobs: number;
  jobs_by_status: JobStatusCount[];
  jobs_by_priority: JobPriorityCount[];
  jobs_by_application_status: ApplicationStatusCount[];
  average_salary_range?: SalaryRangeStats;
  top_companies: CompanyCount[];
  top_locations: LocationCount[];
  application_timeline: ApplicationTimelineEntry[];
  success_rate: number;
  response_rate: number;
}

export interface JobStatusCount {
  status: JobStatus;
  count: number;
}

export interface JobPriorityCount {
  priority: JobPriority;
  count: number;
}

export interface ApplicationStatusCount {
  status: ApplicationStatus;
  count: number;
}

export interface SalaryRangeStats {
  min_avg: number;
  max_avg: number;
  median_min: number;
  median_max: number;
}

export interface CompanyCount {
  company: string;
  count: number;
}

export interface LocationCount {
  location: string;
  count: number;
}

export interface ApplicationTimelineEntry {
  date: string; // ISO string
  applications_count: number;
  responses_count: number;
}

export interface JobUrlExtractionRequest {
  url: string;
}

export interface JobUrlExtractionResult {
  title?: string;
  company?: string;
  content: string;
  location?: string;
  salary_range?: string;
  employment_type?: string;
  remote_options?: string;
  requirements: string[];
  posted_date?: string;
  application_deadline?: string;
  success: boolean;
  error?: string;
}

export interface JobComparisonRequest {
  job_ids: string[];
}

export interface JobComparisonResult {
  jobs: JobDescription[];
  comparison_matrix: JobComparisonMatrix;
}

export interface JobComparisonMatrix {
  salary_comparison: SalaryComparison[];
  location_comparison: LocationComparison[];
  requirements_comparison: RequirementsComparison;
  benefits_comparison: BenefitsComparison[];
  match_scores: JobMatchScore[];
}

export interface SalaryComparison {
  job_id: string;
  min_salary?: number;
  max_salary?: number;
  currency?: string;
  vs_average?: number;
}

export interface LocationComparison {
  job_id: string;
  location: string;
  remote_options: RemoteWorkType;
  commute_score?: number;
}

export interface RequirementsComparison {
  common_requirements: string[];
  unique_requirements: JobUniqueRequirements[];
}

export interface JobUniqueRequirements {
  job_id: string;
  requirements: string[];
}

export interface BenefitsComparison {
  job_id: string;
  benefits: string[];
}

export interface JobMatchScore {
  job_id: string;
  match_score: number;
  match_factors: MatchFactor[];
}

export interface MatchFactor {
  factor: string;
  score: number;
  weight: number;
  explanation: string;
}
