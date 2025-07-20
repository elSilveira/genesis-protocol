//! 🤖 TRON Organisms - Living Digital Entities
//!
//! This module implements TRON (Transcendent Recursive Organism Network) organisms,
//! which are the living digital entities at the heart of Genesis Protocol.
//! Each TRON has its own DNA, lifecycle, neural connections, and adaptive behaviors.

use crate::dna::DigitalDNA;
use crate::neural::{NeuralMessage, Synapse, MessageType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// TRON Organism - A living digital entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TRON {
    /// Unique identifier for this organism
    pub id: String,
    /// Digital DNA containing genetic information
    pub dna: DigitalDNA,
    /// Current lifecycle state
    pub state: OrganismState,
    /// Age in evolution cycles
    pub age: u64,
    /// Available energy (0.0-1.0)
    pub energy: f64,
    /// Health status (0.0-1.0)
    pub health: f64,
    /// Neural connections to other organisms
    pub synapses: HashMap<String, Synapse>,
    /// Organism's memory system
    pub memory: OrganismMemory,
    /// Learned behaviors
    pub behaviors: Vec<Behavior>,
    /// Last evolution timestamp
    pub last_evolution: u64,
    /// Current neural activity level (0.0-1.0)
    pub neural_activity: f64,
    /// Readiness to reproduce (0.0-1.0)
    pub reproduction_readiness: f64,
    /// Organism's consciousness level (0.0-1.0)
    pub consciousness_level: f64,
    /// Social connections and relationships
    pub social_network: SocialNetwork,
    /// Performance metrics
    pub performance: PerformanceMetrics,
}

/// Organism lifecycle states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrganismState {
    /// Just created, initializing systems
    Birth,
    /// Learning and developing capabilities
    Growing,
    /// Fully functional and active
    Mature,
    /// Creating offspring
    Reproducing,
    /// Declining capabilities due to age
    Aging,
    /// End of lifecycle approaching
    Dying,
    /// Cleanup required
    Dead,
}

/// Organism memory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganismMemory {
    /// Short-term working memory
    pub short_term: HashMap<String, MemoryEntry>,
    /// Long-term persistent memory
    pub long_term: HashMap<String, MemoryEntry>,
    /// Episodic memories (experiences)
    pub episodic: Vec<EpisodicMemory>,
    /// Procedural memories (learned skills)
    pub procedural: Vec<ProceduralMemory>,
    /// Memory capacity limit
    pub capacity: usize,
    /// Current memory usage
    pub usage: usize,
}

/// Individual memory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Memory content
    pub content: serde_json::Value,
    /// Memory strength (0.0-1.0)
    pub strength: f64,
    /// When memory was created
    pub created_at: u64,
    /// Last time memory was accessed
    pub last_accessed: u64,
    /// Number of times accessed
    pub access_count: u64,
    /// Emotional weight of memory
    pub emotional_weight: f64,
}

/// Episodic memory (experiences)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemory {
    /// Unique episode ID
    pub episode_id: String,
    /// Description of the experience
    pub description: String,
    /// Participants in the experience
    pub participants: Vec<String>,
    /// When the experience occurred
    pub timestamp: u64,
    /// Emotional impact of the experience
    pub emotional_impact: f64,
    /// Lessons learned from experience
    pub lessons: Vec<String>,
    /// Success/failure rating
    pub outcome_rating: f64,
}

/// Procedural memory (skills)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralMemory {
    /// Skill identifier
    pub skill_id: String,
    /// Skill name
    pub skill_name: String,
    /// Proficiency level (0.0-1.0)
    pub proficiency: f64,
    /// How often skill is used
    pub usage_frequency: f64,
    /// When skill was learned
    pub learned_at: u64,
    /// Skill improvement rate
    pub improvement_rate: f64,
}

/// Learned behaviors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Behavior {
    /// Unique behavior ID
    pub behavior_id: String,
    /// Behavior name
    pub name: String,
    /// What triggers this behavior
    pub trigger: BehaviorTrigger,
    /// What action to take
    pub action: BehaviorAction,
    /// Success rate of this behavior
    pub success_rate: f64,
    /// When behavior was learned
    pub learned_at: u64,
    /// How often behavior is used
    pub usage_count: u64,
    /// Confidence in this behavior
    pub confidence: f64,
}

/// Behavior triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BehaviorTrigger {
    /// Neural message received
    NeuralMessage { message_type: MessageType },
    /// Energy level threshold
    EnergyLevel { threshold: f64, above: bool },
    /// Health level threshold
    HealthLevel { threshold: f64, above: bool },
    /// Social interaction
    SocialInteraction { interaction_type: String },
    /// Environmental change
    EnvironmentalChange { change_type: String },
    /// Time-based trigger
    TimeBased { interval_seconds: u64 },
    /// Custom condition
    Custom { condition: String },
}

/// Behavior actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BehaviorAction {
    /// Send neural message
    SendMessage { target: String, message_type: MessageType, content: String },
    /// Seek energy/resources
    SeekResources,
    /// Reproduce with another organism
    Reproduce { target: String },
    /// Evolve/mutate
    Evolve,
    /// Rest/recuperate
    Rest,
    /// Explore environment
    Explore,
    /// Socialize with others
    Socialize { targets: Vec<String> },
    /// Learn new skill
    Learn { skill: String },
    /// Custom action
    Custom { action: String, parameters: HashMap<String, serde_json::Value> },
}

/// Social network connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialNetwork {
    /// Friends/allies
    pub friends: HashMap<String, Relationship>,
    /// Enemies/competitors
    pub enemies: HashMap<String, Relationship>,
    /// Family/offspring
    pub family: HashMap<String, FamilyRelation>,
    /// Professional connections
    pub colleagues: HashMap<String, Relationship>,
    /// Social reputation score
    pub reputation: f64,
    /// Trust level for new organisms
    pub default_trust: f64,
}

/// Relationship with another organism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    /// Relationship strength (0.0-1.0)
    pub strength: f64,
    /// Trust level (0.0-1.0)
    pub trust: f64,
    /// Number of interactions
    pub interactions: u64,
    /// Last interaction time
    pub last_interaction: u64,
    /// Relationship type
    pub relationship_type: RelationshipType,
}

/// Types of relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Friend,
    Enemy,
    Neutral,
    Mentor,
    Student,
    Competitor,
    Collaborator,
}

/// Family relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyRelation {
    /// Type of family relationship
    pub relation_type: FamilyType,
    /// Genetic similarity
    pub genetic_similarity: f64,
    /// Emotional bond strength
    pub bond_strength: f64,
}

/// Family relationship types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FamilyType {
    Parent,
    Child,
    Sibling,
    Grandparent,
    Grandchild,
    Cousin,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Task completion rate
    pub task_success_rate: f64,
    /// Communication effectiveness
    pub communication_success: f64,
    /// Learning speed
    pub learning_rate: f64,
    /// Adaptation speed
    pub adaptation_speed: f64,
    /// Social skills
    pub social_effectiveness: f64,
    /// Resource efficiency
    pub resource_efficiency: f64,
    /// Problem-solving ability
    pub problem_solving_score: f64,
}

impl TRON {
    /// Create a new TRON organism with given DNA
    pub fn create_with_dna(dna: DigitalDNA) -> Result<Self, TRONError> {
        let id = format!("tron_{}", dna.get_hash()[..16].to_string());
        
        Ok(TRON {
            id,
            dna,
            state: OrganismState::Birth,
            age: 0,
            energy: 1.0,
            health: 1.0,
            synapses: HashMap::new(),
            memory: OrganismMemory::new(),
            behaviors: Vec::new(),
            last_evolution: 0,
            neural_activity: 0.1,
            reproduction_readiness: 0.0,
            consciousness_level: 0.1,
            social_network: SocialNetwork::new(),
            performance: PerformanceMetrics::new(),
        })
    }
    
    /// Create a new TRON organism with random DNA
    pub fn create_new() -> Result<Self, TRONError> {
        let dna = DigitalDNA::generate_new()
            .map_err(|e| TRONError::DNAGenerationFailed(e.to_string()))?;
        Self::create_with_dna(dna)
    }
    
    /// Establish neural connection with another organism
    pub async fn neural_connect(&mut self, target_id: &str) -> Result<String, TRONError> {
        if self.synapses.len() >= crate::MAX_SYNAPSES_PER_ORGANISM {
            return Err(TRONError::TooManySynapses);
        }
        
        let synapse = Synapse::establish(&self.id, target_id)
            .map_err(|e| TRONError::NeuralConnectionFailed(e.to_string()))?;
        
        let synapse_id = synapse.connection_id.clone();
        self.synapses.insert(target_id.to_string(), synapse);
        
        // Update neural activity and consciousness
        self.neural_activity += 0.1;
        self.consciousness_level += 0.05;
        if self.neural_activity > 1.0 {
            self.neural_activity = 1.0;
        }
        if self.consciousness_level > 1.0 {
            self.consciousness_level = 1.0;
        }
        
        // Record this as a social interaction
        self.social_network.add_or_update_relationship(
            target_id,
            RelationshipType::Neutral,
            0.1
        );
        
        Ok(synapse_id)
    }
    
    /// Send neural message to another organism
    pub async fn send_neural_message(
        &self,
        target_id: &str,
        message_type: MessageType,
        payload: Vec<u8>
    ) -> Result<(), TRONError> {
        if let Some(synapse) = self.synapses.get(target_id) {
            let signature = self.dna.sign_data(&payload)
                .map_err(|e| TRONError::SigningFailed(e.to_string()))?;
            
            let message = NeuralMessage {
                message_id: uuid::Uuid::new_v4().to_string(),
                sender_id: self.id.clone(),
                receiver_id: target_id.to_string(),
                message_type,
                neurotransmitter: synapse.neurotransmitter_type.clone(),
                payload,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64,
                ttl: 300, // 5 minutes
                signature,
                urgency: 0.5,
                priority: 128,
                routing: crate::neural::RoutingInfo {
                    direct: true,
                    hop_count: 1,
                    path: vec![self.id.clone(), target_id.to_string()],
                    qos: crate::neural::QualityOfService {
                        max_latency: crate::TARGET_NEURAL_LATENCY_NS,
                        reliability: 0.99,
                        bandwidth: 1_000_000,
                        encryption: true,
                    },
                },
            };
            
            synapse.transmit(message).await
                .map_err(|e| TRONError::NeuralTransmissionFailed(e.to_string()))?;
            
            Ok(())
        } else {
            Err(TRONError::SynapseNotFound(target_id.to_string()))
        }
    }
    
    /// Begin evolution process
    pub fn begin_evolution(&mut self, selection_pressure: f64) -> Result<(), TRONError> {
        // Check if organism is fit enough to evolve
        if self.dna.fitness < selection_pressure {
            return Err(TRONError::EvolutionFailed("Insufficient fitness".to_string()));
        }
        
        // Apply random mutation
        let mutation = self.dna.generate_random_mutation();
        self.dna.mutate(mutation)
            .map_err(|e| TRONError::EvolutionFailed(e.to_string()))?;
        
        // Update organism properties
        self.age += 1;
        self.last_evolution = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Update lifecycle state based on age and fitness
        self.update_lifecycle_state();
        
        // Update performance metrics
        self.performance.adaptation_speed = (self.performance.adaptation_speed * 0.9) + (0.1 * 1.0);
        
        // Record evolution in episodic memory
        self.memory.add_episodic_memory(
            "Evolution",
            "Organism successfully evolved",
            vec![],
            0.8,
            vec!["Adaptation improves survival".to_string()],
            0.8
        );
        
        Ok(())
    }
    
    /// Update lifecycle state based on age, health, and other factors
    fn update_lifecycle_state(&mut self) {
        let previous_state = self.state.clone();
        
        match self.age {
            0..=10 => {
                self.state = OrganismState::Growing;
            },
            11..=50 => {
                if self.health > 0.8 && self.energy > 0.7 {
                    self.state = OrganismState::Mature;
                } else {
                    self.state = OrganismState::Growing;
                }
            },
            51..=80 => {
                if self.reproduction_readiness > 0.8 && self.health > 0.6 {
                    self.state = OrganismState::Reproducing;
                } else if self.health > 0.5 {
                    self.state = OrganismState::Mature;
                } else {
                    self.state = OrganismState::Aging;
                }
            },
            81..=100 => {
                if self.health > 0.3 {
                    self.state = OrganismState::Aging;
                } else {
                    self.state = OrganismState::Dying;
                }
            },
            _ => {
                self.state = OrganismState::Dying;
            }
        }
        
        // Update reproduction readiness based on state
        match self.state {
            OrganismState::Mature => {
                self.reproduction_readiness += 0.1;
                if self.reproduction_readiness > 1.0 {
                    self.reproduction_readiness = 1.0;
                }
            },
            OrganismState::Aging | OrganismState::Dying => {
                self.reproduction_readiness *= 0.9;
            },
            _ => {},
        }
        
        // Record state change if it occurred
        if previous_state != self.state {
            self.memory.add_episodic_memory(
                "Lifecycle Change",
                &format!("Transitioned from {:?} to {:?}", previous_state, self.state),
                vec![],
                0.6,
                vec![format!("Age affects lifecycle: {} cycles", self.age)],
                0.7
            );
        }
    }
    
    /// Reproduce with another organism
    pub fn reproduce_with(&self, partner: &TRON) -> Result<TRON, TRONError> {
        // Check reproductive readiness
        if self.state != OrganismState::Reproducing && self.state != OrganismState::Mature {
            return Err(TRONError::ReproductionNotReady("Self not ready".to_string()));
        }
        
        if partner.state != OrganismState::Reproducing && partner.state != OrganismState::Mature {
            return Err(TRONError::ReproductionNotReady("Partner not ready".to_string()));
        }
        
        if self.reproduction_readiness < 0.5 || partner.reproduction_readiness < 0.5 {
            return Err(TRONError::ReproductionNotReady("Insufficient readiness".to_string()));
        }
        
        // Check genetic compatibility
        let genetic_distance = self.dna.genetic_distance(&partner.dna);
        if genetic_distance > 0.8 {
            return Err(TRONError::GeneticIncompatibility);
        }
        
        // Create offspring through DNA crossover
        let offspring_dna = self.dna.crossover(&partner.dna)
            .map_err(|e| TRONError::ReproductionFailed(e.to_string()))?;
        
        let mut offspring = TRON::create_with_dna(offspring_dna)?;
        
        // Inherit behaviors from both parents
        offspring.inherit_behaviors(self, partner);
        
        // Inherit social connections
        offspring.inherit_social_network(self, partner);
        
        // Set initial family relationships
        offspring.social_network.family.insert(
            self.id.clone(),
            FamilyRelation {
                relation_type: FamilyType::Parent,
                genetic_similarity: 0.5,
                bond_strength: 0.8,
            }
        );
        
        offspring.social_network.family.insert(
            partner.id.clone(),
            FamilyRelation {
                relation_type: FamilyType::Parent,
                genetic_similarity: 0.5,
                bond_strength: 0.8,
            }
        );
        
        Ok(offspring)
    }
    
    /// Inherit behaviors from parents
    fn inherit_behaviors(&mut self, parent1: &TRON, parent2: &TRON) {
        // Inherit successful behaviors from both parents
        for behavior in &parent1.behaviors {
            if behavior.success_rate > 0.7 && rand::random::<f64>() > 0.5 {
                let mut inherited = behavior.clone();
                inherited.behavior_id = uuid::Uuid::new_v4().to_string();
                inherited.confidence *= 0.8; // Inherited behaviors start with lower confidence
                inherited.usage_count = 0;
                inherited.learned_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                self.behaviors.push(inherited);
            }
        }
        
        for behavior in &parent2.behaviors {
            if behavior.success_rate > 0.7 && 
               rand::random::<f64>() > 0.5 && 
               !self.behaviors.iter().any(|b| b.name == behavior.name) {
                let mut inherited = behavior.clone();
                inherited.behavior_id = uuid::Uuid::new_v4().to_string();
                inherited.confidence *= 0.8;
                inherited.usage_count = 0;
                inherited.learned_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                self.behaviors.push(inherited);
            }
        }
    }
    
    /// Inherit social network from parents
    fn inherit_social_network(&mut self, parent1: &TRON, parent2: &TRON) {
        // Start with neutral trust
        self.social_network.default_trust = (parent1.social_network.default_trust + parent2.social_network.default_trust) / 2.0;
        
        // Inherit some friend relationships with reduced strength
        for (friend_id, relationship) in &parent1.social_network.friends {
            if relationship.strength > 0.6 && rand::random::<f64>() > 0.7 {
                self.social_network.friends.insert(
                    friend_id.clone(),
                    Relationship {
                        strength: relationship.strength * 0.3,
                        trust: relationship.trust * 0.5,
                        interactions: 0,
                        last_interaction: 0,
                        relationship_type: RelationshipType::Neutral,
                    }
                );
            }
        }
        
        for (friend_id, relationship) in &parent2.social_network.friends {
            if relationship.strength > 0.6 && 
               rand::random::<f64>() > 0.7 && 
               !self.social_network.friends.contains_key(friend_id) {
                self.social_network.friends.insert(
                    friend_id.clone(),
                    Relationship {
                        strength: relationship.strength * 0.3,
                        trust: relationship.trust * 0.5,
                        interactions: 0,
                        last_interaction: 0,
                        relationship_type: RelationshipType::Neutral,
                    }
                );
            }
        }
    }
    
    /// Get vital signs of the organism
    pub fn get_vital_signs(&self) -> VitalSigns {
        VitalSigns {
            organism_id: self.id.clone(),
            age: self.age,
            energy: self.energy,
            health: self.health,
            neural_activity: self.neural_activity,
            synapse_count: self.synapses.len(),
            memory_usage: self.memory.get_usage_percentage(),
            fitness: self.dna.fitness,
            state: self.state.clone(),
            consciousness_level: self.consciousness_level,
            reproduction_readiness: self.reproduction_readiness,
            social_connections: self.social_network.get_connection_count(),
            behavior_count: self.behaviors.len(),
        }
    }
    
    /// Process behavioral triggers and execute actions
    pub async fn process_behaviors(&mut self) -> Result<Vec<String>, TRONError> {
        let mut executed_behaviors = Vec::new();
        let mut behaviors_to_update = Vec::new();
        
        // First pass: check triggers and collect behaviors to execute
        for (index, behavior) in self.behaviors.iter().enumerate() {
            if self.should_execute_behavior(&behavior.trigger) {
                behaviors_to_update.push((index, behavior.action.clone()));
            }
        }
        
        // Second pass: execute behaviors and update stats
        for (index, action) in behaviors_to_update {
            match self.execute_behavior_action(&action).await {
                Ok(_) => {
                    if let Some(behavior) = self.behaviors.get_mut(index) {
                        behavior.usage_count += 1;
                        behavior.success_rate = (behavior.success_rate * 0.9) + (0.1 * 1.0);
                        behavior.confidence = (behavior.confidence * 0.95) + (0.05 * 1.0);
                        executed_behaviors.push(behavior.name.clone());
                    }
                },
                Err(_) => {
                    if let Some(behavior) = self.behaviors.get_mut(index) {
                        behavior.success_rate *= 0.9;
                        behavior.confidence *= 0.9;
                    }
                }
            }
        }
        
        Ok(executed_behaviors)
    }
    
    /// Check if a behavior should be executed
    fn should_execute_behavior(&self, trigger: &BehaviorTrigger) -> bool {
        match trigger {
            BehaviorTrigger::EnergyLevel { threshold, above } => {
                if *above {
                    self.energy > *threshold
                } else {
                    self.energy < *threshold
                }
            },
            BehaviorTrigger::HealthLevel { threshold, above } => {
                if *above {
                    self.health > *threshold
                } else {
                    self.health < *threshold
                }
            },
            BehaviorTrigger::TimeBased { interval_seconds } => {
                let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                (current_time - self.last_evolution) >= *interval_seconds
            },
            // Add more trigger evaluations as needed
            _ => false,
        }
    }
    
    /// Execute a behavior action
    async fn execute_behavior_action(&mut self, action: &BehaviorAction) -> Result<(), TRONError> {
        match action {
            BehaviorAction::SeekResources => {
                self.energy += 0.1;
                if self.energy > 1.0 {
                    self.energy = 1.0;
                }
                Ok(())
            },
            BehaviorAction::Rest => {
                self.health += 0.1;
                self.energy += 0.05;
                if self.health > 1.0 { self.health = 1.0; }
                if self.energy > 1.0 { self.energy = 1.0; }
                Ok(())
            },
            BehaviorAction::Evolve => {
                self.begin_evolution(0.5)
            },
            BehaviorAction::Explore => {
                self.consciousness_level += 0.02;
                self.performance.learning_rate += 0.01;
                if self.consciousness_level > 1.0 { self.consciousness_level = 1.0; }
                Ok(())
            },
            // Add more action implementations as needed
            _ => Ok(()),
        }
    }
    
    /// Update organism state based on time and interactions
    pub fn update(&mut self, delta_time: f64) {
        // Age-related decline
        let _age_factor = 1.0 - (self.age as f64 / 1000.0);
        self.health *= 0.9999; // Very slow health decline
        self.energy *= 0.999;  // Slow energy decline
        
        // Neural activity decay
        self.neural_activity *= 0.99;
        
        // Update memory
        self.memory.consolidate_memories();
        
        // Update performance metrics
        self.performance.update(delta_time);
        
        // Update lifecycle state
        self.update_lifecycle_state();
    }
}

/// Vital signs summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalSigns {
    pub organism_id: String,
    pub age: u64,
    pub energy: f64,
    pub health: f64,
    pub neural_activity: f64,
    pub synapse_count: usize,
    pub memory_usage: f64,
    pub fitness: f64,
    pub state: OrganismState,
    pub consciousness_level: f64,
    pub reproduction_readiness: f64,
    pub social_connections: usize,
    pub behavior_count: usize,
}

impl OrganismMemory {
    pub fn new() -> Self {
        OrganismMemory {
            short_term: HashMap::new(),
            long_term: HashMap::new(),
            episodic: Vec::new(),
            procedural: Vec::new(),
            capacity: 10000, // 10K memory entries
            usage: 0,
        }
    }
    
    pub fn store_memory(&mut self, key: String, content: serde_json::Value, importance: f64) {
        let entry = MemoryEntry {
            content,
            strength: importance,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            last_accessed: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            access_count: 0,
            emotional_weight: 0.5,
        };
        
        if importance > 0.7 {
            self.long_term.insert(key, entry);
        } else {
            self.short_term.insert(key, entry);
        }
        
        self.usage = self.short_term.len() + self.long_term.len() + self.episodic.len() + self.procedural.len();
        
        // Trigger memory consolidation if near capacity
        if self.usage > self.capacity * 9 / 10 {
            self.consolidate_memories();
        }
    }
    
    pub fn add_episodic_memory(
        &mut self,
        description: &str,
        _details: &str,
        participants: Vec<String>,
        emotional_impact: f64,
        lessons: Vec<String>,
        outcome_rating: f64
    ) {
        let episode = EpisodicMemory {
            episode_id: uuid::Uuid::new_v4().to_string(),
            description: description.to_string(),
            participants,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            emotional_impact,
            lessons,
            outcome_rating,
        };
        
        self.episodic.push(episode);
        self.usage += 1;
    }
    
    pub fn consolidate_memories(&mut self) {
        // Move important short-term memories to long-term
        let mut to_promote = Vec::new();
        
        for (key, entry) in &self.short_term {
            // Lower thresholds for consolidation
            if entry.access_count > 3 || entry.strength > 0.7 || entry.emotional_weight > 0.6 {
                to_promote.push((key.clone(), entry.clone()));
            }
        }
        
        for (key, entry) in to_promote {
            self.short_term.remove(&key);
            self.long_term.insert(key, entry);
        }
        
        // Forget old, unimportant short-term memories
        let cutoff_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() - 3600; // 1 hour ago
        
        self.short_term.retain(|_, entry| {
            entry.last_accessed > cutoff_time || entry.strength > 0.5 || entry.emotional_weight > 0.6
        });
        
        // Remove old episodic memories with low impact
        self.episodic.retain(|episode| {
            let age = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - episode.timestamp;
            episode.emotional_impact > 0.3 || age < 86400 // Keep high-impact or recent memories
        });
        
        self.usage = self.short_term.len() + self.long_term.len() + self.episodic.len() + self.procedural.len();
    }
    
    pub fn get_usage_percentage(&self) -> f64 {
        (self.usage as f64 / self.capacity as f64) * 100.0
    }
}

impl SocialNetwork {
    pub fn new() -> Self {
        SocialNetwork {
            friends: HashMap::new(),
            enemies: HashMap::new(),
            family: HashMap::new(),
            colleagues: HashMap::new(),
            reputation: 0.5,
            default_trust: 0.3,
        }
    }
    
    pub fn add_or_update_relationship(&mut self, organism_id: &str, relationship_type: RelationshipType, strength_change: f64) {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        match relationship_type {
            RelationshipType::Friend => {
                let relationship = self.friends.entry(organism_id.to_string()).or_insert(Relationship {
                    strength: 0.0,
                    trust: self.default_trust,
                    interactions: 0,
                    last_interaction: current_time,
                    relationship_type: RelationshipType::Friend,
                });
                
                relationship.strength = (relationship.strength + strength_change).max(0.0).min(1.0);
                relationship.interactions += 1;
                relationship.last_interaction = current_time;
            },
            RelationshipType::Enemy => {
                let relationship = self.enemies.entry(organism_id.to_string()).or_insert(Relationship {
                    strength: 0.0,
                    trust: 0.0,
                    interactions: 0,
                    last_interaction: current_time,
                    relationship_type: RelationshipType::Enemy,
                });
                
                relationship.strength = (relationship.strength + strength_change).max(0.0).min(1.0);
                relationship.interactions += 1;
                relationship.last_interaction = current_time;
            },
            _ => {
                let relationship = self.colleagues.entry(organism_id.to_string()).or_insert(Relationship {
                    strength: 0.0,
                    trust: self.default_trust,
                    interactions: 0,
                    last_interaction: current_time,
                    relationship_type,
                });
                
                relationship.strength = (relationship.strength + strength_change).max(0.0).min(1.0);
                relationship.interactions += 1;
                relationship.last_interaction = current_time;
            }
        }
    }
    
    pub fn get_connection_count(&self) -> usize {
        self.friends.len() + self.enemies.len() + self.family.len() + self.colleagues.len()
    }
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        PerformanceMetrics {
            task_success_rate: 0.5,
            communication_success: 0.5,
            learning_rate: 0.1,
            adaptation_speed: 0.1,
            social_effectiveness: 0.3,
            resource_efficiency: 0.5,
            problem_solving_score: 0.3,
        }
    }
    
    pub fn update(&mut self, _delta_time: f64) {
        // Gradual improvement in learning
        self.learning_rate = (self.learning_rate * 0.999) + (0.001 * 0.1);
        
        // Performance metrics slowly decay without activity
        self.task_success_rate *= 0.9999;
        self.communication_success *= 0.9999;
        self.social_effectiveness *= 0.9999;
    }
}

/// TRON-related errors
#[derive(Debug, thiserror::Error)]
pub enum TRONError {
    #[error("DNA generation failed: {0}")]
    DNAGenerationFailed(String),
    #[error("Neural connection failed: {0}")]
    NeuralConnectionFailed(String),
    #[error("Neural transmission failed: {0}")]
    NeuralTransmissionFailed(String),
    #[error("Synapse not found: {0}")]
    SynapseNotFound(String),
    #[error("Evolution failed: {0}")]
    EvolutionFailed(String),
    #[error("Reproduction not ready: {0}")]
    ReproductionNotReady(String),
    #[error("Reproduction failed: {0}")]
    ReproductionFailed(String),
    #[error("Genetic incompatibility")]
    GeneticIncompatibility,
    #[error("Too many synapses")]
    TooManySynapses,
    #[error("Signing failed: {0}")]
    SigningFailed(String),
    #[error("Behavior execution failed: {0}")]
    BehaviorExecutionFailed(String),
    #[error("Memory operation failed: {0}")]
    MemoryOperationFailed(String),
    #[error("Social interaction failed: {0}")]
    SocialInteractionFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tron_creation() {
        let tron = TRON::create_new().unwrap();
        assert!(!tron.id.is_empty());
        assert_eq!(tron.state, OrganismState::Birth);
        assert_eq!(tron.age, 0);
        assert_eq!(tron.energy, 1.0);
        assert_eq!(tron.health, 1.0);
    }

    #[tokio::test]
    async fn test_neural_connection() {
        let mut tron1 = TRON::create_new().unwrap();
        let tron2 = TRON::create_new().unwrap();
        
        let synapse_id = tron1.neural_connect(&tron2.id).await.unwrap();
        assert!(!synapse_id.is_empty());
        assert!(tron1.synapses.contains_key(&tron2.id));
        assert!(tron1.neural_activity > 0.1);
    }

    #[tokio::test]
    async fn test_neural_message() {
        let mut tron1 = TRON::create_new().unwrap();
        let tron2 = TRON::create_new().unwrap();
        
        tron1.neural_connect(&tron2.id).await.unwrap();
        
        let result = tron1.send_neural_message(
            &tron2.id,
            MessageType::Consciousness,
            b"Hello, digital mind!".to_vec()
        ).await;
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_evolution() {
        let mut tron = TRON::create_new().unwrap();
        let initial_fitness = tron.dna.fitness;
        let initial_generation = tron.dna.generation;
        
        let result = tron.begin_evolution(0.5);
        assert!(result.is_ok());
        assert_eq!(tron.age, 1);
        assert_eq!(tron.dna.generation, initial_generation + 1);
        assert!(tron.dna.fitness < initial_fitness); // Mutation cost
    }

    #[test]
    fn test_reproduction() {
        let mut tron1 = TRON::create_new().unwrap();
        let mut tron2 = TRON::create_new().unwrap();
        
        // Make DNAs more compatible by adjusting one based on the other
        let mut compatible_dna = tron1.dna.clone();
        
        // Modify only small parts of the sequence to ensure compatibility
        for i in 0..10 {
            if i < compatible_dna.sequence.len() {
                compatible_dna.sequence[i] = tron2.dna.sequence[i % tron2.dna.sequence.len()];
            }
        }
        
        // Update the second tron's DNA to be compatible
        tron2.dna = compatible_dna;
        
        // Set organisms to reproductive state
        tron1.state = OrganismState::Reproducing;
        tron2.state = OrganismState::Reproducing;
        tron1.reproduction_readiness = 0.8;
        tron2.reproduction_readiness = 0.8;
        
        let offspring = tron1.reproduce_with(&tron2).unwrap();
        
        assert!(!offspring.id.is_empty());
        assert_ne!(offspring.id, tron1.id);
        assert_ne!(offspring.id, tron2.id);
        assert_eq!(offspring.state, OrganismState::Birth);
        assert!(offspring.social_network.family.contains_key(&tron1.id));
        assert!(offspring.social_network.family.contains_key(&tron2.id));
    }

    #[test]
    fn test_lifecycle_progression() {
        let mut tron = TRON::create_new().unwrap();
        
        // Simulate aging
        for _ in 0..15 {
            tron.begin_evolution(0.3).unwrap();
        }
        
        assert_eq!(tron.state, OrganismState::Mature);
        assert_eq!(tron.age, 15);
    }

    #[test]
    fn test_memory_system() {
        let mut tron = TRON::create_new().unwrap();
        
        // Add a memory
        tron.memory.store_memory(
            "test_memory".to_string(),
            serde_json::json!({"content": "test"}),
            0.8
        );
        
        assert!(tron.memory.long_term.contains_key("test_memory"));
        
        // Add episodic memory
        tron.memory.add_episodic_memory(
            "First neural connection",
            "Connected to another organism",
            vec!["tron_123".to_string()],
            0.7,
            vec!["Social connections are important".to_string()],
            0.8
        );
        
        assert_eq!(tron.memory.episodic.len(), 1);
    }

    #[test]
    fn test_social_network() {
        let mut tron = TRON::create_new().unwrap();
        
        tron.social_network.add_or_update_relationship(
            "friend_123",
            RelationshipType::Friend,
            0.5
        );
        
        assert!(tron.social_network.friends.contains_key("friend_123"));
        assert_eq!(tron.social_network.get_connection_count(), 1);
    }

    #[test]
    fn test_vital_signs() {
        let tron = TRON::create_new().unwrap();
        let vital_signs = tron.get_vital_signs();
        
        assert_eq!(vital_signs.organism_id, tron.id);
        assert_eq!(vital_signs.age, tron.age);
        assert_eq!(vital_signs.energy, tron.energy);
        assert_eq!(vital_signs.health, tron.health);
        assert_eq!(vital_signs.state, tron.state);
    }

    #[tokio::test]
    async fn test_behavior_execution() {
        let mut tron = TRON::create_new().unwrap();
        
        // Add a behavior that seeks resources when energy is low
        let behavior = Behavior {
            behavior_id: uuid::Uuid::new_v4().to_string(),
            name: "Seek Energy".to_string(),
            trigger: BehaviorTrigger::EnergyLevel { threshold: 0.3, above: false },
            action: BehaviorAction::SeekResources,
            success_rate: 0.8,
            learned_at: 0,
            usage_count: 0,
            confidence: 0.7,
        };
        
        tron.behaviors.push(behavior);
        tron.energy = 0.2; // Low energy to trigger behavior
        
        let executed = tron.process_behaviors().await.unwrap();
        assert_eq!(executed.len(), 1);
        assert_eq!(executed[0], "Seek Energy");
        assert!(tron.energy > 0.2); // Energy should have increased
    }

    #[test]
    fn test_memory_consolidation() {
        let mut memory = OrganismMemory::new();
        
        // Add memories directly to short-term to test consolidation
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        for i in 0..50 {
            let importance = if i % 5 == 0 { 0.9 } else if i % 3 == 0 { 0.8 } else { 0.5 };
            let entry = MemoryEntry {
                content: serde_json::json!({"value": i}),
                strength: importance,
                created_at: current_time,
                last_accessed: current_time,
                access_count: if importance > 0.8 { 5 } else { 1 }, // Important memories accessed more
                emotional_weight: 0.5,
            };
            memory.short_term.insert(format!("memory_{}", i), entry);
        }
        
        let initial_short_term = memory.short_term.len();
        let initial_long_term = memory.long_term.len();
        
        memory.consolidate_memories();
        
        // Important memories should have been promoted to long-term
        assert!(memory.long_term.len() > initial_long_term);
        
        // Some short-term memories should have been forgotten
        assert!(memory.short_term.len() <= initial_short_term);
    }
} 