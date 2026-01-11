//! Session management for multiple user sessions.

use persistence::{load_sessions, save_sessions};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// User session information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub account_id: String,
    pub account_email: String,
    pub created_at: u64,
    pub last_used: u64,
    pub expires_at: Option<u64>,
    pub is_active: bool,
}

impl Session {
    pub fn new(account_id: String, account_email: String, expires_in_seconds: Option<u64>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let expires_at = expires_in_seconds.map(|expires_in| now + expires_in);
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            account_id,
            account_email,
            created_at: now,
            last_used: now,
            expires_at,
            is_active: true,
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now >= expires_at
        } else {
            false
        }
    }

    pub fn update_last_used(&mut self) {
        self.last_used = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

/// Manages user sessions.
pub struct SessionManager {
    sessions: Vec<Session>,
    active_session_id: Option<String>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: vec![],
            active_session_id: None,
        }
    }

    pub fn add_session(&mut self, session: Session) {
        // Deactivate other sessions for the same account
        for s in &mut self.sessions {
            if s.account_id == session.account_id {
                s.is_active = false;
            }
        }
        self.sessions.push(session.clone());
        self.active_session_id = Some(session.id.clone());
    }

    pub fn get_active_session(&self) -> Option<&Session> {
        self.active_session_id
            .as_ref()
            .and_then(|id| self.sessions.iter().find(|s| s.id == *id))
    }

    pub fn get_session(&self, session_id: &str) -> Option<&Session> {
        self.sessions.iter().find(|s| s.id == session_id)
    }

    pub fn list_sessions(&self) -> Vec<&Session> {
        self.sessions.iter().collect()
    }

    pub fn remove_session(&mut self, session_id: &str) -> Option<Session> {
        if let Some(pos) = self.sessions.iter().position(|s| s.id == session_id) {
            let session = self.sessions.remove(pos);
            if self.active_session_id.as_ref() == Some(&session_id) {
                self.active_session_id = None;
            }
            Some(session)
        } else {
            None
        }
    }

    pub fn cleanup_expired(&mut self) {
        self.sessions.retain(|s| !s.is_expired());
        if let Some(active_id) = &self.active_session_id {
            if !self.sessions.iter().any(|s| s.id == *active_id) {
                self.active_session_id = None;
            }
        }
    }

    pub fn set_active(&mut self, session_id: &str) -> Result<(), String> {
        if self.sessions.iter().any(|s| s.id == session_id) {
            // Deactivate all sessions
            for s in &mut self.sessions {
                s.is_active = false;
            }
            // Activate the selected session
            if let Some(session) = self.sessions.iter_mut().find(|s| s.id == session_id) {
                session.is_active = true;
                session.update_last_used();
            }
            self.active_session_id = Some(session_id.to_string());
            Ok(())
        } else {
            Err(format!("Session {} not found", session_id))
        }
    }
}

/// Creates a new session.
#[tauri::command]
pub async fn create_session<R: Runtime>(
    app: AppHandle<R>,
    account_id: String,
    account_email: String,
    expires_in_seconds: Option<u64>,
) -> Result<String, String> {
    log::debug!("Creating session for account: {}", account_email);
    
    let session = Session::new(account_id, account_email, expires_in_seconds);
    let session_id = session.id.clone();
    
    // Load existing sessions
    let mut sessions = load_sessions(&app).unwrap_or_default();
    
    // Add new session
    sessions.push(session);
    
    // Save sessions
    save_sessions(&app, &sessions)?;
    
    log::info!("Session created: {}", session_id);
    Ok(session_id)
}

/// Lists all sessions.
#[tauri::command]
pub fn list_sessions<R: Runtime>(app: AppHandle<R>) -> Result<Vec<Session>, String> {
    let sessions = load_sessions(&app).unwrap_or_default();
    Ok(sessions)
}

/// Gets the active session.
#[tauri::command]
pub fn get_active_session<R: Runtime>(app: AppHandle<R>) -> Result<Option<Session>, String> {
    let sessions = load_sessions(&app).unwrap_or_default();
    Ok(sessions.into_iter().find(|s| s.is_active))
}

/// Sets the active session.
#[tauri::command]
pub fn set_active_session<R: Runtime>(
    app: AppHandle<R>,
    session_id: String,
) -> Result<(), String> {
    let mut sessions = load_sessions(&app).unwrap_or_default();
    
    // Deactivate all
    for s in &mut sessions {
        s.is_active = false;
    }
    
    // Activate selected
    if let Some(session) = sessions.iter_mut().find(|s| s.id == session_id) {
        session.is_active = true;
        session.update_last_used();
        save_sessions(&app, &sessions)?;
        Ok(())
    } else {
        Err(format!("Session {} not found", session_id))
    }
}

/// Removes a session.
#[tauri::command]
pub fn remove_session<R: Runtime>(
    app: AppHandle<R>,
    session_id: String,
) -> Result<(), String> {
    let mut sessions = load_sessions(&app).unwrap_or_default();
    sessions.retain(|s| s.id != session_id);
    save_sessions(&app, &sessions)?;
    Ok(())
}

/// Cleans up expired sessions.
#[tauri::command]
pub fn cleanup_expired_sessions<R: Runtime>(app: AppHandle<R>) -> Result<usize, String> {
    let mut sessions = load_sessions(&app).unwrap_or_default();
    let count_before = sessions.len();
    sessions.retain(|s| !s.is_expired());
    let count_after = sessions.len();
    let removed = count_before - count_after;
    
    if removed > 0 {
        save_sessions(&app, &sessions)?;
    }
    
    Ok(removed)
}
