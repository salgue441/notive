//! Tests for performance monitoring.

use super::{ResourceUsage, PerformanceMetrics};

#[test]
fn test_resource_usage_structure() {
    let usage = ResourceUsage {
        memory_mb: 150.5,
        cpu_percent: 25.0,
        timestamp: 1000,
    };
    
    assert_eq!(usage.memory_mb, 150.5);
    assert_eq!(usage.cpu_percent, 25.0);
    assert_eq!(usage.timestamp, 1000);
}

#[test]
fn test_performance_metrics_structure() {
    let usage = ResourceUsage {
        memory_mb: 100.0,
        cpu_percent: 10.0,
        timestamp: 1000,
    };
    
    let metrics = PerformanceMetrics {
        memory_usage: vec![usage.clone()],
        cpu_usage: vec![usage.clone()],
        average_memory_mb: 100.0,
        average_cpu_percent: 10.0,
        peak_memory_mb: 100.0,
        peak_cpu_percent: 10.0,
    };
    
    assert_eq!(metrics.average_memory_mb, 100.0);
    assert_eq!(metrics.peak_memory_mb, 100.0);
}
