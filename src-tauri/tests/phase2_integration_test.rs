// Integration test for Phase 2 Enhanced Analysis features
use ats_scanner::{
    semantic_analyzer::{SemanticAnalyzer},
    enhanced_scoring::{EnhancedScoringEngine},
    industry_analyzer::{IndustryAnalyzer},
    enhanced_prompts::{EnhancedPromptEngine, EnhancedPromptRequest},
    ats_simulator::{ATSSimulator},
    database::Database,
};

#[tokio::test]
async fn test_phase2_semantic_analysis_integration() {
    // Setup test database
    let db = Database::new().await.expect("Failed to create test database");
    
    // Initialize semantic analyzer
    let analyzer = SemanticAnalyzer::new(db);
    
    // Test data
    let resume_content = "Software engineer with 5 years experience in Python, JavaScript, React, and AWS";
    let job_description = "Looking for a senior software engineer with Python and React experience";
    let industry = "technology";
    
    // Perform semantic analysis
    let result = analyzer.analyze_semantic_keywords(resume_content, job_description, industry).await;
    
    assert!(result.is_ok(), "Semantic analysis should succeed");
    let analysis_result = result.unwrap();
    
    // Verify results structure
    assert!(!analysis_result.keyword_matches.is_empty(), "Should find keyword matches");
    assert!(analysis_result.semantic_similarity_score >= 0.0, "Semantic similarity should be valid");
    assert!(analysis_result.industry_relevance_score >= 0.0, "Industry relevance should be valid");
    assert!(analysis_result.confidence_score >= 0.0, "Confidence score should be valid");
    
    println!("✓ Semantic analysis integration test passed");
}

#[tokio::test]
async fn test_phase2_industry_analysis_integration() {
    // Setup test database
    let db = Database::new().await.expect("Failed to create test database");
    
    // Initialize industry analyzer
    let analyzer = IndustryAnalyzer::new(db);
    
    // Test data
    let resume_content = "Senior software engineer with 7 years experience leading teams and architecting solutions";
    let job_description = "Looking for a senior engineer with leadership experience";
    let industry = "technology";
    
    // Perform industry analysis
    let result = analyzer.analyze_for_industry(resume_content, job_description, industry).await;
    
    assert!(result.is_ok(), "Industry analysis should succeed");
    let analysis_result = result.unwrap();
    
    // Verify results structure
    assert!(!analysis_result.detected_industry.is_empty(), "Should detect an industry");
    assert!(analysis_result.confidence_score >= 0.0, "Confidence score should be valid");
    assert!(!analysis_result.role_level_assessment.detected_level.is_empty(), "Should detect role level");
    
    println!("✓ Industry analysis integration test passed");
}

#[tokio::test]
async fn test_phase2_enhanced_scoring_integration() {
    // Setup test database
    let db = Database::new().await.expect("Failed to create test database");
    
    // Initialize enhanced scoring engine
    let scoring_engine = EnhancedScoringEngine::new(db);
    
    // Test data
    let resume_content = "Software engineer with Python, React, and cloud experience";
    let job_description = "Python developer role with React and AWS skills";
    let target_industry = "technology";
    let target_role_level = "mid";
    
    // Perform comprehensive analysis
    let result = scoring_engine.comprehensive_analysis(
        resume_content,
        job_description,
        target_industry,
        target_role_level
    ).await;
    
    assert!(result.is_ok(), "Comprehensive analysis should succeed");
    let analysis_result = result.unwrap();
    
    // Verify results structure
    assert!(analysis_result.base_analysis.overall_score >= 0.0, "Overall score should be valid");
    assert!(!analysis_result.semantic_analysis.keyword_matches.is_empty(), "Should have semantic analysis");
    assert!(!analysis_result.industry_analysis.detected_industry.is_empty(), "Should have industry analysis");
    assert!(analysis_result.ats_compatibility.overall_compatibility_score >= 0.0, "Should have ATS compatibility");
    assert!(!analysis_result.scoring_breakdown.weighted_scores.is_empty(), "Should have scoring breakdown");
    
    println!("✓ Enhanced scoring integration test passed");
}

#[tokio::test]
async fn test_phase2_prompt_engineering_integration() {
    // Initialize prompt engine
    let prompt_engine = EnhancedPromptEngine::new();
    
    // Test data
    let prompt_request = EnhancedPromptRequest {
        prompt_type: "comprehensive_analysis".to_string(),
        model_name: "llama2".to_string(),
        resume_content: "Software engineer with Python experience".to_string(),
        job_description: "Python developer position".to_string(),
        industry_context: None,
        semantic_context: None,
        analysis_focus: vec!["skills".to_string(), "experience".to_string()],
        output_format: "json".to_string(),
    };
    
    // Create enhanced prompt
    let result = prompt_engine.create_enhanced_prompt(prompt_request);
    
    assert!(result.is_ok(), "Prompt creation should succeed");
    let prompt_response = result.unwrap();
    
    // Verify results structure
    assert!(!prompt_response.formatted_prompt.is_empty(), "Should have formatted prompt");
    assert!(!prompt_response.model_config.model_name.is_empty(), "Should have model config");
    assert!(prompt_response.estimated_tokens > 0, "Should estimate tokens");
    assert!(!prompt_response.prompt_strategy.is_empty(), "Should have strategy");
    
    println!("✓ Prompt engineering integration test passed");
}

#[tokio::test]
async fn test_phase2_ats_simulation_integration() {
    // Setup test database
    let db = Database::new().await.expect("Failed to create test database");
    
    // Initialize ATS simulator
    let simulator = ATSSimulator::new(db);
    
    // Test data
    let resume_content = "John Smith\njohn.smith@email.com\n(555) 123-4567\n\nExperience:\nSoftware Engineer at TechCorp (2020-2023)\n- Developed Python applications\n- Led team of 3 developers\n- Implemented React frontend\n\nEducation:\nB.S. Computer Science, Tech University (2020)\n\nSkills:\nPython, JavaScript, React, AWS, Docker";
    let target_keywords = vec!["Python".to_string(), "React".to_string(), "AWS".to_string(), "team leadership".to_string()];
    
    // Perform ATS simulation
    let result = simulator.simulate_ats_processing(resume_content, &target_keywords).await;
    
    assert!(result.is_ok(), "ATS simulation should succeed");
    let simulation_result = result.unwrap();
    
    // Verify results structure
    assert!(simulation_result.overall_ats_score >= 0.0, "Overall ATS score should be valid");
    assert!(!simulation_result.system_simulations.is_empty(), "Should have system simulations");
    assert!(simulation_result.parsing_analysis.contact_info_extraction.email_detected, "Should detect email");
    assert!(simulation_result.parsing_analysis.contact_info_extraction.phone_detected, "Should detect phone");
    assert!(simulation_result.keyword_extraction.keywords_found.len() >= 2, "Should find some keywords");
    assert!(!simulation_result.optimization_recommendations.is_empty(), "Should provide recommendations");
    
    println!("✓ ATS simulation integration test passed");
    println!("   🎯 Overall ATS Score: {:.1}%", simulation_result.overall_ats_score * 100.0);
    println!("   📞 Contact extraction confidence: {:.1}%", 
             simulation_result.parsing_analysis.contact_info_extraction.extraction_confidence * 100.0);
    println!("   🔍 Keywords found: {}/{}", 
             simulation_result.keyword_extraction.keywords_found.len(),
             target_keywords.len());
    println!("   💡 Optimization recommendations: {}", 
             simulation_result.optimization_recommendations.len());
}

#[tokio::test]
async fn test_phase2_complete_workflow_integration() {
    println!("🚀 Testing complete Phase 2 workflow integration...");
    
    // Setup test database
    let db = Database::new().await.expect("Failed to create test database");
    
    // Test data
    let resume_content = "Senior Software Engineer with 6 years of experience in Python, React, AWS, and Docker. Led teams of 3-5 developers and architected scalable solutions.";
    let job_description = "We are looking for a Senior Software Engineer with Python, React, cloud experience and team leadership skills.";
    let target_industry = "technology";
    let target_role_level = "senior";
    
    // Step 1: Semantic Analysis
    println!("  📊 Running semantic analysis...");
    let semantic_analyzer = SemanticAnalyzer::new(db.clone());
    let semantic_result = semantic_analyzer.analyze_semantic_keywords(
        resume_content, 
        job_description, 
        target_industry
    ).await.expect("Semantic analysis should succeed");
    
    // Step 2: Industry Analysis
    println!("  🏭 Running industry analysis...");
    let industry_analyzer = IndustryAnalyzer::new(db.clone());
    let industry_result = industry_analyzer.analyze_for_industry(
        resume_content,
        job_description,
        target_industry
    ).await.expect("Industry analysis should succeed");
    
    // Step 3: Enhanced Scoring
    println!("  🎯 Running enhanced scoring...");
    let scoring_engine = EnhancedScoringEngine::new(db.clone());
    let comprehensive_result = scoring_engine.comprehensive_analysis(
        resume_content,
        job_description,
        target_industry,
        target_role_level
    ).await.expect("Comprehensive analysis should succeed");
    
    // Step 4: ATS Simulation
    println!("  🏢 Running ATS simulation...");
    let ats_simulator = ATSSimulator::new(db.clone());
    let target_keywords = vec!["Python".to_string(), "React".to_string(), "AWS".to_string(), "leadership".to_string()];
    let ats_result = ats_simulator.simulate_ats_processing(
        resume_content,
        &target_keywords
    ).await.expect("ATS simulation should succeed");

    // Step 5: Enhanced Prompt Engineering
    println!("  🤖 Testing prompt engineering...");
    let prompt_engine = EnhancedPromptEngine::new();
    let prompt_request = EnhancedPromptRequest {
        prompt_type: "comprehensive_analysis".to_string(),
        model_name: "llama2".to_string(),
        resume_content: resume_content.to_string(),
        job_description: job_description.to_string(),
        industry_context: Some(industry_result.clone()),
        semantic_context: Some(semantic_result.clone()),
        analysis_focus: vec!["skills".to_string(), "experience".to_string(), "leadership".to_string()],
        output_format: "json".to_string(),
    };
    let prompt_result = prompt_engine.create_enhanced_prompt(prompt_request)
        .expect("Prompt creation should succeed");
    
    // Verify integrated results
    println!("  ✅ Verifying integrated results...");
    
    // Semantic analysis verification
    assert!(semantic_result.confidence_score > 0.5, "Should have good semantic confidence");
    assert!(!semantic_result.keyword_matches.is_empty(), "Should match keywords");
    
    // Industry analysis verification  
    assert_eq!(industry_result.detected_industry, "technology", "Should detect technology industry");
    assert_eq!(industry_result.role_level_assessment.detected_level, "senior", "Should detect senior level");
    
    // Enhanced scoring verification
    assert!(comprehensive_result.base_analysis.overall_score > 60.0, "Should have good overall score");
    assert!(!comprehensive_result.scoring_breakdown.weighted_scores.is_empty(), "Should have weighted scores");
    
    // ATS simulation verification
    assert!(ats_result.overall_ats_score > 0.5, "Should have good ATS compatibility");
    assert!(!ats_result.system_simulations.is_empty(), "Should simulate multiple ATS systems");
    assert!(ats_result.parsing_analysis.contact_info_extraction.email_detected, "Should detect contact info");
    
    // Prompt engineering verification
    assert!(prompt_result.formatted_prompt.contains("Python"), "Prompt should contain relevant skills");
    assert!(prompt_result.estimated_tokens > 100, "Should estimate reasonable token count");
    
    println!("✅ Complete Phase 2 workflow integration test passed!");
    println!("   📈 Overall Score: {:.1}", comprehensive_result.base_analysis.overall_score);
    println!("   🎯 Semantic Confidence: {:.1}%", semantic_result.confidence_score * 100.0);
    println!("   🏭 Industry: {} (confidence: {:.1}%)", 
             industry_result.detected_industry, 
             industry_result.confidence_score * 100.0);
    println!("   👔 Role Level: {} (confidence: {:.1}%)", 
             industry_result.role_level_assessment.detected_level,
             industry_result.role_level_assessment.confidence * 100.0);
    println!("   🤖 Prompt Tokens: {}", prompt_result.estimated_tokens);
    println!("   🏢 ATS Compatibility: {:.1}%", ats_result.overall_ats_score * 100.0);
    println!("   🔍 Keywords Found: {}/{}", 
             ats_result.keyword_extraction.keywords_found.len(),
             target_keywords.len());
}