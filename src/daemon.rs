//! # Gazelle Daemon Service
//! 
//! HTTP/gRPC service layer providing access to Gazelle's structural analysis engine.
//! Inspired by the Docker Daemon architecture - exposing a standardized API
//! that can be consumed by any client (CLI, Python, web UI, etc).

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{Model, Result as GazelleResult, GazelleError};
use crate::analysis::Analysis;
use crate::core::AnalysisResults;

/// Daemon service managing analysis sessions and models
pub struct GazelleDaemon {
    /// Active analysis sessions
    sessions: Arc<RwLock<HashMap<String, AnalysisSession>>>,
    /// Configuration
    config: DaemonConfig,
}

/// Analysis session holding model and results
#[derive(Debug, Clone)]
pub struct AnalysisSession {
    pub id: String,
    pub model: Model,
    pub static_results: Option<AnalysisResults>,
    pub modal_results: Option<AnalysisResults>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
}

/// Daemon configuration
#[derive(Debug, Clone)]
pub struct DaemonConfig {
    pub host: String,
    pub port: u16,
    pub max_sessions: usize,
    pub session_timeout_minutes: u64,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 3000, // Your envisioned port from IDEAS.md
            max_sessions: 100,
            session_timeout_minutes: 60,
        }
    }
}

/// REST API request/response types
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub model: Model,
    pub session_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionResponse {
    pub session_id: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisRequest {
    pub analysis_type: AnalysisType,
    pub options: AnalysisOptions,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AnalysisType {
    Static,
    Modal { num_modes: usize },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisOptions {
    pub solver_tolerance: Option<f64>,
    pub max_iterations: Option<usize>,
    pub output_format: OutputFormat,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OutputFormat {
    Json,
    Yaml,
    Vtk,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResponse {
    pub session_id: String,
    pub analysis_type: AnalysisType,
    pub success: bool,
    pub results: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionListResponse {
    pub sessions: Vec<SessionInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub has_static_results: bool,
    pub has_modal_results: bool,
    pub node_count: usize,
    pub element_count: usize,
}

impl GazelleDaemon {
    pub fn new(config: DaemonConfig) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Create a new analysis session
    pub async fn create_session(&self, request: CreateSessionRequest) -> GazelleResult<CreateSessionResponse> {
        let session_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        
        let session = AnalysisSession {
            id: session_id.clone(),
            model: request.model,
            static_results: None,
            modal_results: None,
            created_at: now,
            last_accessed: now,
        };

        let mut sessions = self.sessions.write().await;
        
        // Clean up old sessions if at capacity
        if sessions.len() >= self.config.max_sessions {
            self.cleanup_old_sessions(&mut sessions).await;
        }
        
        sessions.insert(session_id.clone(), session);
        
        Ok(CreateSessionResponse {
            session_id,
            message: "Session created successfully".to_string(),
        })
    }

    /// Run analysis on a session
    pub async fn run_analysis(&self, session_id: &str, request: AnalysisRequest) -> GazelleResult<AnalysisResponse> {
        let mut sessions = self.sessions.write().await;
        
        let session = sessions.get_mut(session_id)
            .ok_or_else(|| GazelleError::NotFound(format!("Session {} not found", session_id)))?;
        
        session.last_accessed = chrono::Utc::now();
        
        // Run the requested analysis
        let analysis = Analysis::new(session.model.clone());
        
        match request.analysis_type {
            AnalysisType::Static => {
                match analysis.static_analysis() {
                    Ok(results) => {
                        session.static_results = Some(results.clone());
                        
                        let results_json = match request.options.output_format {
                            OutputFormat::Json => serde_json::to_value(&results)?,
                            OutputFormat::Yaml => {
                                let yaml_str = serde_yaml::to_string(&results)
                                    .map_err(|e| GazelleError::SerializationError(e.to_string()))?;
                                serde_json::Value::String(yaml_str)
                            },
                            OutputFormat::Vtk => {
                                // TODO: Implement VTK export
                                serde_json::Value::String("VTK export not yet implemented".to_string())
                            }
                        };
                        
                        Ok(AnalysisResponse {
                            session_id: session_id.to_string(),
                            analysis_type: request.analysis_type,
                            success: true,
                            results: Some(results_json),
                            error: None,
                        })
                    },
                    Err(e) => {
                        Ok(AnalysisResponse {
                            session_id: session_id.to_string(),
                            analysis_type: request.analysis_type,
                            success: false,
                            results: None,
                            error: Some(e.to_string()),
                        })
                    }
                }
            },
            AnalysisType::Modal { num_modes } => {
                match analysis.modal_analysis(num_modes) {
                    Ok(results) => {
                        session.modal_results = Some(results.clone());
                        
                        let results_json = serde_json::to_value(&results)?;
                        
                        Ok(AnalysisResponse {
                            session_id: session_id.to_string(),
                            analysis_type: request.analysis_type,
                            success: true,
                            results: Some(results_json),
                            error: None,
                        })
                    },
                    Err(e) => {
                        Ok(AnalysisResponse {
                            session_id: session_id.to_string(),
                            analysis_type: request.analysis_type,
                            success: false,
                            results: None,
                            error: Some(e.to_string()),
                        })
                    }
                }
            }
        }
    }

    /// List all active sessions
    pub async fn list_sessions(&self) -> SessionListResponse {
        let sessions = self.sessions.read().await;
        
        let session_infos: Vec<SessionInfo> = sessions.values().map(|session| {
            SessionInfo {
                id: session.id.clone(),
                created_at: session.created_at,
                last_accessed: session.last_accessed,
                has_static_results: session.static_results.is_some(),
                has_modal_results: session.modal_results.is_some(),
                node_count: session.model.nodes.len(),
                element_count: session.model.elements.len(),
            }
        }).collect();
        
        SessionListResponse {
            sessions: session_infos,
        }
    }

    /// Get session details (returns owned copy for simplicity)
    pub async fn get_session(&self, session_id: &str) -> GazelleResult<AnalysisSession> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id)
            .cloned()
            .ok_or_else(|| GazelleError::NotFound(format!("Session {} not found", session_id)))
    }

    /// Delete a session
    pub async fn delete_session(&self, session_id: &str) -> GazelleResult<String> {
        let mut sessions = self.sessions.write().await;
        
        match sessions.remove(session_id) {
            Some(_) => Ok("Session deleted successfully".to_string()),
            None => Err(GazelleError::NotFound(format!("Session {} not found", session_id))),
        }
    }

    /// Cleanup old sessions based on timeout
    async fn cleanup_old_sessions(&self, sessions: &mut HashMap<String, AnalysisSession>) {
        let timeout_duration = chrono::Duration::minutes(self.config.session_timeout_minutes as i64);
        let cutoff_time = chrono::Utc::now() - timeout_duration;
        
        sessions.retain(|_, session| session.last_accessed > cutoff_time);
    }

    /// Get daemon status
    pub async fn status(&self) -> DaemonStatus {
        let sessions = self.sessions.read().await;
        
        DaemonStatus {
            active_sessions: sessions.len(),
            max_sessions: self.config.max_sessions,
            uptime_seconds: 0, // TODO: Track actual uptime
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DaemonStatus {
    pub active_sessions: usize,
    pub max_sessions: usize,
    pub uptime_seconds: u64,
    pub version: String,
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::*;

    #[tokio::test]
    async fn test_daemon_session_management() {
        let daemon = GazelleDaemon::new(DaemonConfig::default());
        
        // Create a simple model
        let mut model = Model::new();
        model.add_node(Node::new(0, 0.0, 0.0, 0.0)).unwrap();
        model.add_node(Node::new(1, 1.0, 0.0, 0.0)).unwrap();
        
        let request = CreateSessionRequest {
            model,
            session_name: Some("Test Session".to_string()),
        };
        
        // Test session creation
        let response = daemon.create_session(request).await.unwrap();
        assert!(!response.session_id.is_empty());
        
        // Test session listing
        let list_response = daemon.list_sessions().await;
        assert_eq!(list_response.sessions.len(), 1);
        assert_eq!(list_response.sessions[0].id, response.session_id);
        
        // Test session deletion
        let delete_response = daemon.delete_session(&response.session_id).await.unwrap();
        assert_eq!(delete_response, "Session deleted successfully");
        
        // Verify session is deleted
        let list_response = daemon.list_sessions().await;
        assert_eq!(list_response.sessions.len(), 0);
    }
}