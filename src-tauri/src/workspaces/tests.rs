//! Tests for workspace management.

use super::{Workspace, WorkspaceManager};

#[test]
fn test_workspace_default() {
    let workspace = Workspace::default();
    assert_eq!(workspace.name, "Default Workspace");
    assert_eq!(workspace.url, "https://www.notion.so");
    assert_eq!(workspace.zoom_level, 1.0);
    assert!(!workspace.id.is_empty());
}

#[test]
fn test_workspace_manager_new() {
    let manager = WorkspaceManager::new();
    assert_eq!(manager.list_workspaces().len(), 0);
    assert!(manager.get_active().is_none());
}

#[test]
fn test_workspace_manager_add() {
    let mut manager = WorkspaceManager::new();
    let workspace = Workspace {
        id: "test-1".to_string(),
        name: "Test Workspace".to_string(),
        url: "https://www.notion.so/test".to_string(),
        zoom_level: 1.0,
    };
    
    manager.add_workspace(workspace);
    
    assert_eq!(manager.list_workspaces().len(), 1);
    assert!(manager.get_workspace("test-1").is_some());
}

#[test]
fn test_workspace_manager_remove() {
    let mut manager = WorkspaceManager::new();
    let workspace = Workspace {
        id: "test-1".to_string(),
        name: "Test".to_string(),
        url: "https://example.com".to_string(),
        zoom_level: 1.0,
    };
    
    manager.add_workspace(workspace);
    assert_eq!(manager.list_workspaces().len(), 1);
    
    let removed = manager.remove_workspace("test-1");
    assert!(removed.is_some());
    assert_eq!(manager.list_workspaces().len(), 0);
}

#[test]
fn test_workspace_manager_set_active() {
    let mut manager = WorkspaceManager::new();
    let workspace = Workspace {
        id: "test-1".to_string(),
        name: "Test".to_string(),
        url: "https://example.com".to_string(),
        zoom_level: 1.0,
    };
    
    manager.add_workspace(workspace);
    manager.set_active(Some("test-1".to_string()));
    
    assert!(manager.get_active().is_some());
    assert_eq!(manager.get_active().unwrap().id, "test-1");
}

#[test]
fn test_workspace_manager_multiple_workspaces() {
    let mut manager = WorkspaceManager::new();
    
    for i in 0..5 {
        let workspace = Workspace {
            id: format!("test-{}", i),
            name: format!("Workspace {}", i),
            url: format!("https://example.com/{}", i),
            zoom_level: 1.0,
        };
        manager.add_workspace(workspace);
    }
    
    assert_eq!(manager.list_workspaces().len(), 5);
}

#[test]
fn test_workspace_manager_get_nonexistent() {
    let manager = WorkspaceManager::new();
    assert!(manager.get_workspace("nonexistent").is_none());
}

#[test]
fn test_workspace_manager_remove_nonexistent() {
    let mut manager = WorkspaceManager::new();
    let removed = manager.remove_workspace("nonexistent");
    assert!(removed.is_none());
}
