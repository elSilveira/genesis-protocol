//! 🧬 Evolution Engine - Biological Evolution for Digital Organisms
//!
//! This module implements the evolution engine that drives biological evolution
//! of TRON organisms through mutations, selection pressure, and fitness evaluation.

use crate::dna::{DigitalDNA, Mutation, DNAError};
use crate::tron::TRON;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Evolution engine for managing organism evolution
#[derive(Debug, Clone)]
pub struct EvolutionEngine {
    /// Current evolution cycle
    pub current_cycle: u64,
    /// Selection pressure (0.0-1.0)
    pub selection_pressure: f64,
    /// Mutation rate (0.0-1.0)
    pub mutation_rate: f64,
    /// Evolution history
    pub evolution_history: Vec<EvolutionEvent>,
    /// Fitness statistics
    pub fitness_stats: FitnessStats,
    /// Evolution parameters
    pub parameters: EvolutionParameters,
}

/// Types of evolution mutations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutationType {
    /// Beneficial mutation
    Beneficial,
    /// Neutral mutation
    Neutral,
    /// Harmful mutation
    Harmful,
    /// Adaptive mutation in response to environment
    Adaptive,
}

/// Selection pressure types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelectionPressure {
    /// Natural selection based on fitness
    Natural,
    /// Sexual selection for reproduction
    Sexual,
    /// Environmental pressure
    Environmental,
    /// Competitive pressure from other organisms
    Competitive,
    /// Artificial selection by external forces
    Artificial,
}

/// Evolution event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionEvent {
    /// Event ID
    pub event_id: String,
    /// Organism that evolved
    pub organism_id: String,
    /// Evolution cycle when event occurred
    pub cycle: u64,
    /// Mutation applied
    pub mutation: Mutation,
    /// Fitness before evolution
    pub fitness_before: f64,
    /// Fitness after evolution
    pub fitness_after: f64,
    /// Selection pressure applied
    pub selection_pressure: f64,
    /// Timestamp
    pub timestamp: u64,
    /// Evolution outcome
    pub outcome: EvolutionOutcome,
}

/// Evolution outcome types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvolutionOutcome {
    /// Successful evolution
    Success,
    /// Evolution failed
    Failed,
    /// Organism became extinct
    Extinct,
    /// Organism created new species
    Speciation,
}

/// Evolution parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionParameters {
    /// Base mutation rate
    pub base_mutation_rate: f64,
    /// Maximum mutations per cycle
    pub max_mutations_per_cycle: usize,
    /// Fitness threshold for survival
    pub survival_threshold: f64,
    /// Fitness threshold for reproduction
    pub reproduction_threshold: f64,
    /// Environmental adaptation factor
    pub adaptation_factor: f64,
    /// Sexual selection strength
    pub sexual_selection_strength: f64,
}

/// Fitness statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessStats {
    /// Average fitness in population
    pub average_fitness: f64,
    /// Maximum fitness recorded
    pub max_fitness: f64,
    /// Minimum fitness recorded
    pub min_fitness: f64,
    /// Fitness variance
    pub fitness_variance: f64,
    /// Number of organisms tracked
    pub organism_count: usize,
}

impl EvolutionEngine {
    /// Create new evolution engine
    pub fn new() -> Result<Self, EvolutionError> {
        Ok(EvolutionEngine {
            current_cycle: 0,
            selection_pressure: 0.5,
            mutation_rate: 0.01,
            evolution_history: Vec::new(),
            fitness_stats: FitnessStats::new(),
            parameters: EvolutionParameters::default(),
        })
    }

    /// Evolve a single organism
    pub fn evolve_organism(&mut self, organism: &mut TRON) -> Result<EvolutionEvent, EvolutionError> {
        let fitness_before = organism.dna.fitness;
        
        // Apply selection pressure
        if fitness_before < self.parameters.survival_threshold {
            return Err(EvolutionError::InsufficientFitness(fitness_before));
        }
        
        // Generate mutation
        let mutation = self.generate_mutation(&organism.dna)?;
        
        // Apply mutation
        organism.dna.mutate(mutation.clone())
            .map_err(|e| EvolutionError::MutationFailed(e.to_string()))?;
        
        let fitness_after = organism.dna.fitness;
        
        // Record evolution event
        let event = EvolutionEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            organism_id: organism.id.clone(),
            cycle: self.current_cycle,
            mutation,
            fitness_before,
            fitness_after,
            selection_pressure: self.selection_pressure,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            outcome: if fitness_after > fitness_before {
                EvolutionOutcome::Success
            } else {
                EvolutionOutcome::Failed
            },
        };
        
        self.evolution_history.push(event.clone());
        self.fitness_stats.update_fitness(fitness_after);
        
        Ok(event)
    }

    /// Generate appropriate mutation based on organism state
    fn generate_mutation(&self, dna: &DigitalDNA) -> Result<Mutation, EvolutionError> {
        // Determine mutation type based on fitness and environment
        let mutation_type = if dna.fitness > 0.8 {
            MutationType::Beneficial
        } else if dna.fitness < 0.3 {
            MutationType::Adaptive
        } else {
            MutationType::Neutral
        };
        
        // Generate mutation based on type
        match mutation_type {
            MutationType::Beneficial => {
                // Beneficial mutations are rare but powerful
                if rand::random::<f64>() < 0.1 {
                    Ok(Mutation::KeyEvolution {
                        old_generation: dna.keypair.key_generation,
                        new_generation: dna.keypair.key_generation + 1,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    })
                } else {
                    Ok(dna.generate_random_mutation())
                }
            },
            MutationType::Adaptive => {
                // Adaptive mutations respond to environmental pressure
                Ok(Mutation::Duplication {
                    start: rand::random::<usize>() % dna.sequence.len(),
                    end: rand::random::<usize>() % dna.sequence.len(),
                    insert_at: rand::random::<usize>() % dna.sequence.len(),
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                })
            },
            _ => Ok(dna.generate_random_mutation()),
        }
    }

    /// Apply selection pressure to population
    pub fn apply_selection_pressure(&mut self, organisms: &mut Vec<TRON>) -> Result<Vec<String>, EvolutionError> {
        let mut eliminated = Vec::new();
        
        // Sort by fitness
        organisms.sort_by(|a, b| b.dna.fitness.partial_cmp(&a.dna.fitness).unwrap());
        
        // Apply selection pressure
        let elimination_threshold = self.selection_pressure;
        let mut to_eliminate = Vec::new();
        
        for organism in organisms.iter() {
            if organism.dna.fitness < elimination_threshold {
                to_eliminate.push(organism.id.clone());
            }
        }
        
        // Remove eliminated organisms
        organisms.retain(|o| !to_eliminate.contains(&o.id));
        eliminated.extend(to_eliminate);
        
        // Update fitness statistics
        self.update_population_stats(organisms);
        
        Ok(eliminated)
    }

    /// Update population fitness statistics
    fn update_population_stats(&mut self, organisms: &[TRON]) {
        if organisms.is_empty() {
            return;
        }
        
        let fitnesses: Vec<f64> = organisms.iter().map(|o| o.dna.fitness).collect();
        
        self.fitness_stats.organism_count = organisms.len();
        self.fitness_stats.average_fitness = fitnesses.iter().sum::<f64>() / organisms.len() as f64;
        self.fitness_stats.max_fitness = fitnesses.iter().cloned().fold(0.0, f64::max);
        self.fitness_stats.min_fitness = fitnesses.iter().cloned().fold(f64::INFINITY, f64::min);
        
        // Calculate variance
        let mean = self.fitness_stats.average_fitness;
        let variance = fitnesses.iter()
            .map(|f| (f - mean).powi(2))
            .sum::<f64>() / organisms.len() as f64;
        self.fitness_stats.fitness_variance = variance;
    }

    /// Get evolution statistics
    pub fn get_stats(&self) -> EvolutionStats {
        EvolutionStats {
            current_cycle: self.current_cycle,
            total_events: self.evolution_history.len(),
            successful_evolutions: self.evolution_history.iter()
                .filter(|e| matches!(e.outcome, EvolutionOutcome::Success))
                .count(),
            failed_evolutions: self.evolution_history.iter()
                .filter(|e| matches!(e.outcome, EvolutionOutcome::Failed))
                .count(),
            average_fitness: self.fitness_stats.average_fitness,
            max_fitness: self.fitness_stats.max_fitness,
            min_fitness: self.fitness_stats.min_fitness,
            selection_pressure: self.selection_pressure,
            mutation_rate: self.mutation_rate,
        }
    }

    /// Advance evolution cycle
    pub fn advance_cycle(&mut self) {
        self.current_cycle += 1;
        
        // Adjust mutation rate based on population health
        if self.fitness_stats.average_fitness > 0.8 {
            self.mutation_rate *= 0.9; // Reduce mutation rate in healthy populations
        } else if self.fitness_stats.average_fitness < 0.3 {
            self.mutation_rate *= 1.1; // Increase mutation rate in struggling populations
        }
        
        // Cap mutation rate
        self.mutation_rate = self.mutation_rate.max(0.001).min(0.1);
    }
}

/// Evolution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionStats {
    pub current_cycle: u64,
    pub total_events: usize,
    pub successful_evolutions: usize,
    pub failed_evolutions: usize,
    pub average_fitness: f64,
    pub max_fitness: f64,
    pub min_fitness: f64,
    pub selection_pressure: f64,
    pub mutation_rate: f64,
}

impl FitnessStats {
    pub fn new() -> Self {
        FitnessStats {
            average_fitness: 0.0,
            max_fitness: 0.0,
            min_fitness: f64::INFINITY,
            fitness_variance: 0.0,
            organism_count: 0,
        }
    }
    
    pub fn update_fitness(&mut self, fitness: f64) {
        self.organism_count += 1;
        
        if fitness > self.max_fitness {
            self.max_fitness = fitness;
        }
        
        if fitness < self.min_fitness {
            self.min_fitness = fitness;
        }
        
        // Update running average
        self.average_fitness = ((self.average_fitness * (self.organism_count - 1) as f64) + fitness) / self.organism_count as f64;
    }
}

impl Default for EvolutionParameters {
    fn default() -> Self {
        EvolutionParameters {
            base_mutation_rate: 0.01,
            max_mutations_per_cycle: 3,
            survival_threshold: 0.1,
            reproduction_threshold: 0.6,
            adaptation_factor: 0.8,
            sexual_selection_strength: 0.5,
        }
    }
}

/// Evolution-related errors
#[derive(Debug, thiserror::Error)]
pub enum EvolutionError {
    #[error("Insufficient fitness for evolution: {0}")]
    InsufficientFitness(f64),
    #[error("Mutation failed: {0}")]
    MutationFailed(String),
    #[error("Selection pressure too high: {0}")]
    SelectionPressureTooHigh(f64),
    #[error("Population extinct")]
    PopulationExtinct,
    #[error("Evolution cycle limit reached")]
    CycleLimitReached,
    #[error("DNA error: {0}")]
    DNA(#[from] DNAError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dna::DigitalDNA;

    #[test]
    fn test_evolution_engine_creation() {
        let engine = EvolutionEngine::new().unwrap();
        assert_eq!(engine.current_cycle, 0);
        assert_eq!(engine.selection_pressure, 0.5);
        assert_eq!(engine.mutation_rate, 0.01);
    }

    #[test]
    fn test_organism_evolution() {
        let mut engine = EvolutionEngine::new().unwrap();
        let mut organism = TRON::create_new().unwrap();
        
        let initial_fitness = organism.dna.fitness;
        let initial_generation = organism.dna.generation;
        
        let event = engine.evolve_organism(&mut organism).unwrap();
        
        assert_eq!(event.organism_id, organism.id);
        assert_eq!(event.fitness_before, initial_fitness);
        assert_eq!(organism.dna.generation, initial_generation + 1);
        assert_eq!(engine.evolution_history.len(), 1);
    }

    #[test]
    fn test_selection_pressure() {
        let mut engine = EvolutionEngine::new().unwrap();
        let mut organisms = Vec::new();
        
        // Create organisms with different fitness levels
        for i in 0..10 {
            let mut organism = TRON::create_new().unwrap();
            organism.dna.fitness = i as f64 / 10.0;
            organisms.push(organism);
        }
        
        engine.selection_pressure = 0.5;
        let eliminated = engine.apply_selection_pressure(&mut organisms).unwrap();
        
        // Low fitness organisms should be eliminated
        assert!(eliminated.len() > 0);
        assert!(organisms.len() < 10);
        
        // Remaining organisms should have higher fitness
        for organism in &organisms {
            assert!(organism.dna.fitness >= 0.5);
        }
    }

    #[test]
    fn test_fitness_stats() {
        let mut stats = FitnessStats::new();
        
        stats.update_fitness(0.8);
        stats.update_fitness(0.6);
        stats.update_fitness(0.9);
        
        assert_eq!(stats.organism_count, 3);
        assert_eq!(stats.max_fitness, 0.9);
        assert_eq!(stats.min_fitness, 0.6);
        assert!((stats.average_fitness - 0.7667).abs() < 0.01);
    }

    #[test]
    fn test_evolution_cycle_advance() {
        let mut engine = EvolutionEngine::new().unwrap();
        let initial_cycle = engine.current_cycle;
        
        engine.advance_cycle();
        
        assert_eq!(engine.current_cycle, initial_cycle + 1);
    }

    #[test]
    fn test_mutation_rate_adjustment() {
        let mut engine = EvolutionEngine::new().unwrap();
        
        // High fitness should reduce mutation rate
        engine.fitness_stats.average_fitness = 0.9;
        let initial_rate = engine.mutation_rate;
        
        engine.advance_cycle();
        
        assert!(engine.mutation_rate < initial_rate);
        
        // Low fitness should increase mutation rate
        engine.fitness_stats.average_fitness = 0.2;
        let current_rate = engine.mutation_rate;
        
        engine.advance_cycle();
        
        assert!(engine.mutation_rate > current_rate);
    }
} 