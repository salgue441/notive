//! Tests for multi-monitor support.

use super::{MonitorInfo, WindowPlacement};

#[test]
fn test_monitor_info_structure() {
    let monitor = MonitorInfo {
        id: "monitor-1".to_string(),
        name: "Primary Monitor".to_string(),
        position: (0, 0),
        size: (1920, 1080),
        scale_factor: 1.0,
        is_primary: true,
    };
    
    assert_eq!(monitor.name, "Primary Monitor");
    assert_eq!(monitor.size, (1920, 1080));
    assert!(monitor.is_primary);
}

#[test]
fn test_window_placement_structure() {
    let placement = WindowPlacement {
        window_label: "main".to_string(),
        monitor_id: Some("monitor-1".to_string()),
        position: Some((100, 100)),
        size: Some((800, 600)),
        maximized: false,
    };
    
    assert_eq!(placement.window_label, "main");
    assert_eq!(placement.position, Some((100, 100)));
}
