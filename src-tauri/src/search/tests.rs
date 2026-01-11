//! Tests for global search functionality.

use super::*;

#[test]
fn test_calculate_relevance_exact_match() {
    let score = calculate_relevance("Test Page", "https://example.com", "test page");
    assert!(score >= 100.0);
}

#[test]
fn test_calculate_relevance_starts_with() {
    let score = calculate_relevance("Test Page", "https://example.com", "test");
    assert!(score >= 50.0);
    assert!(score < 100.0);
}

#[test]
fn test_calculate_relevance_contains() {
    let score = calculate_relevance("My Test Page", "https://example.com", "test");
    assert!(score >= 25.0);
    assert!(score < 50.0);
}

#[test]
fn test_calculate_relevance_url_match() {
    let score = calculate_relevance("Test", "https://www.notion.so/test", "notion");
    assert!(score >= 10.0);
}

#[test]
fn test_calculate_relevance_word_boundary() {
    let score = calculate_relevance("Test Page Content", "https://example.com", "test page");
    assert!(score > 0.0);
}

#[test]
fn test_calculate_relevance_no_match() {
    let score = calculate_relevance("Test Page", "https://example.com", "nonexistent");
    assert_eq!(score, 0.0);
}

#[test]
fn test_calculate_relevance_case_insensitive() {
    let score1 = calculate_relevance("Test Page", "https://example.com", "test");
    let score2 = calculate_relevance("Test Page", "https://example.com", "TEST");
    assert_eq!(score1, score2);
}

#[test]
fn test_calculate_relevance_multiple_words() {
    let score = calculate_relevance("My Test Page", "https://example.com", "test page");
    assert!(score > 0.0);
}

#[test]
fn test_calculate_relevance_empty_query() {
    let score = calculate_relevance("Test Page", "https://example.com", "");
    assert_eq!(score, 0.0);
}
