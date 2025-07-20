//! 🌐 Network Discovery and Topology
//!
//! This module implements network discovery, topology management, and
//! organism networking capabilities for the Genesis Protocol.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

/// Network discovery and management system
#[derive(Debug, Clone)]
pub struct NetworkDiscovery {
    /// Known organisms in the network
    pub known_organisms: HashMap<String, OrganismNode>,
    /// Network topology
    pub topology: NetworkTopology,
    /// Discovery metrics
    pub discovery_metrics: DiscoveryMetrics,
    /// Network configuration
    pub config: NetworkConfig,
}

/// Network topology manager
#[derive(Debug, Clone)]
pub struct NetworkTopology {
    /// Network nodes
    pub nodes: HashMap<String, NetworkNode>,
    /// Network connections
    pub connections: HashMap<String, NetworkConnection>,
    /// Network clusters
    pub clusters: HashMap<String, NetworkCluster>,
    /// Topology metrics
    pub metrics: TopologyMetrics,
}

/// Individual organism node in the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganismNode {
    /// Organism ID
    pub organism_id: String,
    /// Network address
    pub address: SocketAddr,
    /// Node capabilities
    pub capabilities: NodeCapabilities,
    /// Node status
    pub status: NodeStatus,
    /// Last seen timestamp
    pub last_seen: u64,
    /// Connection quality
    pub connection_quality: f64,
    /// Trust level
    pub trust_level: f64,
    /// Performance metrics
    pub performance: NodePerformance,
}

/// Network node (can represent multiple organisms)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkNode {
    /// Node ID
    pub node_id: String,
    /// Node address
    pub address: SocketAddr,
    /// Organisms hosted on this node
    pub organisms: Vec<String>,
    /// Node type
    pub node_type: NodeType,
    /// Node capabilities
    pub capabilities: NodeCapabilities,
    /// Node status
    pub status: NodeStatus,
    /// Resource usage
    pub resource_usage: ResourceUsage,
}

/// Network connection between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnection {
    /// Connection ID
    pub connection_id: String,
    /// Source node
    pub source_node: String,
    /// Target node
    pub target_node: String,
    /// Connection type
    pub connection_type: ConnectionType,
    /// Connection quality metrics
    pub quality: ConnectionQuality,
    /// Connection established time
    pub established_at: u64,
}

/// Network cluster (group of related nodes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkCluster {
    /// Cluster ID
    pub cluster_id: String,
    /// Cluster members
    pub members: Vec<String>,
    /// Cluster purpose
    pub purpose: String,
    /// Cluster coordination node
    pub coordinator: Option<String>,
    /// Cluster health
    pub health: f64,
}

/// Node capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapabilities {
    /// Maximum organisms supported
    pub max_organisms: usize,
    /// Maximum connections
    pub max_connections: usize,
    /// Supported protocols
    pub protocols: Vec<String>,
    /// Computing resources
    pub computing_power: f64,
    /// Memory capacity
    pub memory_capacity: u64,
    /// Network bandwidth
    pub bandwidth: u64,
}

/// Node status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeStatus {
    /// Node is online and active
    Online,
    /// Node is offline
    Offline,
    /// Node is connecting
    Connecting,
    /// Node is disconnecting
    Disconnecting,
    /// Node is under maintenance
    Maintenance,
    /// Node is experiencing issues
    Degraded,
}

/// Node type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    /// Full node (hosts organisms)
    Full,
    /// Relay node (forwards messages)
    Relay,
    /// Gateway node (connects to external networks)
    Gateway,
    /// Bootstrap node (helps with discovery)
    Bootstrap,
    /// Archive node (stores historical data)
    Archive,
}

/// Connection type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    /// Direct peer-to-peer connection
    Direct,
    /// Relayed connection through intermediary
    Relayed,
    /// Gateway connection to external network
    Gateway,
    /// Cluster internal connection
    Cluster,
}

/// Connection quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionQuality {
    /// Round-trip time (milliseconds)
    pub rtt: f64,
    /// Packet loss rate (0.0-1.0)
    pub packet_loss: f64,
    /// Bandwidth (bytes per second)
    pub bandwidth: u64,
    /// Reliability score (0.0-1.0)
    pub reliability: f64,
    /// Connection stability
    pub stability: f64,
}

/// Node performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePerformance {
    /// Messages processed per second
    pub messages_per_second: f64,
    /// Average response time (milliseconds)
    pub avg_response_time: f64,
    /// Uptime percentage
    pub uptime: f64,
    /// Error rate (0.0-1.0)
    pub error_rate: f64,
}

/// Resource usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage percentage
    pub memory_usage: f64,
    /// Network usage percentage
    pub network_usage: f64,
    /// Storage usage percentage
    pub storage_usage: f64,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Discovery interval (seconds)
    pub discovery_interval: u64,
    /// Maximum discovery attempts
    pub max_discovery_attempts: u32,
    /// Connection timeout (seconds)
    pub connection_timeout: u64,
    /// Heartbeat interval (seconds)
    pub heartbeat_interval: u64,
    /// Trust threshold
    pub trust_threshold: f64,
    /// Default port for communication
    pub default_port: u16,
}

/// Discovery metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryMetrics {
    /// Total discovered organisms
    pub total_discovered: usize,
    /// Active connections
    pub active_connections: usize,
    /// Failed connection attempts
    pub failed_connections: u64,
    /// Average discovery time (milliseconds)
    pub avg_discovery_time: f64,
    /// Network coverage percentage
    pub network_coverage: f64,
}

/// Topology metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyMetrics {
    /// Total nodes in network
    pub total_nodes: usize,
    /// Total connections
    pub total_connections: usize,
    /// Network diameter (max hops between any two nodes)
    pub network_diameter: usize,
    /// Average clustering coefficient
    pub clustering_coefficient: f64,
    /// Network density
    pub network_density: f64,
}

impl NetworkDiscovery {
    /// Create new network discovery system
    pub fn new() -> Result<Self, NetworkError> {
        Ok(NetworkDiscovery {
            known_organisms: HashMap::new(),
            topology: NetworkTopology::new(),
            discovery_metrics: DiscoveryMetrics::new(),
            config: NetworkConfig::default(),
        })
    }

    /// Discover organisms in the network
    pub async fn discover_organisms(&mut self) -> Result<Vec<String>, NetworkError> {
        let mut discovered = Vec::new();
        
        // Simulate network discovery process
        // In real implementation, this would:
        // 1. Broadcast discovery messages
        // 2. Listen for responses
        // 3. Validate responding organisms
        // 4. Add them to known organisms
        
        // For now, simulate discovery
        for i in 0..rand::random::<usize>() % 5 + 1 {
            let organism_id = format!("discovered_tron_{}", i);
            let address = format!("127.0.0.1:{}", 8000 + i).parse().unwrap();
            
            let organism_node = OrganismNode {
                organism_id: organism_id.clone(),
                address,
                capabilities: NodeCapabilities::default(),
                status: NodeStatus::Online,
                last_seen: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                connection_quality: 0.8,
                trust_level: 0.5,
                performance: NodePerformance::default(),
            };
            
            self.known_organisms.insert(organism_id.clone(), organism_node);
            discovered.push(organism_id);
        }
        
        self.discovery_metrics.total_discovered = self.known_organisms.len();
        
        Ok(discovered)
    }

    /// Connect to an organism
    pub async fn connect_to_organism(&mut self, organism_id: &str) -> Result<(), NetworkError> {
        if let Some(organism) = self.known_organisms.get_mut(organism_id) {
            if organism.status == NodeStatus::Offline {
                return Err(NetworkError::OrganismOffline(organism_id.to_string()));
            }
            
            // Simulate connection process
            organism.status = NodeStatus::Connecting;
            
            // Simulate connection delay
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            organism.status = NodeStatus::Online;
            organism.connection_quality = 0.9;
            organism.last_seen = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            
            self.discovery_metrics.active_connections += 1;
            
            Ok(())
        } else {
            Err(NetworkError::OrganismNotFound(organism_id.to_string()))
        }
    }

    /// Update network topology
    pub fn update_topology(&mut self) {
        // Update topology based on known organisms
        self.topology.update_from_organisms(&self.known_organisms);
        
        // Calculate topology metrics
        self.topology.calculate_metrics();
    }

    /// Get network statistics
    pub fn get_network_stats(&self) -> NetworkStats {
        NetworkStats {
            total_organisms: self.known_organisms.len(),
            online_organisms: self.known_organisms.values()
                .filter(|o| o.status == NodeStatus::Online)
                .count(),
            total_nodes: self.topology.nodes.len(),
            total_connections: self.topology.connections.len(),
            network_health: self.calculate_network_health(),
            average_connection_quality: self.calculate_average_connection_quality(),
        }
    }

    /// Calculate overall network health
    fn calculate_network_health(&self) -> f64 {
        if self.known_organisms.is_empty() {
            return 0.0;
        }
        
        let total_quality: f64 = self.known_organisms.values()
            .map(|o| o.connection_quality)
            .sum();
        
        let online_count = self.known_organisms.values()
            .filter(|o| o.status == NodeStatus::Online)
            .count();
        
        let availability = online_count as f64 / self.known_organisms.len() as f64;
        let average_quality = total_quality / self.known_organisms.len() as f64;
        
        (availability + average_quality) / 2.0
    }

    /// Calculate average connection quality
    fn calculate_average_connection_quality(&self) -> f64 {
        if self.known_organisms.is_empty() {
            return 0.0;
        }
        
        let total_quality: f64 = self.known_organisms.values()
            .map(|o| o.connection_quality)
            .sum();
        
        total_quality / self.known_organisms.len() as f64
    }
}

impl NetworkTopology {
    pub fn new() -> Self {
        NetworkTopology {
            nodes: HashMap::new(),
            connections: HashMap::new(),
            clusters: HashMap::new(),
            metrics: TopologyMetrics::new(),
        }
    }

    /// Update topology from organism information
    pub fn update_from_organisms(&mut self, organisms: &HashMap<String, OrganismNode>) {
        // Create nodes from organisms
        for (organism_id, organism) in organisms {
            let node = NetworkNode {
                node_id: organism_id.clone(),
                address: organism.address,
                organisms: vec![organism_id.clone()],
                node_type: NodeType::Full,
                capabilities: organism.capabilities.clone(),
                status: organism.status.clone(),
                resource_usage: ResourceUsage::default(),
            };
            
            self.nodes.insert(organism_id.clone(), node);
        }
        
        // Create connections between nodes (simplified)
        self.create_connections();
    }

    /// Create connections between nodes
    fn create_connections(&mut self) {
        let node_ids: Vec<String> = self.nodes.keys().cloned().collect();
        
        for i in 0..node_ids.len() {
            for j in i + 1..node_ids.len() {
                let connection_id = format!("conn_{}_{}", i, j);
                let connection = NetworkConnection {
                    connection_id: connection_id.clone(),
                    source_node: node_ids[i].clone(),
                    target_node: node_ids[j].clone(),
                    connection_type: ConnectionType::Direct,
                    quality: ConnectionQuality::default(),
                    established_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                };
                
                self.connections.insert(connection_id, connection);
            }
        }
    }

    /// Calculate topology metrics
    pub fn calculate_metrics(&mut self) {
        self.metrics.total_nodes = self.nodes.len();
        self.metrics.total_connections = self.connections.len();
        
        // Calculate network density
        let max_connections = if self.nodes.len() > 1 {
            self.nodes.len() * (self.nodes.len() - 1) / 2
        } else {
            1
        };
        
        self.metrics.network_density = self.connections.len() as f64 / max_connections as f64;
        
        // Simplified clustering coefficient
        self.metrics.clustering_coefficient = 0.7; // Placeholder
        
        // Simplified network diameter
        self.metrics.network_diameter = (self.nodes.len() as f64).sqrt() as usize;
    }
}

impl Default for NodeCapabilities {
    fn default() -> Self {
        NodeCapabilities {
            max_organisms: 100,
            max_connections: 1000,
            protocols: vec!["genesis-neural".to_string()],
            computing_power: 1.0,
            memory_capacity: 1_000_000_000, // 1GB
            bandwidth: 100_000_000,         // 100MB/s
        }
    }
}

impl Default for NodePerformance {
    fn default() -> Self {
        NodePerformance {
            messages_per_second: 1000.0,
            avg_response_time: 10.0,
            uptime: 99.9,
            error_rate: 0.001,
        }
    }
}

impl Default for ResourceUsage {
    fn default() -> Self {
        ResourceUsage {
            cpu_usage: 25.0,
            memory_usage: 30.0,
            network_usage: 15.0,
            storage_usage: 10.0,
        }
    }
}

impl Default for ConnectionQuality {
    fn default() -> Self {
        ConnectionQuality {
            rtt: 10.0,
            packet_loss: 0.001,
            bandwidth: 100_000_000,
            reliability: 0.999,
            stability: 0.95,
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            discovery_interval: 30,
            max_discovery_attempts: 3,
            connection_timeout: 10,
            heartbeat_interval: 5,
            trust_threshold: 0.7,
            default_port: 8000,
        }
    }
}

impl DiscoveryMetrics {
    fn new() -> Self {
        DiscoveryMetrics {
            total_discovered: 0,
            active_connections: 0,
            failed_connections: 0,
            avg_discovery_time: 0.0,
            network_coverage: 0.0,
        }
    }
}

impl TopologyMetrics {
    fn new() -> Self {
        TopologyMetrics {
            total_nodes: 0,
            total_connections: 0,
            network_diameter: 0,
            clustering_coefficient: 0.0,
            network_density: 0.0,
        }
    }
}

/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub total_organisms: usize,
    pub online_organisms: usize,
    pub total_nodes: usize,
    pub total_connections: usize,
    pub network_health: f64,
    pub average_connection_quality: f64,
}

/// Network-related errors
#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    #[error("Organism not found: {0}")]
    OrganismNotFound(String),
    #[error("Organism offline: {0}")]
    OrganismOffline(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Discovery failed: {0}")]
    DiscoveryFailed(String),
    #[error("Network timeout")]
    NetworkTimeout,
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    #[error("Protocol not supported: {0}")]
    ProtocolNotSupported(String),
    #[error("Network overloaded")]
    NetworkOverloaded,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_discovery_creation() {
        let discovery = NetworkDiscovery::new().unwrap();
        assert_eq!(discovery.known_organisms.len(), 0);
        assert_eq!(discovery.topology.nodes.len(), 0);
    }

    #[tokio::test]
    async fn test_organism_discovery() {
        let mut discovery = NetworkDiscovery::new().unwrap();
        
        let discovered = discovery.discover_organisms().await.unwrap();
        assert!(discovered.len() > 0);
        assert_eq!(discovery.known_organisms.len(), discovered.len());
        
        for organism_id in &discovered {
            assert!(discovery.known_organisms.contains_key(organism_id));
        }
    }

    #[tokio::test]
    async fn test_organism_connection() {
        let mut discovery = NetworkDiscovery::new().unwrap();
        
        let discovered = discovery.discover_organisms().await.unwrap();
        if let Some(organism_id) = discovered.first() {
            let result = discovery.connect_to_organism(organism_id).await;
            assert!(result.is_ok());
            assert_eq!(discovery.known_organisms[organism_id].status, NodeStatus::Online);
        }
    }

    #[test]
    fn test_topology_update() {
        let mut discovery = NetworkDiscovery::new().unwrap();
        
        // Add some organisms
        let organism1 = OrganismNode {
            organism_id: "tron_1".to_string(),
            address: "127.0.0.1:8000".parse().unwrap(),
            capabilities: NodeCapabilities::default(),
            status: NodeStatus::Online,
            last_seen: 0,
            connection_quality: 0.8,
            trust_level: 0.7,
            performance: NodePerformance::default(),
        };
        
        discovery.known_organisms.insert("tron_1".to_string(), organism1);
        
        discovery.update_topology();
        
        assert_eq!(discovery.topology.nodes.len(), 1);
    }

    #[test]
    fn test_network_stats() {
        let mut discovery = NetworkDiscovery::new().unwrap();
        
        // Add organisms
        for i in 0..3 {
            let organism = OrganismNode {
                organism_id: format!("tron_{}", i),
                address: format!("127.0.0.1:{}", 8000 + i).parse().unwrap(),
                capabilities: NodeCapabilities::default(),
                status: NodeStatus::Online,
                last_seen: 0,
                connection_quality: 0.8,
                trust_level: 0.7,
                performance: NodePerformance::default(),
            };
            
            discovery.known_organisms.insert(format!("tron_{}", i), organism);
        }
        
        let stats = discovery.get_network_stats();
        assert_eq!(stats.total_organisms, 3);
        assert_eq!(stats.online_organisms, 3);
        assert!(stats.network_health > 0.0);
    }
} 