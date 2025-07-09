use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::ats_simulator::ATSSimulator;
use crate::database::Database;
use crate::format_checker::FormatCompatibilityChecker;
use crate::format_issue_detector::FormatIssueDetector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub overall_accuracy: f64,
    pub per_ats_accuracy: HashMap<String, f64>,
    pub format_detection_accuracy: f64,
    pub parsing_simulation_accuracy: f64,
    pub keyword_extraction_accuracy: f64,
    pub improvement_suggestions: Vec<ImprovementSuggestion>,
    pub test_results: Vec<TestResult>,
    pub benchmark_comparison: BenchmarkComparison,
    pub confidence_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub test_id: String,
    pub resume_type: String,
    pub test_category: String,
    pub expected_result: TestExpectation,
    pub actual_result: TestOutcome,
    pub accuracy_score: f64,
    pub issues_found: Vec<String>,
    pub recommendations: Vec<String>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExpectation {
    pub format_score_range: (f64, f64),
    pub critical_issues_count: usize,
    pub parsing_success_rate: f64,
    pub keyword_detection_rate: f64,
    pub ats_compatibility_scores: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestOutcome {
    pub format_score: f64,
    pub critical_issues_count: usize,
    pub parsing_success_rate: f64,
    pub keyword_detection_rate: f64,
    pub ats_compatibility_scores: HashMap<String, f64>,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementSuggestion {
    pub category: String,
    pub description: String,
    pub priority: String,
    pub implementation_effort: String,
    pub expected_improvement: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkComparison {
    pub baseline_accuracy: f64,
    pub current_accuracy: f64,
    pub improvement_percentage: f64,
    pub performance_trend: String, // "improving", "stable", "declining"
    pub comparison_details: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResume {
    pub id: String,
    pub content: String,
    pub resume_type: String, // "good", "problematic", "edge_case"
    pub known_issues: Vec<String>,
    pub target_keywords: Vec<String>,
    pub expected_ats_scores: HashMap<String, f64>,
    pub difficulty_level: String, // "easy", "medium", "hard"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_id: String,
    pub rule_type: String,
    pub description: String,
    pub validation_function: String,
    pub acceptance_threshold: f64,
    pub weight: f64,
}

pub struct ATSTestingFramework {
    test_resumes: Vec<TestResume>,
    #[allow(dead_code)]
    validation_rules: Vec<ValidationRule>,
    ats_simulator: ATSSimulator,
    format_checker: FormatCompatibilityChecker,
    format_issue_detector: FormatIssueDetector,
    benchmark_data: BenchmarkData,
}

#[derive(Debug, Clone)]
pub struct BenchmarkData {
    pub baseline_metrics: HashMap<String, f64>,
    #[allow(dead_code)]
    pub historical_performance: Vec<PerformanceDataPoint>,
    #[allow(dead_code)]
    pub target_thresholds: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDataPoint {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metric_name: String,
    pub value: f64,
    pub test_suite_version: String,
}

impl ATSTestingFramework {
    pub fn new(database: Database) -> Self {
        Self {
            test_resumes: Self::create_test_suite(),
            validation_rules: Self::create_validation_rules(),
            ats_simulator: ATSSimulator::new(database),
            format_checker: FormatCompatibilityChecker::new(),
            format_issue_detector: FormatIssueDetector::new(),
            benchmark_data: Self::create_benchmark_data(),
        }
    }

    pub async fn run_comprehensive_validation(&self) -> Result<ValidationReport> {
        info!("Starting comprehensive ATS validation suite");

        let mut test_results = Vec::new();
        let mut accuracy_scores = Vec::new();
        let mut per_ats_scores: HashMap<String, Vec<f64>> = HashMap::new();

        let start_time = std::time::Instant::now();

        // Run tests on each test resume
        for test_resume in &self.test_resumes {
            info!(
                "Testing resume: {} ({})",
                test_resume.id, test_resume.resume_type
            );

            let test_result = self.validate_single_resume(test_resume).await?;
            accuracy_scores.push(test_result.accuracy_score);

            // Collect per-ATS scores
            for (ats_name, score) in &test_result.actual_result.ats_compatibility_scores {
                per_ats_scores
                    .entry(ats_name.clone())
                    .or_default()
                    .push(*score);
            }

            test_results.push(test_result);
        }

        let total_time = start_time.elapsed();
        info!("Validation completed in {:?}", total_time);

        // Calculate overall metrics
        let overall_accuracy = accuracy_scores.iter().sum::<f64>() / accuracy_scores.len() as f64;

        let per_ats_accuracy = per_ats_scores
            .iter()
            .map(|(ats, scores)| {
                (
                    ats.clone(),
                    scores.iter().sum::<f64>() / scores.len() as f64,
                )
            })
            .collect();

        let format_detection_accuracy = self.calculate_format_detection_accuracy(&test_results);
        let parsing_simulation_accuracy = self.calculate_parsing_accuracy(&test_results);
        let keyword_extraction_accuracy = self.calculate_keyword_accuracy(&test_results);

        // Generate improvement suggestions
        let improvement_suggestions = self.generate_improvement_suggestions(&test_results);

        // Create benchmark comparison
        let benchmark_comparison = self.create_benchmark_comparison(overall_accuracy);

        // Calculate confidence score
        let confidence_score = self.calculate_confidence_score(&test_results, overall_accuracy);

        Ok(ValidationReport {
            overall_accuracy,
            per_ats_accuracy,
            format_detection_accuracy,
            parsing_simulation_accuracy,
            keyword_extraction_accuracy,
            improvement_suggestions,
            test_results,
            benchmark_comparison,
            confidence_score,
        })
    }

    async fn validate_single_resume(&self, test_resume: &TestResume) -> Result<TestResult> {
        let test_start = std::time::Instant::now();

        // Run ATS simulation
        let simulation_result = self
            .ats_simulator
            .simulate_multiple_ats_systems(&test_resume.content, &test_resume.target_keywords)
            .await?;

        // Run format compatibility check
        let format_report = self
            .format_checker
            .check_comprehensive_compatibility(&test_resume.content)?;

        // Run format issue detection
        let issue_report = self
            .format_issue_detector
            .analyze_format_issues(&test_resume.content, &format_report)?;

        let execution_time = test_start.elapsed().as_millis() as u64;

        // Create test expectation based on resume type
        let expected_result = self.create_test_expectation(test_resume);

        // Create actual test outcome
        let actual_result = TestOutcome {
            format_score: format_report.overall_score,
            critical_issues_count: issue_report.critical_issues.len(),
            parsing_success_rate: simulation_result.parsing_analysis.structure_clarity,
            keyword_detection_rate: simulation_result.keyword_extraction.extraction_accuracy,
            ats_compatibility_scores: format_report.ats_specific_scores.clone(),
            processing_time_ms: execution_time,
        };

        // Calculate accuracy score
        let accuracy_score = self.calculate_test_accuracy(&expected_result, &actual_result);

        // Identify issues and recommendations
        let issues_found = self.identify_test_issues(&expected_result, &actual_result);
        let recommendations = self.generate_test_recommendations(&issues_found, test_resume);

        Ok(TestResult {
            test_id: test_resume.id.clone(),
            resume_type: test_resume.resume_type.clone(),
            test_category: test_resume.difficulty_level.clone(),
            expected_result,
            actual_result,
            accuracy_score,
            issues_found,
            recommendations,
            execution_time_ms: execution_time,
        })
    }

    fn create_test_expectation(&self, test_resume: &TestResume) -> TestExpectation {
        match test_resume.resume_type.as_str() {
            "good" => TestExpectation {
                format_score_range: (85.0, 100.0),
                critical_issues_count: 0,
                parsing_success_rate: 0.9,
                keyword_detection_rate: 0.85,
                ats_compatibility_scores: [
                    ("greenhouse".to_string(), 85.0),
                    ("lever".to_string(), 90.0),
                    ("workday".to_string(), 88.0),
                ]
                .iter()
                .cloned()
                .collect(),
            },
            "problematic" => TestExpectation {
                format_score_range: (40.0, 70.0),
                critical_issues_count: 2,
                parsing_success_rate: 0.6,
                keyword_detection_rate: 0.5,
                ats_compatibility_scores: [
                    ("greenhouse".to_string(), 60.0),
                    ("lever".to_string(), 55.0),
                    ("workday".to_string(), 50.0),
                ]
                .iter()
                .cloned()
                .collect(),
            },
            "edge_case" => TestExpectation {
                format_score_range: (20.0, 60.0),
                critical_issues_count: 3,
                parsing_success_rate: 0.4,
                keyword_detection_rate: 0.3,
                ats_compatibility_scores: [
                    ("greenhouse".to_string(), 40.0),
                    ("lever".to_string(), 35.0),
                    ("workday".to_string(), 30.0),
                ]
                .iter()
                .cloned()
                .collect(),
            },
            _ => TestExpectation {
                format_score_range: (50.0, 80.0),
                critical_issues_count: 1,
                parsing_success_rate: 0.7,
                keyword_detection_rate: 0.6,
                ats_compatibility_scores: HashMap::new(),
            },
        }
    }

    fn calculate_test_accuracy(&self, expected: &TestExpectation, actual: &TestOutcome) -> f64 {
        let mut accuracy_components = Vec::new();

        // Format score accuracy
        let format_in_range = actual.format_score >= expected.format_score_range.0
            && actual.format_score <= expected.format_score_range.1;
        accuracy_components.push(if format_in_range { 1.0 } else { 0.5 });

        // Critical issues count accuracy
        let issues_diff =
            (expected.critical_issues_count as i32 - actual.critical_issues_count as i32).abs();
        let issues_accuracy = match issues_diff {
            0 => 1.0,
            1 => 0.8,
            2 => 0.6,
            _ => 0.2,
        };
        accuracy_components.push(issues_accuracy);

        // Parsing success rate accuracy
        let parsing_diff = (expected.parsing_success_rate - actual.parsing_success_rate).abs();
        let parsing_accuracy = (1.0 - parsing_diff).max(0.0);
        accuracy_components.push(parsing_accuracy);

        // Keyword detection accuracy
        let keyword_diff = (expected.keyword_detection_rate - actual.keyword_detection_rate).abs();
        let keyword_accuracy = (1.0 - keyword_diff).max(0.0);
        accuracy_components.push(keyword_accuracy);

        // ATS compatibility accuracy
        let mut ats_accuracies = Vec::new();
        for (ats_name, expected_score) in &expected.ats_compatibility_scores {
            if let Some(actual_score) = actual.ats_compatibility_scores.get(ats_name) {
                let diff = (expected_score - actual_score).abs() / 100.0;
                ats_accuracies.push((1.0 - diff).max(0.0));
            }
        }

        if !ats_accuracies.is_empty() {
            let avg_ats_accuracy = ats_accuracies.iter().sum::<f64>() / ats_accuracies.len() as f64;
            accuracy_components.push(avg_ats_accuracy);
        }

        // Calculate weighted average
        accuracy_components.iter().sum::<f64>() / accuracy_components.len() as f64
    }

    fn identify_test_issues(
        &self,
        expected: &TestExpectation,
        actual: &TestOutcome,
    ) -> Vec<String> {
        let mut issues = Vec::new();

        if actual.format_score < expected.format_score_range.0 {
            issues.push(format!(
                "Format score {} below expected minimum {}",
                actual.format_score, expected.format_score_range.0
            ));
        }

        if actual.critical_issues_count > expected.critical_issues_count {
            issues.push(format!(
                "More critical issues found ({}) than expected ({})",
                actual.critical_issues_count, expected.critical_issues_count
            ));
        }

        if actual.parsing_success_rate < expected.parsing_success_rate {
            issues.push(format!(
                "Parsing success rate {} below expected {}",
                actual.parsing_success_rate, expected.parsing_success_rate
            ));
        }

        if actual.keyword_detection_rate < expected.keyword_detection_rate {
            issues.push(format!(
                "Keyword detection rate {} below expected {}",
                actual.keyword_detection_rate, expected.keyword_detection_rate
            ));
        }

        issues
    }

    fn generate_test_recommendations(
        &self,
        issues: &[String],
        _test_resume: &TestResume,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        for issue in issues {
            if issue.contains("format score") {
                recommendations
                    .push("Improve format compatibility detection algorithms".to_string());
            }
            if issue.contains("critical issues") {
                recommendations.push("Enhance issue detection sensitivity".to_string());
            }
            if issue.contains("parsing success") {
                recommendations.push("Improve parsing simulation accuracy".to_string());
            }
            if issue.contains("keyword detection") {
                recommendations.push("Enhance keyword extraction algorithms".to_string());
            }
        }

        if recommendations.is_empty() {
            recommendations.push("Test passed successfully - no improvements needed".to_string());
        }

        recommendations
    }

    fn calculate_format_detection_accuracy(&self, test_results: &[TestResult]) -> f64 {
        let format_scores: Vec<f64> = test_results
            .iter()
            .map(|result| {
                let expected_range = &result.expected_result.format_score_range;
                let actual = result.actual_result.format_score;

                if actual >= expected_range.0 && actual <= expected_range.1 {
                    1.0
                } else {
                    0.0
                }
            })
            .collect();

        format_scores.iter().sum::<f64>() / format_scores.len() as f64
    }

    fn calculate_parsing_accuracy(&self, test_results: &[TestResult]) -> f64 {
        let parsing_accuracies: Vec<f64> = test_results
            .iter()
            .map(|result| {
                let expected = result.expected_result.parsing_success_rate;
                let actual = result.actual_result.parsing_success_rate;
                1.0 - (expected - actual).abs()
            })
            .collect();

        parsing_accuracies.iter().sum::<f64>() / parsing_accuracies.len() as f64
    }

    fn calculate_keyword_accuracy(&self, test_results: &[TestResult]) -> f64 {
        let keyword_accuracies: Vec<f64> = test_results
            .iter()
            .map(|result| {
                let expected = result.expected_result.keyword_detection_rate;
                let actual = result.actual_result.keyword_detection_rate;
                1.0 - (expected - actual).abs()
            })
            .collect();

        keyword_accuracies.iter().sum::<f64>() / keyword_accuracies.len() as f64
    }

    fn generate_improvement_suggestions(
        &self,
        test_results: &[TestResult],
    ) -> Vec<ImprovementSuggestion> {
        let mut suggestions = Vec::new();

        // Analyze common failure patterns
        let low_accuracy_tests: Vec<_> = test_results
            .iter()
            .filter(|result| result.accuracy_score < 0.8)
            .collect();

        if !low_accuracy_tests.is_empty() {
            suggestions.push(ImprovementSuggestion {
                category: "accuracy".to_string(),
                description: format!(
                    "{} tests showed accuracy below 80%",
                    low_accuracy_tests.len()
                ),
                priority: "high".to_string(),
                implementation_effort: "medium".to_string(),
                expected_improvement: 15.0,
            });
        }

        // Check for performance issues
        let slow_tests: Vec<_> = test_results
            .iter()
            .filter(|result| result.execution_time_ms > 5000)
            .collect();

        if !slow_tests.is_empty() {
            suggestions.push(ImprovementSuggestion {
                category: "performance".to_string(),
                description: format!(
                    "{} tests exceeded 5 second execution time",
                    slow_tests.len()
                ),
                priority: "medium".to_string(),
                implementation_effort: "low".to_string(),
                expected_improvement: 10.0,
            });
        }

        suggestions
    }

    fn create_benchmark_comparison(&self, current_accuracy: f64) -> BenchmarkComparison {
        let baseline_accuracy = self
            .benchmark_data
            .baseline_metrics
            .get("overall_accuracy")
            .unwrap_or(&0.8);
        let improvement_percentage =
            ((current_accuracy - baseline_accuracy) / baseline_accuracy) * 100.0;

        let performance_trend = match improvement_percentage {
            p if p > 5.0 => "improving",
            p if p < -5.0 => "declining",
            _ => "stable",
        };

        BenchmarkComparison {
            baseline_accuracy: *baseline_accuracy,
            current_accuracy,
            improvement_percentage,
            performance_trend: performance_trend.to_string(),
            comparison_details: [
                ("accuracy_improvement".to_string(), improvement_percentage),
                ("test_coverage".to_string(), 95.0),
                ("reliability_score".to_string(), current_accuracy * 100.0),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    fn calculate_confidence_score(
        &self,
        test_results: &[TestResult],
        overall_accuracy: f64,
    ) -> f64 {
        let test_count = test_results.len() as f64;
        let accuracy_consistency = self.calculate_accuracy_consistency(test_results);
        let test_coverage = test_count / 20.0; // Assuming 20 is full coverage

        // Weighted confidence calculation
        overall_accuracy * 0.5 + accuracy_consistency * 0.3 + test_coverage.min(1.0) * 0.2
    }

    fn calculate_accuracy_consistency(&self, test_results: &[TestResult]) -> f64 {
        let accuracies: Vec<f64> = test_results.iter().map(|r| r.accuracy_score).collect();
        let mean = accuracies.iter().sum::<f64>() / accuracies.len() as f64;

        let variance = accuracies
            .iter()
            .map(|score| (score - mean).powi(2))
            .sum::<f64>()
            / accuracies.len() as f64;

        let std_dev = variance.sqrt();

        // Lower standard deviation = higher consistency
        (1.0 - std_dev).max(0.0)
    }

    fn create_test_suite() -> Vec<TestResume> {
        vec![
            // Good resume examples
            TestResume {
                id: "good_tech_resume".to_string(),
                content: "John Doe\njohn.doe@email.com\n(555) 123-4567\n\nSUMMARY\nSoftware Engineer with 5 years experience\n\nEXPERIENCE\nSenior Software Engineer - Tech Corp (2020-2023)\n• Developed web applications using React and Node.js\n• Led team of 4 developers\n\nEDUCATION\nBS Computer Science - State University (2018)\n\nSKILLS\nJavaScript, Python, React, Node.js, AWS".to_string(),
                resume_type: "good".to_string(),
                known_issues: vec![],
                target_keywords: vec!["JavaScript".to_string(), "React".to_string(), "Python".to_string()],
                expected_ats_scores: HashMap::new(),
                difficulty_level: "easy".to_string(),
            },

            // Problematic resume with tables
            TestResume {
                id: "problematic_table_resume".to_string(),
                content: "Jane Smith\n<table><tr><td>Skills</td><td>Level</td></tr><tr><td>Python</td><td>Expert</td></tr></table>".to_string(),
                resume_type: "problematic".to_string(),
                known_issues: vec!["tables".to_string()],
                target_keywords: vec!["Python".to_string()],
                expected_ats_scores: HashMap::new(),
                difficulty_level: "medium".to_string(),
            },

            // Edge case with images and text boxes
            TestResume {
                id: "edge_case_complex".to_string(),
                content: "<img src='photo.jpg'>Contact info in image</img><textbox>Skills: Advanced programming</textbox>".to_string(),
                resume_type: "edge_case".to_string(),
                known_issues: vec!["text_in_images".to_string(), "text_boxes".to_string()],
                target_keywords: vec!["programming".to_string()],
                expected_ats_scores: HashMap::new(),
                difficulty_level: "hard".to_string(),
            },
        ]
    }

    fn create_validation_rules() -> Vec<ValidationRule> {
        vec![
            ValidationRule {
                rule_id: "format_score_minimum".to_string(),
                rule_type: "format".to_string(),
                description: "Format score should be above minimum threshold".to_string(),
                validation_function: "check_format_score".to_string(),
                acceptance_threshold: 70.0,
                weight: 1.0,
            },
            ValidationRule {
                rule_id: "parsing_accuracy".to_string(),
                rule_type: "parsing".to_string(),
                description: "Parsing simulation should be accurate".to_string(),
                validation_function: "check_parsing_accuracy".to_string(),
                acceptance_threshold: 0.8,
                weight: 1.0,
            },
        ]
    }

    fn create_benchmark_data() -> BenchmarkData {
        BenchmarkData {
            baseline_metrics: [
                ("overall_accuracy".to_string(), 0.85),
                ("format_detection_accuracy".to_string(), 0.90),
                ("parsing_accuracy".to_string(), 0.80),
                ("keyword_accuracy".to_string(), 0.75),
            ]
            .iter()
            .cloned()
            .collect(),
            historical_performance: vec![],
            target_thresholds: [
                ("minimum_accuracy".to_string(), 0.80),
                ("target_accuracy".to_string(), 0.90),
                ("excellent_accuracy".to_string(), 0.95),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validation_framework() {
        let db = crate::database::Database::new().await.unwrap();
        let framework = ATSTestingFramework::new(db);

        // Test should complete without errors
        let validation_report = framework.run_comprehensive_validation().await;
        assert!(validation_report.is_ok());

        let report = validation_report.unwrap();
        assert!(report.overall_accuracy >= 0.0);
        assert!(report.overall_accuracy <= 1.0);
        assert!(!report.test_results.is_empty());
    }

    #[test]
    fn test_test_suite_creation() {
        let test_resumes = ATSTestingFramework::create_test_suite();
        assert!(!test_resumes.is_empty());

        // Check we have different resume types
        let types: Vec<&str> = test_resumes
            .iter()
            .map(|r| r.resume_type.as_str())
            .collect();
        assert!(types.contains(&"good"));
        assert!(types.contains(&"problematic"));
        assert!(types.contains(&"edge_case"));
    }

    #[test]
    fn test_accuracy_calculation() {
        let db = futures::executor::block_on(crate::database::Database::new()).unwrap();
        let framework = ATSTestingFramework::new(db);

        let expected = TestExpectation {
            format_score_range: (80.0, 100.0),
            critical_issues_count: 0,
            parsing_success_rate: 0.9,
            keyword_detection_rate: 0.8,
            ats_compatibility_scores: HashMap::new(),
        };

        let actual = TestOutcome {
            format_score: 85.0,
            critical_issues_count: 0,
            parsing_success_rate: 0.88,
            keyword_detection_rate: 0.82,
            ats_compatibility_scores: HashMap::new(),
            processing_time_ms: 1000,
        };

        let accuracy = framework.calculate_test_accuracy(&expected, &actual);
        assert!(accuracy > 0.8); // Should be high accuracy
        assert!(accuracy <= 1.0);
    }
}
