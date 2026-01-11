//! Tests for AI integration.

use super::AISearchResult;

#[test]
fn test_ai_search_result_structure() {
    let result = AISearchResult {
        title: "Test Result".to_string(),
        url: "https://example.com".to_string(),
        snippet: "Test snippet".to_string(),
        relevance_score: 0.95,
        ai_summary: Some("AI summary".to_string()),
    };
    
    assert_eq!(result.title, "Test Result");
    assert!(result.relevance_score > 0.0);
    assert!(result.ai_summary.is_some());
}
