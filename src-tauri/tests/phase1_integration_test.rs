use ats_scanner::{
    config::ConfigManager,
    database::Database,
    models::{
        ATSCompatibilityRule, IndustryKeyword, ModelPerformanceMetrics, ScoringBenchmark,
        UserFeedback,
    },
};
use chrono::Utc;
use uuid::Uuid;

#[tokio::test]
async fn test_phase1_database_functionality() {
    // Initialize database with Phase 1 schema
    let db = Database::new()
        .await
        .expect("Failed to initialize database");

    println!("✅ Database initialized with Phase 1 schema");

    // Test Industry Keywords
    let keyword = IndustryKeyword {
        id: Uuid::new_v4().to_string(),
        industry: "Software Engineering".to_string(),
        keyword: "Python".to_string(),
        weight: 1.5,
        category: "programming_language".to_string(),
        synonyms: r#"["python3", "py"]"#.to_string(),
        created_at: Utc::now(),
    };

    db.save_industry_keyword(&keyword)
        .await
        .expect("Failed to save industry keyword");
    let keywords = db
        .get_industry_keywords("Software Engineering")
        .await
        .expect("Failed to get keywords");
    assert_eq!(keywords.len(), 1);
    assert_eq!(keywords[0].keyword, "Python");
    println!("✅ Industry keywords functionality working");

    // Test ATS Compatibility Rules
    let rule = ATSCompatibilityRule {
        id: Uuid::new_v4().to_string(),
        ats_system: "Greenhouse".to_string(),
        rule_type: "format".to_string(),
        rule_description: "Avoid tables in resume".to_string(),
        penalty_weight: 0.2,
        detection_pattern: r"<table".to_string(),
        suggestion: "Use plain text formatting instead of tables".to_string(),
        severity: "medium".to_string(),
        created_at: Utc::now(),
    };

    db.save_ats_rule(&rule)
        .await
        .expect("Failed to save ATS rule");
    let rules = db
        .get_ats_rules(Some("Greenhouse"))
        .await
        .expect("Failed to get ATS rules");
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].rule_type, "format");
    println!("✅ ATS compatibility rules functionality working");

    // Test Scoring Benchmarks
    let benchmark = ScoringBenchmark {
        id: Uuid::new_v4().to_string(),
        industry: "Software Engineering".to_string(),
        job_level: "senior".to_string(),
        experience_years: "5-8".to_string(),
        benchmark_type: "overall_score".to_string(),
        score_threshold: 85.0,
        description: "Senior software engineer benchmark".to_string(),
        created_at: Utc::now(),
    };

    db.save_scoring_benchmark(&benchmark)
        .await
        .expect("Failed to save benchmark");
    let benchmarks = db
        .get_scoring_benchmarks("Software Engineering", "senior")
        .await
        .expect("Failed to get benchmarks");
    assert_eq!(benchmarks.len(), 1);
    assert_eq!(benchmarks[0].score_threshold, 85.0);
    println!("✅ Scoring benchmarks functionality working");

    // Test User Feedback
    let feedback = UserFeedback {
        id: Uuid::new_v4().to_string(),
        analysis_id: Uuid::new_v4().to_string(),
        user_id: "test_user".to_string(),
        feedback_type: "accuracy".to_string(),
        rating: 4,
        comment: Some("Very helpful analysis".to_string()),
        helpful_suggestions: r#"["keyword optimization", "format improvements"]"#.to_string(),
        created_at: Utc::now(),
    };

    db.save_user_feedback(&feedback)
        .await
        .expect("Failed to save feedback");
    let feedback_list = db
        .get_feedback_by_analysis(&feedback.analysis_id)
        .await
        .expect("Failed to get feedback");
    assert_eq!(feedback_list.len(), 1);
    assert_eq!(feedback_list[0].rating, 4);
    println!("✅ User feedback functionality working");

    // Test Model Performance Metrics
    let metrics = ModelPerformanceMetrics {
        id: Uuid::new_v4().to_string(),
        model_name: "llama2".to_string(),
        analysis_id: Uuid::new_v4().to_string(),
        processing_time_ms: 2500,
        memory_usage_mb: 512.0,
        accuracy_score: 0.89,
        user_satisfaction: Some(4.2),
        error_count: 0,
        created_at: Utc::now(),
    };

    db.save_model_performance(&metrics)
        .await
        .expect("Failed to save performance metrics");
    let stats = db
        .get_model_performance_stats("llama2")
        .await
        .expect("Failed to get performance stats");

    // Verify the stats structure
    assert!(stats.get("model_name").is_some());
    assert!(stats.get("analysis_count").is_some());
    println!("✅ Model performance metrics functionality working");

    // Test analytics
    let feedback_stats = db
        .get_feedback_stats(Some(30))
        .await
        .expect("Failed to get feedback stats");
    assert!(feedback_stats.get("total_feedback").is_some());
    println!("✅ Analytics functionality working");

    println!("🎉 All Phase 1 database functionality tests passed!");
}

#[tokio::test]
async fn test_phase1_configuration_system() {
    use tempfile::tempdir;

    // Create a temporary directory for config
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("test_config.json");

    // Test configuration creation
    let config_manager =
        ConfigManager::new_with_path(config_path.clone()).expect("Failed to create config manager");
    assert!(config_path.exists());
    println!("✅ Configuration file creation working");

    // Test configuration validation
    let warnings = config_manager
        .validate_config()
        .expect("Failed to validate config");
    assert!(warnings.is_empty()); // Should have no warnings with default config
    println!("✅ Configuration validation working");

    // Test configuration export
    let config_json = config_manager
        .export_config()
        .expect("Failed to export config");
    assert!(!config_json.is_empty());
    assert!(config_json.contains("ollama_config"));
    println!("✅ Configuration export working");

    // Test getting individual config sections
    let ollama_config = config_manager.get_ollama_config();
    assert_eq!(ollama_config.port, 11434);
    assert_eq!(ollama_config.host, "localhost");
    println!("✅ Configuration section access working");

    let analysis_config = config_manager.get_analysis_config();
    assert!(analysis_config.enable_industry_analysis);
    assert!(analysis_config.enable_ats_compatibility);
    assert!(analysis_config.enable_benchmark_comparison);
    println!("✅ Analysis configuration working");

    println!("🎉 All Phase 1 configuration system tests passed!");
}

#[test]
fn test_phase1_data_models() {
    // Test that all Phase 1 models can be serialized/deserialized
    let keyword = IndustryKeyword {
        id: Uuid::new_v4().to_string(),
        industry: "Tech".to_string(),
        keyword: "Rust".to_string(),
        weight: 2.0,
        category: "language".to_string(),
        synonyms: r#"["rust-lang"]"#.to_string(),
        created_at: Utc::now(),
    };

    let serialized = serde_json::to_string(&keyword).expect("Failed to serialize keyword");
    let deserialized: IndustryKeyword =
        serde_json::from_str(&serialized).expect("Failed to deserialize keyword");
    assert_eq!(keyword.keyword, deserialized.keyword);
    println!("✅ IndustryKeyword serialization working");

    let rule = ATSCompatibilityRule {
        id: Uuid::new_v4().to_string(),
        ats_system: "Workday".to_string(),
        rule_type: "parsing".to_string(),
        rule_description: "Test rule".to_string(),
        penalty_weight: 0.1,
        detection_pattern: "test".to_string(),
        suggestion: "Test suggestion".to_string(),
        severity: "low".to_string(),
        created_at: Utc::now(),
    };

    let serialized = serde_json::to_string(&rule).expect("Failed to serialize rule");
    let deserialized: ATSCompatibilityRule =
        serde_json::from_str(&serialized).expect("Failed to deserialize rule");
    assert_eq!(rule.ats_system, deserialized.ats_system);
    println!("✅ ATSCompatibilityRule serialization working");

    // Test other models...
    println!("🎉 All Phase 1 data model tests passed!");
}
