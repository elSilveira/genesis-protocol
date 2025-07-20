//! 🌐 Collective Intelligence System
//!
//! This module implements collective intelligence capabilities that enable
//! groups of TRON organisms to exhibit emergent swarm behavior and make
//! collective decisions that surpass individual intelligence.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Collective intelligence coordinator
#[derive(Debug, Clone)]
pub struct CollectiveIntelligence {
    /// Registered organism groups
    pub groups: HashMap<String, OrganismGroup>,
    /// Active collective decisions
    pub active_decisions: Vec<CollectiveDecision>,
    /// Swarm behaviors
    pub swarm_behaviors: Vec<SwarmBehavior>,
    /// Collective memory
    pub collective_memory: CollectiveMemory,
    /// Intelligence metrics
    pub intelligence_metrics: IntelligenceMetrics,
}

/// Group of organisms working together
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganismGroup {
    /// Group identifier
    pub group_id: String,
    /// Group members
    pub members: Vec<String>,
    /// Group purpose/goal
    pub purpose: String,
    /// Group intelligence level
    pub intelligence_level: f64,
    /// Group cohesion strength
    pub cohesion: f64,
    /// Group creation time
    pub created_at: u64,
    /// Group performance history
    pub performance_history: Vec<GroupPerformance>,
}

/// Collective decision making
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveDecision {
    /// Decision ID
    pub decision_id: String,
    /// Decision question/problem
    pub question: String,
    /// Participating organisms
    pub participants: Vec<String>,
    /// Voting options
    pub options: Vec<DecisionOption>,
    /// Decision algorithm used
    pub algorithm: DecisionAlgorithm,
    /// Decision status
    pub status: DecisionStatus,
    /// Final result
    pub result: Option<String>,
    /// Confidence in decision
    pub confidence: f64,
    /// Decision timestamp
    pub timestamp: u64,
}

/// Decision option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionOption {
    /// Option ID
    pub option_id: String,
    /// Option description
    pub description: String,
    /// Votes for this option
    pub votes: Vec<Vote>,
    /// Option weight/score
    pub weight: f64,
}

/// Individual vote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    /// Voting organism
    pub organism_id: String,
    /// Vote strength (0.0-1.0)
    pub strength: f64,
    /// Confidence in vote
    pub confidence: f64,
    /// Vote timestamp
    pub timestamp: u64,
}

/// Decision algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionAlgorithm {
    /// Simple majority voting
    Majority,
    /// Weighted voting by fitness
    WeightedByFitness,
    /// Consensus building
    Consensus,
    /// Swarm intelligence
    SwarmIntelligence,
    /// Artificial neural network
    NeuralNetwork,
}

/// Decision status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecisionStatus {
    /// Decision is being proposed
    Proposed,
    /// Voting is in progress
    Voting,
    /// Decision has been made
    Decided,
    /// Decision was cancelled
    Cancelled,
    /// Decision is being executed
    Executing,
    /// Decision execution complete
    Complete,
}

/// Swarm behavior patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmBehavior {
    /// Behavior ID
    pub behavior_id: String,
    /// Behavior name
    pub name: String,
    /// Behavior type
    pub behavior_type: SwarmBehaviorType,
    /// Participating organisms
    pub participants: Vec<String>,
    /// Behavior parameters
    pub parameters: HashMap<String, f64>,
    /// Behavior effectiveness
    pub effectiveness: f64,
    /// Behavior creation time
    pub created_at: u64,
}

/// Types of swarm behaviors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwarmBehaviorType {
    /// Flocking/herding behavior
    Flocking,
    /// Foraging for resources
    Foraging,
    /// Collective problem solving
    ProblemSolving,
    /// Information sharing
    InformationSharing,
    /// Coordinated evolution
    CoordinatedEvolution,
    /// Defensive behavior
    Defense,
    /// Exploration behavior
    Exploration,
}

/// Collective memory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveMemory {
    /// Shared knowledge base
    pub knowledge_base: HashMap<String, SharedKnowledge>,
    /// Collective experiences
    pub experiences: Vec<CollectiveExperience>,
    /// Wisdom accumulated over time
    pub wisdom: Vec<CollectiveWisdom>,
    /// Memory consolidation rules
    pub consolidation_rules: Vec<ConsolidationRule>,
}

/// Shared knowledge entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedKnowledge {
    /// Knowledge ID
    pub knowledge_id: String,
    /// Knowledge content
    pub content: String,
    /// Contributing organisms
    pub contributors: Vec<String>,
    /// Knowledge reliability
    pub reliability: f64,
    /// Knowledge age
    pub age: u64,
    /// Times knowledge was accessed
    pub access_count: u64,
}

/// Collective experience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveExperience {
    /// Experience ID
    pub experience_id: String,
    /// Experience description
    pub description: String,
    /// Participants
    pub participants: Vec<String>,
    /// Experience outcome
    pub outcome: ExperienceOutcome,
    /// Lessons learned
    pub lessons: Vec<String>,
    /// Experience timestamp
    pub timestamp: u64,
}

/// Experience outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperienceOutcome {
    Success,
    Failure,
    Partial,
    Learning,
}

/// Collective wisdom
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveWisdom {
    /// Wisdom statement
    pub statement: String,
    /// Supporting experiences
    pub supporting_experiences: Vec<String>,
    /// Wisdom confidence
    pub confidence: f64,
    /// Wisdom age
    pub age: u64,
}

/// Memory consolidation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationRule {
    /// Rule name
    pub name: String,
    /// Rule condition
    pub condition: String,
    /// Rule action
    pub action: String,
    /// Rule priority
    pub priority: u32,
}

/// Group performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupPerformance {
    /// Performance measurement time
    pub timestamp: u64,
    /// Task completion rate
    pub task_completion_rate: f64,
    /// Decision accuracy
    pub decision_accuracy: f64,
    /// Coordination efficiency
    pub coordination_efficiency: f64,
    /// Innovation rate
    pub innovation_rate: f64,
}

/// Intelligence metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceMetrics {
    /// Total groups managed
    pub total_groups: usize,
    /// Active decisions
    pub active_decisions: usize,
    /// Successful decisions
    pub successful_decisions: u64,
    /// Failed decisions
    pub failed_decisions: u64,
    /// Average decision time
    pub avg_decision_time: f64,
    /// Collective intelligence score
    pub collective_iq: f64,
}

impl CollectiveIntelligence {
    /// Create new collective intelligence system
    pub fn new() -> Result<Self, CollectiveError> {
        Ok(CollectiveIntelligence {
            groups: HashMap::new(),
            active_decisions: Vec::new(),
            swarm_behaviors: Vec::new(),
            collective_memory: CollectiveMemory::new(),
            intelligence_metrics: IntelligenceMetrics::new(),
        })
    }

    /// Create new organism group
    pub fn create_group(&mut self, members: Vec<String>, purpose: String) -> Result<String, CollectiveError> {
        let group_id = format!("group_{}", uuid::Uuid::new_v4());
        
        let group = OrganismGroup {
            group_id: group_id.clone(),
            members,
            purpose,
            intelligence_level: 0.5,
            cohesion: 0.6,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            performance_history: Vec::new(),
        };
        
        self.groups.insert(group_id.clone(), group);
        self.intelligence_metrics.total_groups += 1;
        
        Ok(group_id)
    }

    /// Initiate collective decision
    pub fn initiate_decision(
        &mut self,
        question: String,
        participants: Vec<String>,
        options: Vec<String>,
        algorithm: DecisionAlgorithm,
    ) -> Result<String, CollectiveError> {
        let decision_id = format!("decision_{}", uuid::Uuid::new_v4());
        
        let decision_options: Vec<DecisionOption> = options
            .into_iter()
            .map(|desc| DecisionOption {
                option_id: uuid::Uuid::new_v4().to_string(),
                description: desc,
                votes: Vec::new(),
                weight: 0.0,
            })
            .collect();
        
        let decision = CollectiveDecision {
            decision_id: decision_id.clone(),
            question,
            participants,
            options: decision_options,
            algorithm,
            status: DecisionStatus::Proposed,
            result: None,
            confidence: 0.0,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };
        
        self.active_decisions.push(decision);
        self.intelligence_metrics.active_decisions += 1;
        
        Ok(decision_id)
    }

    /// Cast vote in collective decision
    pub fn cast_vote(
        &mut self,
        decision_id: &str,
        organism_id: &str,
        option_id: &str,
        strength: f64,
        confidence: f64,
    ) -> Result<(), CollectiveError> {
        if let Some(decision) = self.active_decisions.iter_mut().find(|d| d.decision_id == decision_id) {
            if decision.status != DecisionStatus::Voting {
                return Err(CollectiveError::DecisionNotVoting);
            }
            
            if let Some(option) = decision.options.iter_mut().find(|o| o.option_id == option_id) {
                let vote = Vote {
                    organism_id: organism_id.to_string(),
                    strength,
                    confidence,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                };
                
                option.votes.push(vote);
                option.weight += strength * confidence;
                
                Ok(())
            } else {
                Err(CollectiveError::OptionNotFound(option_id.to_string()))
            }
        } else {
            Err(CollectiveError::DecisionNotFound(decision_id.to_string()))
        }
    }

    /// Finalize collective decision
    pub fn finalize_decision(&mut self, decision_id: &str) -> Result<String, CollectiveError> {
        if let Some(decision) = self.active_decisions.iter_mut().find(|d| d.decision_id == decision_id) {
            match decision.algorithm {
                DecisionAlgorithm::Majority => {
                    let best_option = decision.options.iter()
                        .max_by(|a, b| a.votes.len().cmp(&b.votes.len()));
                    
                    if let Some(option) = best_option {
                        decision.result = Some(option.description.clone());
                        decision.status = DecisionStatus::Decided;
                        decision.confidence = option.votes.len() as f64 / decision.participants.len() as f64;
                        
                        self.intelligence_metrics.successful_decisions += 1;
                        self.intelligence_metrics.active_decisions -= 1;
                        
                        Ok(option.description.clone())
                    } else {
                        Err(CollectiveError::NoVotesCast)
                    }
                },
                DecisionAlgorithm::WeightedByFitness => {
                    let best_option = decision.options.iter()
                        .max_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());
                    
                    if let Some(option) = best_option {
                        decision.result = Some(option.description.clone());
                        decision.status = DecisionStatus::Decided;
                        decision.confidence = option.weight / decision.options.len() as f64;
                        
                        self.intelligence_metrics.successful_decisions += 1;
                        self.intelligence_metrics.active_decisions -= 1;
                        
                        Ok(option.description.clone())
                    } else {
                        Err(CollectiveError::NoVotesCast)
                    }
                },
                _ => {
                    // Other algorithms can be implemented as needed
                    Err(CollectiveError::AlgorithmNotImplemented)
                }
            }
        } else {
            Err(CollectiveError::DecisionNotFound(decision_id.to_string()))
        }
    }

    /// Get collective intelligence metrics
    pub fn get_metrics(&self) -> IntelligenceMetrics {
        self.intelligence_metrics.clone()
    }
}

impl CollectiveMemory {
    pub fn new() -> Self {
        CollectiveMemory {
            knowledge_base: HashMap::new(),
            experiences: Vec::new(),
            wisdom: Vec::new(),
            consolidation_rules: Vec::new(),
        }
    }
}

impl IntelligenceMetrics {
    pub fn new() -> Self {
        IntelligenceMetrics {
            total_groups: 0,
            active_decisions: 0,
            successful_decisions: 0,
            failed_decisions: 0,
            avg_decision_time: 0.0,
            collective_iq: 100.0,
        }
    }
}

/// Collective intelligence errors
#[derive(Debug, thiserror::Error)]
pub enum CollectiveError {
    #[error("Group not found: {0}")]
    GroupNotFound(String),
    #[error("Decision not found: {0}")]
    DecisionNotFound(String),
    #[error("Option not found: {0}")]
    OptionNotFound(String),
    #[error("Decision is not in voting state")]
    DecisionNotVoting,
    #[error("No votes cast")]
    NoVotesCast,
    #[error("Algorithm not implemented")]
    AlgorithmNotImplemented,
    #[error("Insufficient participants")]
    InsufficientParticipants,
    #[error("Group capacity exceeded")]
    GroupCapacityExceeded,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collective_intelligence_creation() {
        let ci = CollectiveIntelligence::new().unwrap();
        assert_eq!(ci.groups.len(), 0);
        assert_eq!(ci.active_decisions.len(), 0);
    }

    #[test]
    fn test_group_creation() {
        let mut ci = CollectiveIntelligence::new().unwrap();
        
        let members = vec!["tron_1".to_string(), "tron_2".to_string()];
        let group_id = ci.create_group(members.clone(), "Test Group".to_string()).unwrap();
        
        assert!(!group_id.is_empty());
        assert!(ci.groups.contains_key(&group_id));
        assert_eq!(ci.groups[&group_id].members, members);
    }

    #[test]
    fn test_decision_initiation() {
        let mut ci = CollectiveIntelligence::new().unwrap();
        
        let participants = vec!["tron_1".to_string(), "tron_2".to_string()];
        let options = vec!["Option A".to_string(), "Option B".to_string()];
        
        let decision_id = ci.initiate_decision(
            "Test Question".to_string(),
            participants,
            options,
            DecisionAlgorithm::Majority,
        ).unwrap();
        
        assert!(!decision_id.is_empty());
        assert_eq!(ci.active_decisions.len(), 1);
        assert_eq!(ci.active_decisions[0].options.len(), 2);
    }

    #[test]
    fn test_voting_and_decision() {
        let mut ci = CollectiveIntelligence::new().unwrap();
        
        let participants = vec!["tron_1".to_string(), "tron_2".to_string()];
        let options = vec!["Option A".to_string(), "Option B".to_string()];
        
        let decision_id = ci.initiate_decision(
            "Test Question".to_string(),
            participants,
            options,
            DecisionAlgorithm::Majority,
        ).unwrap();
        
        // Change status to voting
        ci.active_decisions[0].status = DecisionStatus::Voting;
        let option_id = ci.active_decisions[0].options[0].option_id.clone();
        
        // Cast votes
        ci.cast_vote(&decision_id, "tron_1", &option_id, 1.0, 0.8).unwrap();
        ci.cast_vote(&decision_id, "tron_2", &option_id, 0.8, 0.9).unwrap();
        
        // Finalize decision
        let result = ci.finalize_decision(&decision_id).unwrap();
        assert_eq!(result, "Option A");
        
        assert_eq!(ci.active_decisions[0].status, DecisionStatus::Decided);
        assert!(ci.active_decisions[0].confidence > 0.0);
    }
} 