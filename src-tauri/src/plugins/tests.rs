//! Tests for plugin system.

use super::{PluginMetadata, PluginManifest};

#[test]
fn test_plugin_metadata_structure() {
    let metadata = PluginMetadata {
        id: "plugin-1".to_string(),
        name: "Test Plugin".to_string(),
        version: "1.0.0".to_string(),
        description: Some("Test description".to_string()),
        author: Some("Test Author".to_string()),
        enabled: true,
    };
    
    assert_eq!(metadata.name, "Test Plugin");
    assert!(metadata.enabled);
}

#[test]
fn test_plugin_manifest_structure() {
    let manifest = PluginManifest {
        id: "plugin-1".to_string(),
        name: "Test Plugin".to_string(),
        version: "1.0.0".to_string(),
        description: Some("Test".to_string()),
        author: Some("Author".to_string()),
        entry_point: "index.js".to_string(),
        permissions: vec!["read".to_string(), "write".to_string()],
    };
    
    assert_eq!(manifest.entry_point, "index.js");
    assert_eq!(manifest.permissions.len(), 2);
}
