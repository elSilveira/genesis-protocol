//! 🚨 Genesis Protocol Error Types
//!
//! This module defines all error types used throughout the Genesis Protocol.
//! Each component has its own error type that can be converted to the main
//! GenesisError for unified error handling.

use thiserror::Error;

/// Main error type for Genesis Protocol
#[derive(Error, Debug)]
pub enum GenesisError {
    #[error("DNA error: {0}")]
    DNA(#[from] crate::dna::DNAError),
    
    #[error("TRON organism error: {0}")]
    TRON(#[from] crate::tron::TRONError),
    
    #[error("Neural communication error: {0}")]
    Neural(#[from] crate::neural::SynapseError),
    
    #[error("Evolution error: {0}")]
    Evolution(#[from] crate::evolution::EvolutionError),
    
    #[error("Collective intelligence error: {0}")]
    Collective(#[from] crate::collective::CollectiveError),
    
    #[error("Network error: {0}")]
    Network(#[from] crate::network::NetworkError),
    
    #[error("Organism not found: {0}")]
    OrganismNotFound(String),
    
    #[error("Network capacity exceeded: {current} organisms (max: {max})")]
    NetworkCapacityExceeded { current: usize, max: usize },
    
    #[error("Protocol version mismatch: expected {expected}, got {actual}")]
    ProtocolVersionMismatch { expected: String, actual: String },
    
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    
    #[error("Insufficient resources: {resource}")]
    InsufficientResources { resource: String },
    
    #[error("Operation timeout: {operation} took longer than {timeout_ms}ms")]
    OperationTimeout { operation: String, timeout_ms: u64 },
    
    #[error("Security violation: {0}")]
    SecurityViolation(String),
    
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for Genesis Protocol operations
pub type GenesisResult<T> = Result<T, GenesisError>;

/// Error recovery strategies
#[derive(Debug, Clone, Copy)]
pub enum ErrorRecovery {
    /// Retry the operation
    Retry,
    /// Fallback to alternative method
    Fallback,
    /// Gracefully degrade functionality
    Degrade,
    /// Fail fast and propagate error
    Fail,
}

/// Error context for better debugging
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation: String,
    pub organism_id: Option<String>,
    pub timestamp: u64,
    pub recovery_strategy: ErrorRecovery,
    pub metadata: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    pub fn new(operation: &str) -> Self {
        ErrorContext {
            operation: operation.to_string(),
            organism_id: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            recovery_strategy: ErrorRecovery::Fail,
            metadata: std::collections::HashMap::new(),
        }
    }
    
    pub fn with_organism(mut self, organism_id: &str) -> Self {
        self.organism_id = Some(organism_id.to_string());
        self
    }
    
    pub fn with_recovery(mut self, strategy: ErrorRecovery) -> Self {
        self.recovery_strategy = strategy;
        self
    }
    
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

/// Enhanced error with context
#[derive(Debug)]
pub struct ContextualError {
    pub error: GenesisError,
    pub context: ErrorContext,
    pub source_location: Option<String>,
}

impl ContextualError {
    pub fn new(error: GenesisError, context: ErrorContext) -> Self {
        ContextualError {
            error,
            context,
            source_location: None,
        }
    }
    
    pub fn with_location(mut self, location: &str) -> Self {
        self.source_location = Some(location.to_string());
        self
    }
}

impl std::fmt::Display for ContextualError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error in {}: {}", self.context.operation, self.error)?;
        if let Some(organism_id) = &self.context.organism_id {
            write!(f, " (organism: {})", organism_id)?;
        }
        if let Some(location) = &self.source_location {
            write!(f, " at {}", location)?;
        }
        Ok(())
    }
}

impl std::error::Error for ContextualError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

/// Macro for creating contextual errors
#[macro_export]
macro_rules! genesis_error {
    ($error:expr, $operation:expr) => {
        ContextualError::new(
            $error.into(),
            ErrorContext::new($operation)
        ).with_location(&format!("{}:{}", file!(), line!()))
    };
    
    ($error:expr, $operation:expr, $organism_id:expr) => {
        ContextualError::new(
            $error.into(),
            ErrorContext::new($operation).with_organism($organism_id)
        ).with_location(&format!("{}:{}", file!(), line!()))
    };
    
    ($error:expr, $operation:expr, $organism_id:expr, $recovery:expr) => {
        ContextualError::new(
            $error.into(),
            ErrorContext::new($operation)
                .with_organism($organism_id)
                .with_recovery($recovery)
        ).with_location(&format!("{}:{}", file!(), line!()))
    };
}

/// Error metrics for monitoring
#[derive(Debug, Clone)]
pub struct ErrorMetrics {
    pub total_errors: u64,
    pub errors_by_type: std::collections::HashMap<String, u64>,
    pub errors_by_organism: std::collections::HashMap<String, u64>,
    pub recovery_attempts: u64,
    pub successful_recoveries: u64,
    pub last_error_time: u64,
}

impl ErrorMetrics {
    pub fn new() -> Self {
        ErrorMetrics {
            total_errors: 0,
            errors_by_type: std::collections::HashMap::new(),
            errors_by_organism: std::collections::HashMap::new(),
            recovery_attempts: 0,
            successful_recoveries: 0,
            last_error_time: 0,
        }
    }
    
    pub fn record_error(&mut self, error: &GenesisError, organism_id: Option<&str>) {
        self.total_errors += 1;
        self.last_error_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let error_type = match error {
            GenesisError::DNA(_) => "DNA",
            GenesisError::TRON(_) => "TRON",
            GenesisError::Neural(_) => "Neural",
            GenesisError::Evolution(_) => "Evolution",
            GenesisError::Collective(_) => "Collective",
            GenesisError::Network(_) => "Network",
            GenesisError::OrganismNotFound(_) => "OrganismNotFound",
            GenesisError::NetworkCapacityExceeded { .. } => "NetworkCapacityExceeded",
            GenesisError::ProtocolVersionMismatch { .. } => "ProtocolVersionMismatch",
            GenesisError::InvalidConfiguration(_) => "InvalidConfiguration",
            GenesisError::InsufficientResources { .. } => "InsufficientResources",
            GenesisError::OperationTimeout { .. } => "OperationTimeout",
            GenesisError::SecurityViolation(_) => "SecurityViolation",
            GenesisError::IO(_) => "IO",
            GenesisError::Serialization(_) => "Serialization",
            GenesisError::Internal(_) => "Internal",
        };
        
        *self.errors_by_type.entry(error_type.to_string()).or_insert(0) += 1;
        
        if let Some(organism_id) = organism_id {
            *self.errors_by_organism.entry(organism_id.to_string()).or_insert(0) += 1;
        }
    }
    
    pub fn record_recovery_attempt(&mut self, successful: bool) {
        self.recovery_attempts += 1;
        if successful {
            self.successful_recoveries += 1;
        }
    }
    
    pub fn get_error_rate(&self) -> f64 {
        if self.total_errors == 0 {
            0.0
        } else {
            self.total_errors as f64 / std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as f64
        }
    }
    
    pub fn get_recovery_rate(&self) -> f64 {
        if self.recovery_attempts == 0 {
            0.0
        } else {
            self.successful_recoveries as f64 / self.recovery_attempts as f64
        }
    }
}

impl Default for ErrorMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Error handler trait for custom error handling
pub trait ErrorHandler {
    fn handle_error(&self, error: &GenesisError, context: &ErrorContext) -> ErrorRecovery;
    fn should_retry(&self, error: &GenesisError, attempt: u32) -> bool;
    fn get_retry_delay(&self, error: &GenesisError, attempt: u32) -> std::time::Duration;
}

/// Default error handler implementation
pub struct DefaultErrorHandler {
    max_retries: u32,
    base_delay_ms: u64,
}

impl DefaultErrorHandler {
    pub fn new() -> Self {
        DefaultErrorHandler {
            max_retries: 3,
            base_delay_ms: 100,
        }
    }
    
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }
    
    pub fn with_base_delay(mut self, delay_ms: u64) -> Self {
        self.base_delay_ms = delay_ms;
        self
    }
}

impl ErrorHandler for DefaultErrorHandler {
    fn handle_error(&self, error: &GenesisError, _context: &ErrorContext) -> ErrorRecovery {
        match error {
            GenesisError::Neural(_) => ErrorRecovery::Retry,
            GenesisError::Network(_) => ErrorRecovery::Retry,
            GenesisError::OperationTimeout { .. } => ErrorRecovery::Retry,
            GenesisError::InsufficientResources { .. } => ErrorRecovery::Degrade,
            GenesisError::NetworkCapacityExceeded { .. } => ErrorRecovery::Degrade,
            GenesisError::DNA(_) => ErrorRecovery::Fail,
            GenesisError::TRON(_) => ErrorRecovery::Fallback,
            GenesisError::Evolution(_) => ErrorRecovery::Fallback,
            GenesisError::SecurityViolation(_) => ErrorRecovery::Fail,
            _ => ErrorRecovery::Fail,
        }
    }
    
    fn should_retry(&self, error: &GenesisError, attempt: u32) -> bool {
        if attempt >= self.max_retries {
            return false;
        }
        
        match error {
            GenesisError::Neural(_) => true,
            GenesisError::Network(_) => true,
            GenesisError::OperationTimeout { .. } => true,
            GenesisError::IO(_) => true,
            _ => false,
        }
    }
    
    fn get_retry_delay(&self, _error: &GenesisError, attempt: u32) -> std::time::Duration {
        // Exponential backoff
        let delay_ms = self.base_delay_ms * (2_u64.pow(attempt));
        std::time::Duration::from_millis(delay_ms)
    }
}

impl Default for DefaultErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dna::DNAError;

    #[test]
    fn test_genesis_error_conversion() {
        let dna_error = DNAError::InvalidSecretKey;
        let genesis_error: GenesisError = dna_error.into();
        
        match genesis_error {
            GenesisError::DNA(DNAError::InvalidSecretKey) => {},
            _ => panic!("Error conversion failed"),
        }
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new("test_operation")
            .with_organism("tron_123")
            .with_recovery(ErrorRecovery::Retry)
            .with_metadata("key", "value");
        
        assert_eq!(context.operation, "test_operation");
        assert_eq!(context.organism_id, Some("tron_123".to_string()));
        assert!(matches!(context.recovery_strategy, ErrorRecovery::Retry));
        assert_eq!(context.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_contextual_error() {
        let error = GenesisError::OrganismNotFound("tron_123".to_string());
        let context = ErrorContext::new("neural_connect");
        let contextual = ContextualError::new(error, context)
            .with_location("test.rs:42");
        
        let error_string = contextual.to_string();
        assert!(error_string.contains("neural_connect"));
        assert!(error_string.contains("test.rs:42"));
    }

    #[test]
    fn test_error_metrics() {
        let mut metrics = ErrorMetrics::new();
        
        let error = GenesisError::DNA(DNAError::InvalidSecretKey);
        metrics.record_error(&error, Some("tron_123"));
        
        assert_eq!(metrics.total_errors, 1);
        assert_eq!(metrics.errors_by_type.get("DNA"), Some(&1));
        assert_eq!(metrics.errors_by_organism.get("tron_123"), Some(&1));
        
        metrics.record_recovery_attempt(true);
        assert_eq!(metrics.recovery_attempts, 1);
        assert_eq!(metrics.successful_recoveries, 1);
        assert_eq!(metrics.get_recovery_rate(), 1.0);
    }

    #[test]
    fn test_default_error_handler() {
        let handler = DefaultErrorHandler::new();
        
        let neural_error = GenesisError::Neural(crate::neural::SynapseError::ConnectionRefused);
        let context = ErrorContext::new("test");
        
        assert!(matches!(handler.handle_error(&neural_error, &context), ErrorRecovery::Retry));
        assert!(handler.should_retry(&neural_error, 1));
        assert!(!handler.should_retry(&neural_error, 5));
        
        let delay = handler.get_retry_delay(&neural_error, 1);
        assert_eq!(delay, std::time::Duration::from_millis(200));
    }
} 