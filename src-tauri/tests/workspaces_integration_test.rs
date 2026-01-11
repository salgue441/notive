//! Integration tests for workspace functionality.

use notive::workspaces::Workspace;

#[test]
fn test_workspace_serialization() {
    let workspace = Workspace::default();
    let json = serde_json::to_string(&workspace).unwrap();
    
    assert!(!json.is_empty());
    assert!(json.contains("Default Workspace"));
    assert!(json.contains("https://www.notion.so"));
}

#[test]
fn test_workspace_deserialization() {
    let json = r#"{
        "id": "test-id",
        "name": "Test Workspace",
        "url": "https://www.notion.so/test",
        "zoom_level": 1.5
    }"#;
    
    let workspace: Workspace = serde_json::from_str(json).unwrap();
    assert_eq!(workspace.name, "Test Workspace");
    assert_eq!(workspace.url, "https://www.notion.so/test");
    assert_eq!(workspace.zoom_level, 1.5);
}

#[test]
fn test_workspace_round_trip() {
    let original = Workspace {
        id: "test-id".to_string(),
        name: "Test".to_string(),
        url: "https://example.com".to_string(),
        zoom_level: 1.25,
    };
    
    let json = serde_json::to_string(&original).unwrap();
    let restored: Workspace = serde_json::from_str(&json).unwrap();
    
    assert_eq!(original.name, restored.name);
    assert_eq!(original.url, restored.url);
    assert_eq!(original.zoom_level, restored.zoom_level);
}
