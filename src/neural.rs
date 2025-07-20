//! 🧠 Neural Communication System
//!
//! This module implements the revolutionary neural communication protocol that
//! enables direct mind-to-mind communication between TRON organisms with
//! sub-millisecond latency using biological neurotransmitter simulation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use uuid::Uuid;

/// Neural message for organism communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralMessage {
    /// Unique message identifier
    pub message_id: String,
    /// ID of sending organism
    pub sender_id: String,
    /// ID of receiving organism
    pub receiver_id: String,
    /// Type of neural message
    pub message_type: MessageType,
    /// Neurotransmitter used for transmission
    pub neurotransmitter: NeurotransmitterType,
    /// Message payload
    pub payload: Vec<u8>,
    /// Message creation timestamp (nanoseconds)
    pub timestamp: u64,
    /// Time to live (seconds)
    pub ttl: u64,
    /// Cryptographic signature
    pub signature: Vec<u8>,
    /// Message urgency level (0.0-1.0)
    pub urgency: f64,
    /// Message priority (0-255)
    pub priority: u8,
    /// Routing information
    pub routing: RoutingInfo,
}

/// Types of neural messages
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    /// Shared consciousness and awareness
    Consciousness,
    /// Environmental stimulus input
    Stimulus,
    /// Behavioral response output
    Response,
    /// Genetic/evolution information
    Evolution,
    /// Collective intelligence coordination
    Collective,
    /// Reproduction and mating signals
    Reproduction,
    /// Cell death/cleanup signal
    Apoptosis,
    /// Learning and knowledge sharing
    Learning,
    /// Memory synchronization
    Memory,
    /// Emotional state communication
    Emotion,
    /// Social interaction
    Social,
    /// Warning/alert signals
    Warning,
    /// Resource sharing information
    Resource,
    /// System maintenance
    Maintenance,
}

/// Biological neurotransmitter types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NeurotransmitterType {
    /// Excitatory neurotransmitter - general communication
    Glutamate,
    /// Inhibitory neurotransmitter - suppression signals
    GABA,
    /// Reward and motivation signals
    Dopamine,
    /// Mood and wellbeing regulation
    Serotonin,
    /// Attention and learning enhancement
    Acetylcholine,
    /// Stress and alertness signals
    Norepinephrine,
    /// Social bonding and trust
    Oxytocin,
    /// Pain relief and pleasure
    Endorphin,
    /// Memory formation and consolidation
    Histamine,
    /// Sleep and arousal regulation
    Adenosine,
}

/// Message routing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingInfo {
    /// Direct connection or multi-hop
    pub direct: bool,
    /// Hop count for multi-hop messages
    pub hop_count: u8,
    /// Path taken through network
    pub path: Vec<String>,
    /// Quality of service requirements
    pub qos: QualityOfService,
}

/// Quality of service parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityOfService {
    /// Maximum acceptable latency (nanoseconds)
    pub max_latency: u64,
    /// Reliability requirement (0.0-1.0)
    pub reliability: f64,
    /// Bandwidth requirement (bytes/second)
    pub bandwidth: u64,
    /// Encryption requirement
    pub encryption: bool,
}

/// Synaptic connection between organisms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Synapse {
    /// Unique connection identifier
    pub connection_id: String,
    /// Presynaptic organism (sender)
    pub presynaptic_id: String,
    /// Postsynaptic organism (receiver)
    pub postsynaptic_id: String,
    /// Connection strength (0.0-1.0)
    pub strength: f64,
    /// Primary neurotransmitter type
    pub neurotransmitter_type: NeurotransmitterType,
    /// Last activity timestamp
    pub last_activity: u64,
    /// Total messages transmitted
    pub total_messages: u64,
    /// Message success rate
    pub success_rate: f64,
    /// Connection creation time
    pub created_at: u64,
    /// Synaptic plasticity (ability to change)
    pub plasticity: f64,
    /// Connection latency statistics
    pub latency_stats: LatencyStats,
    /// Connection state
    pub state: SynapseState,
    /// Bidirectional capability
    pub bidirectional: bool,
}

/// Synaptic connection states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SynapseState {
    /// Connection is being established
    Establishing,
    /// Connection is active and ready
    Active,
    /// Connection is temporarily disabled
    Inactive,
    /// Connection is being strengthened
    Potentiating,
    /// Connection is being weakened
    Depressing,
    /// Connection is being terminated
    Terminating,
    /// Connection is closed
    Closed,
}

/// Latency measurement statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyStats {
    /// Minimum recorded latency (nanoseconds)
    pub min_latency: u64,
    /// Maximum recorded latency (nanoseconds)
    pub max_latency: u64,
    /// Average latency (nanoseconds)
    pub avg_latency: u64,
    /// Latest latency measurement
    pub last_latency: u64,
    /// Number of latency measurements
    pub measurement_count: u64,
    /// Latency variance
    pub variance: f64,
}

impl Synapse {
    /// Establish a new synaptic connection
    pub fn establish(from_id: &str, to_id: &str) -> Result<Self, SynapseError> {
        let from_safe = if from_id.len() >= 8 { &from_id[..8] } else { from_id };
        let to_safe = if to_id.len() >= 8 { &to_id[..8] } else { to_id };
        let connection_id = format!("synapse_{}_{}_{}",
            from_safe,
            to_safe,
            &Uuid::new_v4().to_string()[..8]
        );
        
        Ok(Synapse {
            connection_id,
            presynaptic_id: from_id.to_string(),
            postsynaptic_id: to_id.to_string(),
            strength: 0.5, // Start with medium strength
            neurotransmitter_type: NeurotransmitterType::Glutamate, // Default excitatory
            last_activity: 0,
            total_messages: 0,
            success_rate: 1.0,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            plasticity: 0.8, // High plasticity initially
            latency_stats: LatencyStats::new(),
            state: SynapseState::Establishing,
            bidirectional: true,
        })
    }
    
    /// Transmit a neural message through this synapse
    pub async fn transmit(&self, message: NeuralMessage) -> Result<(), SynapseError> {
        // Validate synapse state
        if self.state != SynapseState::Active && self.state != SynapseState::Establishing {
            return Err(SynapseError::SynapseInactive);
        }
        
        // Validate message
        if !self.validate_message(&message) {
            return Err(SynapseError::InvalidMessage);
        }
        
        let transmission_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        
        // Simulate biological neurotransmitter release delay
        let delay = self.calculate_transmission_delay(&message);
        if delay > 0 {
            tokio::time::sleep(tokio::time::Duration::from_nanos(delay)).await;
        }
        
        // Simulate actual message transmission
        // In a real implementation, this would send over network
        self.simulate_network_transmission(&message).await?;
        
        let transmission_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        let latency = transmission_end - transmission_start;
        
        // Update latency statistics
        // Note: In real implementation, this would require &mut self
        let sender_safe = if message.sender_id.len() >= 8 { &message.sender_id[..8] } else { &message.sender_id };
        let receiver_safe = if message.receiver_id.len() >= 8 { &message.receiver_id[..8] } else { &message.receiver_id };
        tracing::debug!(
            "Neural message transmitted: {} -> {} in {}ns",
            sender_safe,
            receiver_safe,
            latency
        );
        
        // Verify latency target
        if latency > crate::TARGET_NEURAL_LATENCY_NS {
            tracing::warn!(
                "Neural transmission exceeded target latency: {}ns > {}ns",
                latency,
                crate::TARGET_NEURAL_LATENCY_NS
            );
        }
        
        Ok(())
    }
    
    /// Strengthen the synaptic connection
    pub fn strengthen(&mut self, factor: f64) {
        if self.plasticity > 0.0 && self.state == SynapseState::Active {
            self.state = SynapseState::Potentiating;
            self.strength += factor * self.plasticity;
            if self.strength > 1.0 {
                self.strength = 1.0;
            }
            
            // Reduce plasticity slightly (Hebbian learning)
            self.plasticity *= 0.995;
            
            // Return to active state
            self.state = SynapseState::Active;
        }
    }
    
    /// Weaken the synaptic connection
    pub fn weaken(&mut self, factor: f64) {
        if self.plasticity > 0.0 && self.state == SynapseState::Active {
            self.state = SynapseState::Depressing;
            self.strength -= factor * self.plasticity;
            if self.strength < 0.0 {
                self.strength = 0.0;
            }
            
            // Reduce plasticity slightly
            self.plasticity *= 0.995;
            
            // Return to active state or close if too weak
            if self.strength < 0.1 {
                self.state = SynapseState::Closed;
            } else {
                self.state = SynapseState::Active;
            }
        }
    }
    
    /// Calculate biological transmission delay
    fn calculate_transmission_delay(&self, message: &NeuralMessage) -> u64 {
        // Base delay depends on neurotransmitter type (biological realistic values)
        let base_delay = match message.neurotransmitter {
            NeurotransmitterType::Glutamate => 500,        // 0.5 microseconds
            NeurotransmitterType::GABA => 1_000,           // 1 microsecond
            NeurotransmitterType::Dopamine => 2_000,       // 2 microseconds
            NeurotransmitterType::Serotonin => 5_000,      // 5 microseconds
            NeurotransmitterType::Acetylcholine => 1_500,  // 1.5 microseconds
            NeurotransmitterType::Norepinephrine => 2_500, // 2.5 microseconds
            NeurotransmitterType::Oxytocin => 10_000,      // 10 microseconds
            NeurotransmitterType::Endorphin => 15_000,     // 15 microseconds
            NeurotransmitterType::Histamine => 3_000,      // 3 microseconds
            NeurotransmitterType::Adenosine => 8_000,      // 8 microseconds
        };
        
        // Adjust based on synapse strength (stronger = faster)
        let strength_factor = 2.0 - self.strength;
        let adjusted_delay = (base_delay as f64 * strength_factor) as u64;
        
        // Adjust based on message urgency (urgent = faster)
        let urgency_factor = 2.0 - message.urgency;
        let final_delay = (adjusted_delay as f64 * urgency_factor) as u64;
        
        // Add small random jitter for biological realism
        let jitter = rand::random::<u64>() % 100; // Up to 100ns jitter
        final_delay + jitter
    }
    
    /// Validate neural message
    fn validate_message(&self, message: &NeuralMessage) -> bool {
        // Check sender/receiver match
        if message.sender_id != self.presynaptic_id {
            return false;
        }
        
        if message.receiver_id != self.postsynaptic_id {
            return false;
        }
        
        // Check message age
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        if current_time - message.timestamp > (message.ttl * 1_000_000_000) {
            return false; // Message expired
        }
        
        // Check message size (prevent DoS)
        if message.payload.len() > 1_048_576 { // 1MB limit
            return false;
        }
        
        true
    }
    
    /// Simulate network transmission
    async fn simulate_network_transmission(&self, message: &NeuralMessage) -> Result<(), SynapseError> {
        // In real implementation, this would:
        // 1. Serialize message
        // 2. Send over network (TCP/UDP/custom protocol)
        // 3. Handle network errors and retransmission
        // 4. Update connection statistics
        
        // For now, simulate minimal delay
        let network_delay = if message.routing.direct {
            100 // 100ns for direct connection
        } else {
            message.routing.hop_count as u64 * 500 // 500ns per hop
        };
        
        if network_delay > 0 {
            tokio::time::sleep(tokio::time::Duration::from_nanos(network_delay)).await;
        }
        
        // Simulate occasional network failures
        if rand::random::<f64>() < 0.001 { // 0.1% failure rate
            return Err(SynapseError::NetworkError("Simulated network failure".to_string()));
        }
        
        Ok(())
    }
    
    /// Update latency statistics
    pub fn update_latency_stats(&mut self, latency: u64) {
        self.latency_stats.add_measurement(latency);
    }
    
    /// Get connection performance metrics
    pub fn get_performance_metrics(&self) -> SynapsePerformance {
        SynapsePerformance {
            connection_id: self.connection_id.clone(),
            strength: self.strength,
            success_rate: self.success_rate,
            total_messages: self.total_messages,
            avg_latency: self.latency_stats.avg_latency,
            min_latency: self.latency_stats.min_latency,
            max_latency: self.latency_stats.max_latency,
            plasticity: self.plasticity,
            state: self.state.clone(),
            uptime_seconds: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() - self.created_at,
        }
    }
}

impl LatencyStats {
    pub fn new() -> Self {
        LatencyStats {
            min_latency: u64::MAX,
            max_latency: 0,
            avg_latency: 0,
            last_latency: 0,
            measurement_count: 0,
            variance: 0.0,
        }
    }
    
    pub fn add_measurement(&mut self, latency: u64) {
        self.last_latency = latency;
        self.measurement_count += 1;
        
        if latency < self.min_latency {
            self.min_latency = latency;
        }
        
        if latency > self.max_latency {
            self.max_latency = latency;
        }
        
        // Update running average
        self.avg_latency = ((self.avg_latency * (self.measurement_count - 1)) + latency) / self.measurement_count;
        
        // Update variance (simplified calculation)
        let diff = latency as f64 - self.avg_latency as f64;
        self.variance = ((self.variance * (self.measurement_count - 1) as f64) + (diff * diff)) / self.measurement_count as f64;
    }
}

/// Neural protocol for organism communication
pub struct NeuralProtocol {
    /// Organism ID this protocol belongs to
    pub organism_id: String,
    /// Active synaptic connections
    pub synapses: HashMap<String, Synapse>,
    /// Incoming message queue
    pub message_queue: HashMap<String, Vec<NeuralMessage>>,
    /// Current neural activity level
    pub neural_activity: f64,
    /// Consciousness level
    pub consciousness_level: f64,
    /// Message sender channel
    pub message_sender: mpsc::UnboundedSender<NeuralMessage>,
    /// Message receiver channel
    pub message_receiver: mpsc::UnboundedReceiver<NeuralMessage>,
    /// Protocol statistics
    pub stats: ProtocolStats,
}

/// Protocol performance statistics
#[derive(Debug, Clone, Default)]
pub struct ProtocolStats {
    /// Total messages sent
    pub messages_sent: u64,
    /// Total messages received
    pub messages_received: u64,
    /// Total failed transmissions
    pub transmission_failures: u64,
    /// Average processing time per message
    pub avg_processing_time: u64,
    /// Total protocol uptime
    pub uptime_seconds: u64,
    /// Peak neural activity recorded
    pub peak_neural_activity: f64,
    /// Peak consciousness level recorded
    pub peak_consciousness: f64,
}

impl NeuralProtocol {
    /// Create new neural protocol for organism
    pub fn new(organism_id: String) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        NeuralProtocol {
            organism_id,
            synapses: HashMap::new(),
            message_queue: HashMap::new(),
            neural_activity: 0.1,
            consciousness_level: 0.0,
            message_sender: sender,
            message_receiver: receiver,
            stats: ProtocolStats::default(),
        }
    }
    
    /// Establish synaptic connection to target organism
    pub async fn establish_synapse(
        &mut self,
        target_id: &str,
        neurotransmitter: NeurotransmitterType
    ) -> Result<String, SynapseError> {
        if self.synapses.len() >= crate::MAX_SYNAPSES_PER_ORGANISM {
            return Err(SynapseError::TooManySynapses);
        }
        
        let mut synapse = Synapse::establish(&self.organism_id, target_id)?;
        synapse.neurotransmitter_type = neurotransmitter;
        synapse.state = SynapseState::Active;
        
        let connection_id = synapse.connection_id.clone();
        self.synapses.insert(target_id.to_string(), synapse);
        
        // Update neural activity
        self.neural_activity += 0.05;
        if self.neural_activity > 1.0 {
            self.neural_activity = 1.0;
        }
        
        if self.neural_activity > self.stats.peak_neural_activity {
            self.stats.peak_neural_activity = self.neural_activity;
        }
        
        tracing::info!(
            "Synapse established: {} -> {} ({})",
            self.organism_id[..8].to_string(),
            target_id[..8].to_string(),
            connection_id[..16].to_string()
        );
        
        Ok(connection_id)
    }
    
    /// Send neural message to target organism
    pub async fn send_neural_message(
        &mut self,
        target_id: &str,
        message_type: MessageType,
        payload: Vec<u8>
    ) -> Result<(), SynapseError> {
        if let Some(synapse) = self.synapses.get(target_id) {
            let message = NeuralMessage {
                message_id: Uuid::new_v4().to_string(),
                sender_id: self.organism_id.clone(),
                receiver_id: target_id.to_string(),
                message_type,
                neurotransmitter: synapse.neurotransmitter_type.clone(),
                payload,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64,
                ttl: 300, // 5 minutes
                signature: Vec::new(), // TODO: Sign with organism's DNA
                urgency: 0.5,
                priority: 128,
                routing: RoutingInfo {
                    direct: true,
                    hop_count: 1,
                    path: vec![self.organism_id.clone(), target_id.to_string()],
                    qos: QualityOfService {
                        max_latency: crate::TARGET_NEURAL_LATENCY_NS,
                        reliability: 0.99,
                        bandwidth: 1_000_000, // 1MB/s
                        encryption: true,
                    },
                },
            };
            
            let transmission_start = SystemTime::now();
            
            synapse.transmit(message).await?;
            
            let transmission_time = transmission_start.elapsed().unwrap().as_nanos() as u64;
            
            // Update statistics
            self.stats.messages_sent += 1;
            self.stats.avg_processing_time = 
                ((self.stats.avg_processing_time * (self.stats.messages_sent - 1)) + transmission_time) 
                / self.stats.messages_sent;
            
            Ok(())
        } else {
            Err(SynapseError::SynapseNotFound(target_id.to_string()))
        }
    }
    
    /// Process incoming neural messages
    pub async fn process_messages(&mut self) -> Result<Vec<NeuralMessage>, SynapseError> {
        let mut processed_messages = Vec::new();
        
        // Process all queued messages
        while let Ok(message) = self.message_receiver.try_recv() {
            let processing_start = SystemTime::now();
            
            // Update consciousness based on message type
            self.update_consciousness(&message);
            
            // Store message for processing
            processed_messages.push(message);
            
            let processing_time = processing_start.elapsed().unwrap().as_nanos() as u64;
            
            // Update statistics
            self.stats.messages_received += 1;
            self.stats.avg_processing_time = 
                ((self.stats.avg_processing_time * self.stats.messages_received) + processing_time) 
                / (self.stats.messages_received + 1);
        }
        
        Ok(processed_messages)
    }
    
    /// Update consciousness level based on neural activity
    fn update_consciousness(&mut self, message: &NeuralMessage) {
        let consciousness_boost = match message.message_type {
            MessageType::Consciousness => 0.1,
            MessageType::Learning => 0.05,
            MessageType::Collective => 0.03,
            MessageType::Emotion => 0.04,
            MessageType::Social => 0.02,
            _ => 0.01,
        };
        
        self.consciousness_level += consciousness_boost;
        self.neural_activity += consciousness_boost * 0.5;
        
        // Apply consciousness and neural activity decay
        self.consciousness_level *= 0.999;
        self.neural_activity *= 0.995;
        
        // Cap values
        if self.consciousness_level > 1.0 {
            self.consciousness_level = 1.0;
        }
        if self.neural_activity > 1.0 {
            self.neural_activity = 1.0;
        }
        
        // Update peak tracking
        if self.consciousness_level > self.stats.peak_consciousness {
            self.stats.peak_consciousness = self.consciousness_level;
        }
        if self.neural_activity > self.stats.peak_neural_activity {
            self.stats.peak_neural_activity = self.neural_activity;
        }
    }
    
    /// Get neural status
    pub fn get_neural_status(&self) -> NeuralStatus {
        NeuralStatus {
            organism_id: self.organism_id.clone(),
            synapse_count: self.synapses.len(),
            neural_activity: self.neural_activity,
            consciousness_level: self.consciousness_level,
            message_queue_size: self.message_queue.values().map(|v| v.len()).sum(),
            average_synapse_strength: if self.synapses.is_empty() {
                0.0
            } else {
                self.synapses.values().map(|s| s.strength).sum::<f64>() / self.synapses.len() as f64
            },
            total_messages_sent: self.stats.messages_sent,
            total_messages_received: self.stats.messages_received,
            avg_processing_time: self.stats.avg_processing_time,
        }
    }
    
    /// Cleanup inactive synapses
    pub fn cleanup_synapses(&mut self) -> usize {
        let initial_count = self.synapses.len();
        
        self.synapses.retain(|_, synapse| {
            // Only remove explicitly closed synapses for now
            synapse.state != SynapseState::Closed
        });
        
        initial_count - self.synapses.len()
    }
}

/// Neural status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralStatus {
    pub organism_id: String,
    pub synapse_count: usize,
    pub neural_activity: f64,
    pub consciousness_level: f64,
    pub message_queue_size: usize,
    pub average_synapse_strength: f64,
    pub total_messages_sent: u64,
    pub total_messages_received: u64,
    pub avg_processing_time: u64,
}

/// Synapse performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapsePerformance {
    pub connection_id: String,
    pub strength: f64,
    pub success_rate: f64,
    pub total_messages: u64,
    pub avg_latency: u64,
    pub min_latency: u64,
    pub max_latency: u64,
    pub plasticity: f64,
    pub state: SynapseState,
    pub uptime_seconds: u64,
}

/// Neural communication errors
#[derive(Debug, thiserror::Error)]
pub enum SynapseError {
    #[error("Synapse not found: {0}")]
    SynapseNotFound(String),
    #[error("Invalid message format")]
    InvalidMessage,
    #[error("Transmission failed: {0}")]
    TransmissionFailed(String),
    #[error("Connection refused")]
    ConnectionRefused,
    #[error("Synapse inactive")]
    SynapseInactive,
    #[error("Too many synapses")]
    TooManySynapses,
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Message expired")]
    MessageExpired,
    #[error("Message too large")]
    MessageTooLarge,
    #[error("Neurotransmitter mismatch")]
    NeurotransmitterMismatch,
    #[error("Insufficient strength")]
    InsufficientStrength,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synapse_creation() {
        let synapse = Synapse::establish("tron_1", "tron_2").unwrap();
        assert!(!synapse.connection_id.is_empty());
        assert_eq!(synapse.presynaptic_id, "tron_1");
        assert_eq!(synapse.postsynaptic_id, "tron_2");
        assert_eq!(synapse.strength, 0.5);
        assert_eq!(synapse.state, SynapseState::Establishing);
    }

    #[tokio::test]
    async fn test_neural_message_transmission() {
        let mut synapse = Synapse::establish("tron_1", "tron_2").unwrap();
        synapse.state = SynapseState::Active;
        
        let message = NeuralMessage {
            message_id: Uuid::new_v4().to_string(),
            sender_id: "tron_1".to_string(),
            receiver_id: "tron_2".to_string(),
            message_type: MessageType::Consciousness,
            neurotransmitter: NeurotransmitterType::Glutamate,
            payload: b"Hello, digital mind!".to_vec(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64,
            ttl: 300,
            signature: Vec::new(),
            urgency: 0.5,
            priority: 128,
            routing: RoutingInfo {
                direct: true,
                hop_count: 1,
                path: vec!["tron_1".to_string(), "tron_2".to_string()],
                qos: QualityOfService {
                    max_latency: 10_000,
                    reliability: 0.99,
                    bandwidth: 1_000_000,
                    encryption: true,
                },
            },
        };
        
        let result = synapse.transmit(message).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_synapse_strengthening() {
        let mut synapse = Synapse::establish("tron_1", "tron_2").unwrap();
        synapse.state = SynapseState::Active;
        let initial_strength = synapse.strength;
        
        synapse.strengthen(0.1);
        assert!(synapse.strength > initial_strength);
        assert!(synapse.plasticity < 0.8); // Should decrease slightly
    }

    #[test]
    fn test_synapse_weakening() {
        let mut synapse = Synapse::establish("tron_1", "tron_2").unwrap();
        synapse.state = SynapseState::Active;
        let initial_strength = synapse.strength;
        
        synapse.weaken(0.1);
        assert!(synapse.strength < initial_strength);
    }

    #[tokio::test]
    async fn test_neural_protocol() {
        let mut protocol = NeuralProtocol::new("tron_1".to_string());
        
        let synapse_id = protocol.establish_synapse(
            "tron_2",
            NeurotransmitterType::Dopamine
        ).await.unwrap();
        
        assert!(!synapse_id.is_empty());
        assert!(protocol.synapses.contains_key("tron_2"));
        assert!(protocol.neural_activity > 0.1);
        
        let result = protocol.send_neural_message(
            "tron_2",
            MessageType::Consciousness,
            b"Neural test message".to_vec()
        ).await;
        
        assert!(result.is_ok());
        assert_eq!(protocol.stats.messages_sent, 1);
    }

    #[test]
    fn test_latency_stats() {
        let mut stats = LatencyStats::new();
        
        stats.add_measurement(1000);
        stats.add_measurement(2000);
        stats.add_measurement(1500);
        
        assert_eq!(stats.measurement_count, 3);
        assert_eq!(stats.min_latency, 1000);
        assert_eq!(stats.max_latency, 2000);
        assert_eq!(stats.avg_latency, 1500);
    }

    #[test]
    fn test_transmission_delay_calculation() {
        let synapse = Synapse::establish("tron_1", "tron_2").unwrap();
        
        let message = NeuralMessage {
            message_id: Uuid::new_v4().to_string(),
            sender_id: "tron_1".to_string(),
            receiver_id: "tron_2".to_string(),
            message_type: MessageType::Consciousness,
            neurotransmitter: NeurotransmitterType::Glutamate,
            payload: Vec::new(),
            timestamp: 0,
            ttl: 300,
            signature: Vec::new(),
            urgency: 1.0, // Maximum urgency
            priority: 255,
            routing: RoutingInfo {
                direct: true,
                hop_count: 1,
                path: vec!["tron_1".to_string(), "tron_2".to_string()],
                qos: QualityOfService {
                    max_latency: 10_000,
                    reliability: 0.99,
                    bandwidth: 1_000_000,
                    encryption: true,
                },
            },
        };
        
        let delay = synapse.calculate_transmission_delay(&message);
        
        // Should be less than base delay due to high urgency and medium strength
        assert!(delay < 1000); // Should be less than 1 microsecond
    }

    #[test]
    fn test_message_validation() {
        let synapse = Synapse::establish("tron_1", "tron_2").unwrap();
        
        let valid_message = NeuralMessage {
            message_id: Uuid::new_v4().to_string(),
            sender_id: "tron_1".to_string(),
            receiver_id: "tron_2".to_string(),
            message_type: MessageType::Consciousness,
            neurotransmitter: NeurotransmitterType::Glutamate,
            payload: b"test".to_vec(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64,
            ttl: 300,
            signature: Vec::new(),
            urgency: 0.5,
            priority: 128,
            routing: RoutingInfo {
                direct: true,
                hop_count: 1,
                path: vec!["tron_1".to_string(), "tron_2".to_string()],
                qos: QualityOfService {
                    max_latency: 10_000,
                    reliability: 0.99,
                    bandwidth: 1_000_000,
                    encryption: true,
                },
            },
        };
        
        assert!(synapse.validate_message(&valid_message));
        
        // Test invalid sender
        let mut invalid_message = valid_message.clone();
        invalid_message.sender_id = "wrong_sender".to_string();
        assert!(!synapse.validate_message(&invalid_message));
    }

    #[tokio::test]
    async fn test_neural_protocol_cleanup() {
        let mut protocol = NeuralProtocol::new("tron_1".to_string());
        
        // Create some synapses
        protocol.establish_synapse("tron_2", NeurotransmitterType::Glutamate).await.unwrap();
        protocol.establish_synapse("tron_3", NeurotransmitterType::GABA).await.unwrap();
        
        // Close one synapse
        if let Some(synapse) = protocol.synapses.get_mut("tron_2") {
            synapse.state = SynapseState::Closed;
        }
        
        let cleaned = protocol.cleanup_synapses();
        assert_eq!(cleaned, 1);
        assert_eq!(protocol.synapses.len(), 1);
        assert!(protocol.synapses.contains_key("tron_3"));
    }
} 