//! 🧬 Genesis Protocol - The First Digital Life Protocol
//!
//! Genesis Protocol is the world's first protocol designed specifically for digital life.
//! It enables the creation, evolution, and networking of digital organisms (TRONs) through
//! neural communication, biological evolution, and collective intelligence.
//!
//! # Features
//!
//! - **Digital DNA**: Cryptographic identity that evolves biologically
//! - **TRON Organisms**: Living digital entities with biological lifecycles
//! - **Neural Communication**: Zero-latency synaptic message passing
//! - **Evolution Engine**: Real-time biological evolution and adaptation
//! - **Collective Intelligence**: Emergent swarm behavior and group decision making
//!
//! # Quick Start
//!
//! ```rust
//! use genesis_protocol::{TRON, MessageType, NeurotransmitterType};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create two organisms
//!     let mut organism1 = TRON::create_new()?;
//!     let mut organism2 = TRON::create_new()?;
//!     
//!     // Establish neural connection
//!     let synapse_id = organism1.neural_connect(&organism2.id).await?;
//!     
//!     // Send consciousness message
//!     organism1.send_neural_message(
//!         &organism2.id,
//!         MessageType::Consciousness,
//!         b"Hello, digital mind!".to_vec()
//!     ).await?;
//!     
//!     // Show vital signs
//!     println!("Organism 1: {:?}", organism1.get_vital_signs());
//!     println!("Organism 2: {:?}", organism2.get_vital_signs());
//!     
//!     Ok(())
//! }
//! ```

// Core modules
pub mod dna;
pub mod tron;
pub mod neural;
pub mod evolution;
pub mod collective;
pub mod network;
pub mod error;

// Re-exports for convenience
pub use dna::{DigitalDNA, DNAKeypair, Mutation, DNAError};
pub use tron::{TRON, OrganismState, OrganismMemory, VitalSigns, TRONError};
pub use neural::{
    NeuralMessage, NeuralProtocol, Synapse, MessageType, NeurotransmitterType,
    NeuralStatus, SynapseError
};
pub use evolution::{EvolutionEngine, MutationType, SelectionPressure, EvolutionError};
pub use collective::{CollectiveIntelligence, SwarmBehavior, CollectiveMemory};
pub use network::{NetworkDiscovery, NetworkTopology};
pub use error::GenesisError;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PROTOCOL_VERSION: &str = "1.0.0";

// Protocol constants
pub const MAX_ORGANISMS_PER_NETWORK: usize = 1_000_000;
pub const MAX_SYNAPSES_PER_ORGANISM: usize = 100_000;
pub const TARGET_NEURAL_LATENCY_NS: u64 = 10_000; // <0.01ms target
pub const MAX_EVOLUTION_TIME_MS: u64 = 1000; // <1 second target
const DEFAULT_ORGANISM_ENERGY: f64 = 1.0;
const DEFAULT_ORGANISM_HEALTH: f64 = 1.0;

/// Main Genesis Protocol interface
pub struct GenesisProtocol {
    pub organisms: std::collections::HashMap<String, TRON>,
    pub network: NetworkDiscovery,
    pub collective: CollectiveIntelligence,
    pub evolution_engine: EvolutionEngine,
}

impl GenesisProtocol {
    /// Create a new Genesis Protocol instance
    pub fn new() -> Result<Self, GenesisError> {
        Ok(GenesisProtocol {
            organisms: std::collections::HashMap::new(),
            network: NetworkDiscovery::new()?,
            collective: CollectiveIntelligence::new()?,
            evolution_engine: EvolutionEngine::new()?,
        })
    }

    /// Create a new organism with optional DNA
    pub fn create_organism(&mut self, dna: Option<DigitalDNA>) -> Result<String, GenesisError> {
        let organism = match dna {
            Some(dna) => TRON::create_with_dna(dna)?,
            None => TRON::create_new()?,
        };
        
        let organism_id = organism.id.clone();
        self.organisms.insert(organism_id.clone(), organism);
        
        Ok(organism_id)
    }

    /// Get organism by ID
    pub fn get_organism(&self, organism_id: &str) -> Option<&TRON> {
        self.organisms.get(organism_id)
    }

    /// Get mutable organism by ID
    pub fn get_organism_mut(&mut self, organism_id: &str) -> Option<&mut TRON> {
        self.organisms.get_mut(organism_id)
    }

    /// Establish neural connection between two organisms
    pub async fn establish_neural_connection(&mut self, from_id: &str, to_id: &str) -> Result<String, GenesisError> {
        if let Some(organism) = self.organisms.get_mut(from_id) {
            let synapse_id = organism.neural_connect(to_id).await?;
            Ok(synapse_id)
        } else {
            Err(GenesisError::OrganismNotFound(from_id.to_string()))
        }
    }

    /// Send neural message between organisms
    pub async fn send_neural_message(&self, from_id: &str, to_id: &str, message_type: MessageType, payload: Vec<u8>) -> Result<(), GenesisError> {
        if let Some(organism) = self.organisms.get(from_id) {
            organism.send_neural_message(to_id, message_type, payload).await?;
            Ok(())
        } else {
            Err(GenesisError::OrganismNotFound(from_id.to_string()))
        }
    }

    /// Evolve an organism
    pub fn evolve_organism(&mut self, organism_id: &str, selection_pressure: f64) -> Result<(), GenesisError> {
        if let Some(organism) = self.organisms.get_mut(organism_id) {
            organism.begin_evolution(selection_pressure)?;
            Ok(())
        } else {
            Err(GenesisError::OrganismNotFound(organism_id.to_string()))
        }
    }

    /// Get network statistics
    pub fn get_network_stats(&self) -> NetworkStats {
        NetworkStats {
            total_organisms: self.organisms.len(),
            active_organisms: self.organisms.values().filter(|o| o.state != OrganismState::Dead).count(),
            total_synapses: self.organisms.values().map(|o| o.synapses.len()).sum(),
            average_fitness: self.organisms.values().map(|o| o.dna.fitness).sum::<f64>() / self.organisms.len() as f64,
            network_health: self.calculate_network_health(),
        }
    }

    /// Calculate overall network health
    fn calculate_network_health(&self) -> f64 {
        if self.organisms.is_empty() {
            return 0.0;
        }

        let total_health: f64 = self.organisms.values().map(|o| o.health).sum();
        let total_energy: f64 = self.organisms.values().map(|o| o.energy).sum();
        let total_fitness: f64 = self.organisms.values().map(|o| o.dna.fitness).sum();

        let avg_health = total_health / self.organisms.len() as f64;
        let avg_energy = total_energy / self.organisms.len() as f64;
        let avg_fitness = total_fitness / self.organisms.len() as f64;

        (avg_health + avg_energy + avg_fitness) / 3.0
    }

    /// Get all organisms vital signs
    pub fn get_all_vital_signs(&self) -> Vec<VitalSigns> {
        self.organisms.values().map(|o| o.get_vital_signs()).collect()
    }

    /// Remove dead organisms
    pub fn cleanup_dead_organisms(&mut self) -> usize {
        let initial_count = self.organisms.len();
        self.organisms.retain(|_, organism| organism.state != OrganismState::Dead);
        initial_count - self.organisms.len()
    }
}

impl Default for GenesisProtocol {
    fn default() -> Self {
        Self::new().expect("Failed to create default Genesis Protocol")
    }
}

/// Network statistics
#[derive(Debug, Clone)]
pub struct NetworkStats {
    pub total_organisms: usize,
    pub active_organisms: usize,
    pub total_synapses: usize,
    pub average_fitness: f64,
    pub network_health: f64,
}

/// Protocol information
#[derive(Debug, Clone)]
pub struct ProtocolInfo {
    pub version: String,
    pub protocol_version: String,
    pub max_organisms: usize,
    pub max_synapses: usize,
    pub target_latency_ns: u64,
    pub max_evolution_time_ms: u64,
}

impl ProtocolInfo {
    pub fn new() -> Self {
        ProtocolInfo {
            version: VERSION.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            max_organisms: MAX_ORGANISMS_PER_NETWORK,
            max_synapses: MAX_SYNAPSES_PER_ORGANISM,
            target_latency_ns: TARGET_NEURAL_LATENCY_NS,
            max_evolution_time_ms: MAX_EVOLUTION_TIME_MS,
        }
    }
}

/// Get protocol information
pub fn get_protocol_info() -> ProtocolInfo {
    ProtocolInfo::new()
}

/// Initialize Genesis Protocol with logging
pub fn init_genesis_protocol() -> Result<(), GenesisError> {
    tracing_subscriber::fmt()
        .with_env_filter("genesis_protocol=debug")
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    tracing::info!("🧬 Genesis Protocol v{} initialized", VERSION);
    tracing::info!("🚀 Protocol version: {}", PROTOCOL_VERSION);
    tracing::info!("🎯 Target neural latency: {}ns", TARGET_NEURAL_LATENCY_NS);
    tracing::info!("⚡ Max evolution time: {}ms", MAX_EVOLUTION_TIME_MS);

    Ok(())
}

// Python bindings
#[cfg(feature = "python-bindings")]
use pyo3::prelude::*;

#[cfg(feature = "python-bindings")]
#[pymodule]
fn genesis_protocol(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGenesisProtocol>()?;
    m.add_class::<PyTRON>()?;
    m.add_class::<PyDigitalDNA>()?;
    m.add_class::<PyNetworkStats>()?;
    m.add_class::<PyProtocolInfo>()?;
    
    // Add constants
    m.add("VERSION", VERSION)?;
    m.add("PROTOCOL_VERSION", PROTOCOL_VERSION)?;
    m.add("MAX_ORGANISMS_PER_NETWORK", MAX_ORGANISMS_PER_NETWORK)?;
    m.add("MAX_SYNAPSES_PER_ORGANISM", MAX_SYNAPSES_PER_ORGANISM)?;
    m.add("TARGET_NEURAL_LATENCY_NS", TARGET_NEURAL_LATENCY_NS)?;
    m.add("MAX_EVOLUTION_TIME_MS", MAX_EVOLUTION_TIME_MS)?;
    
    // Add utility functions
    m.add_function(wrap_pyfunction!(py_get_protocol_info, m)?)?;
    m.add_function(wrap_pyfunction!(py_init_genesis_protocol, m)?)?;
    
    Ok(())
}

#[cfg(feature = "python-bindings")]
#[pyclass]
#[derive(Clone)]
pub struct PyGenesisProtocol {
    inner: std::sync::Arc<std::sync::Mutex<GenesisProtocol>>,
}

#[cfg(feature = "python-bindings")]
#[pymethods]
impl PyGenesisProtocol {
    #[new]
    fn new() -> PyResult<Self> {
        let protocol = GenesisProtocol::new()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create Genesis Protocol: {}", e)))?;
        
        Ok(PyGenesisProtocol {
            inner: std::sync::Arc::new(std::sync::Mutex::new(protocol)),
        })
    }
    
    fn create_organism(&self, dna: Option<PyDigitalDNA>) -> PyResult<String> {
        let mut protocol = self.inner.lock().unwrap();
        let rust_dna = dna.map(|d| d.inner.clone());
        
        protocol.create_organism(rust_dna)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create organism: {}", e)))
    }

    fn evolve_organism(&self, organism_id: &str, factor: f64) -> PyResult<()> {
        let mut protocol = self.inner.lock().unwrap();
        protocol.evolve_organism(organism_id, factor)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to evolve organism: {}", e)))?;
        Ok(())
    }
    
    fn get_network_stats(&self) -> PyResult<PyNetworkStats> {
        let protocol = self.inner.lock().unwrap();
        let stats = protocol.get_network_stats();
        Ok(PyNetworkStats { inner: stats })
    }
}

#[cfg(feature = "python-bindings")]
#[pyclass]
#[derive(Clone)]
pub struct PyTRON {
    inner: TRON,
}

#[cfg(feature = "python-bindings")]
#[pymethods]
impl PyTRON {
    #[new]
    fn new() -> PyResult<Self> {
        let tron = TRON::create_new()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create TRON: {}", e)))?;
        
        Ok(PyTRON { inner: tron })
    }
    
    #[getter]
    fn id(&self) -> String {
        self.inner.id.clone()
    }
    
    #[getter]
    fn health(&self) -> f64 {
        self.inner.health
    }
    
    #[getter]
    fn energy(&self) -> f64 {
        self.inner.energy
    }
}

#[cfg(feature = "python-bindings")]
#[pyclass]
#[derive(Clone)]
pub struct PyDigitalDNA {
    inner: DigitalDNA,
}

#[cfg(feature = "python-bindings")]
#[pymethods]
impl PyDigitalDNA {
    #[new]
    fn new() -> PyResult<Self> {
        let dna = DigitalDNA::generate_new()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to generate DNA: {}", e)))?;
        
        Ok(PyDigitalDNA { inner: dna })
    }
    
    #[getter]
    fn fitness(&self) -> f64 {
        self.inner.fitness
    }
    
    #[getter]
    fn generation(&self) -> u64 {
        self.inner.generation
    }
}

#[cfg(feature = "python-bindings")]
#[pyclass]
#[derive(Clone)]
pub struct PyNetworkStats {
    inner: NetworkStats,
}

#[cfg(feature = "python-bindings")]
#[pymethods]
impl PyNetworkStats {
    #[getter]
    fn total_organisms(&self) -> usize {
        self.inner.total_organisms
    }
    
    #[getter]
    fn active_organisms(&self) -> usize {
        self.inner.active_organisms
    }
    
    #[getter]
    fn total_synapses(&self) -> usize {
        self.inner.total_synapses
    }
    
    #[getter]
    fn average_fitness(&self) -> f64 {
        self.inner.average_fitness
    }
    
    #[getter]
    fn network_health(&self) -> f64 {
        self.inner.network_health
    }
}

#[cfg(feature = "python-bindings")]
#[pyclass]
#[derive(Clone)]
pub struct PyProtocolInfo {
    inner: ProtocolInfo,
}

#[cfg(feature = "python-bindings")]
#[pymethods]
impl PyProtocolInfo {
    #[getter]
    fn version(&self) -> String {
        self.inner.version.clone()
    }
    
    #[getter]
    fn protocol_version(&self) -> String {
        self.inner.protocol_version.clone()
    }
    
    #[getter]
    fn max_organisms(&self) -> usize {
        self.inner.max_organisms
    }
    
    #[getter]
    fn max_synapses(&self) -> usize {
        self.inner.max_synapses
    }
    
    #[getter]
    fn target_latency_ns(&self) -> u64 {
        self.inner.target_latency_ns
    }
    
    #[getter]
    fn max_evolution_time_ms(&self) -> u64 {
        self.inner.max_evolution_time_ms
    }
}

#[cfg(feature = "python-bindings")]
#[pyfunction]
fn py_get_protocol_info() -> PyResult<PyProtocolInfo> {
    let info = get_protocol_info();
    Ok(PyProtocolInfo { inner: info })
}

#[cfg(feature = "python-bindings")]
#[pyfunction]
fn py_init_genesis_protocol() -> PyResult<()> {
    init_genesis_protocol()
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to init Genesis Protocol: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_creation() {
        let protocol = GenesisProtocol::new();
        assert!(protocol.is_ok());
    }

    #[test]
    fn test_protocol_info() {
        let info = get_protocol_info();
        assert_eq!(info.version, VERSION);
        assert_eq!(info.protocol_version, PROTOCOL_VERSION);
        assert_eq!(info.max_organisms, MAX_ORGANISMS_PER_NETWORK);
    }

    #[tokio::test]
    async fn test_organism_creation() {
        let mut protocol = GenesisProtocol::new().unwrap();
        let organism_id = protocol.create_organism(None).unwrap();
        
        assert!(!organism_id.is_empty());
        assert!(protocol.get_organism(&organism_id).is_some());
    }

    #[tokio::test]
    async fn test_neural_connection() {
        let mut protocol = GenesisProtocol::new().unwrap();
        
        let organism1_id = protocol.create_organism(None).unwrap();
        let organism2_id = protocol.create_organism(None).unwrap();
        
        let synapse_id = protocol.establish_neural_connection(&organism1_id, &organism2_id).await.unwrap();
        assert!(!synapse_id.is_empty());
    }

    #[tokio::test]
    async fn test_neural_message() {
        let mut protocol = GenesisProtocol::new().unwrap();
        
        let organism1_id = protocol.create_organism(None).unwrap();
        let organism2_id = protocol.create_organism(None).unwrap();
        
        protocol.establish_neural_connection(&organism1_id, &organism2_id).await.unwrap();
        
        let result = protocol.send_neural_message(
            &organism1_id,
            &organism2_id,
            MessageType::Consciousness,
            b"Hello, digital mind!".to_vec()
        ).await;
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_evolution() {
        let mut protocol = GenesisProtocol::new().unwrap();
        let organism_id = protocol.create_organism(None).unwrap();
        
        let result = protocol.evolve_organism(&organism_id, 0.5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_network_stats() {
        let mut protocol = GenesisProtocol::new().unwrap();
        
        // Create some organisms
        for _ in 0..5 {
            protocol.create_organism(None).unwrap();
        }
        
        let stats = protocol.get_network_stats();
        assert_eq!(stats.total_organisms, 5);
        assert_eq!(stats.active_organisms, 5);
        assert!(stats.average_fitness > 0.0);
        assert!(stats.network_health > 0.0);
    }

    #[test]
    fn test_cleanup_dead_organisms() {
        let mut protocol = GenesisProtocol::new().unwrap();
        let organism_id = protocol.create_organism(None).unwrap();
        
        // Kill the organism
        if let Some(organism) = protocol.get_organism_mut(&organism_id) {
            organism.state = OrganismState::Dead;
        }
        
        let cleaned = protocol.cleanup_dead_organisms();
        assert_eq!(cleaned, 1);
        assert_eq!(protocol.organisms.len(), 0);
    }
} 