//! 🧬 Digital DNA System
//!
//! This module implements the Digital DNA system that forms the genetic foundation
//! of all TRON organisms. Digital DNA combines cryptographic security with
//! biological evolution principles.
//!
//! # Features
//!
//! - **Cryptographic Identity**: Each DNA has a unique Ed25519 keypair
//! - **Biological Evolution**: DNA can mutate and crossover naturally
//! - **Fitness Tracking**: DNA fitness evolves based on organism performance
//! - **Generational History**: Track evolution through generations
//! - **Secure Signatures**: Sign and verify messages with DNA identity

use serde::{Deserialize, Serialize};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::{SeedableRng, RngCore};
use sha2::{Sha256, Digest};
use rand::rngs::OsRng;
use std::time::{SystemTime, UNIX_EPOCH};

/// Digital DNA - The genetic foundation of TRON organisms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalDNA {
    /// Cryptographic sequence that defines the organism's identity
    pub sequence: Vec<u8>,
    /// Current generation number (starts at 0)
    pub generation: u64,
    /// List of mutations applied to this DNA
    pub mutations: Vec<Mutation>,
    /// Current fitness level (0.0 to 1.0+)
    pub fitness: f64,
    /// Hash of parent DNA (if created through reproduction)
    pub parent_hash: Option<String>,
    /// Timestamp when DNA was created
    pub created_at: u64,
    /// Cryptographic keypair for identity and signing
    pub keypair: DNAKeypair,
    /// DNA-specific metadata
    pub metadata: DNAMetadata,
}

/// Cryptographic keypair embedded in DNA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNAKeypair {
    /// Ed25519 public key (32 bytes)
    pub public_key: [u8; 32],
    /// Ed25519 secret key (32 bytes) - never serialized in production
    #[serde(skip_serializing)]
    pub secret_key: [u8; 32],
    /// Generation when keys were created/evolved
    pub key_generation: u64,
    /// Key derivation path for hierarchical keys
    pub derivation_path: Vec<u32>,
}

/// DNA metadata for tracking biological properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNAMetadata {
    /// Organism species identifier
    pub species: String,
    /// Biological age in evolution cycles
    pub biological_age: u64,
    /// Mutation rate (0.0 to 1.0)
    pub mutation_rate: f64,
    /// Crossover compatibility with other DNA
    pub crossover_compatibility: f64,
    /// Environmental adaptation score
    pub adaptation_score: f64,
    /// Reproductive success rate
    pub reproductive_success: f64,
    /// Neural complexity level
    pub neural_complexity: f64,
}

/// Types of mutations that can occur in DNA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mutation {
    /// Single byte change in sequence
    PointMutation {
        position: usize,
        old_value: u8,
        new_value: u8,
        timestamp: u64,
    },
    /// Insert new sequence at position
    Insertion {
        position: usize,
        sequence: Vec<u8>,
        timestamp: u64,
    },
    /// Remove sequence from position
    Deletion {
        position: usize,
        length: usize,
        timestamp: u64,
    },
    /// Duplicate sequence within DNA
    Duplication {
        start: usize,
        end: usize,
        insert_at: usize,
        timestamp: u64,
    },
    /// Reverse sequence order
    Inversion {
        start: usize,
        end: usize,
        timestamp: u64,
    },
    /// Move sequence to different position
    Translocation {
        from_start: usize,
        from_end: usize,
        to_position: usize,
        timestamp: u64,
    },
    /// Evolution of cryptographic keys
    KeyEvolution {
        old_generation: u64,
        new_generation: u64,
        timestamp: u64,
    },
}

impl DigitalDNA {
    /// Generate completely new DNA with random cryptographic identity
    pub fn generate_new() -> Result<Self, DNAError> {
        let mut csprng = OsRng{};
        let mut secret_bytes = [0u8; 32];
        csprng.fill_bytes(&mut secret_bytes);
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let verifying_key = signing_key.verifying_key();
        
        // Create biological sequence based on public key and entropy
        let mut hasher = Sha256::new();
        hasher.update(verifying_key.to_bytes());
        hasher.update(&SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_le_bytes());
        hasher.update(&rand::random::<u64>().to_le_bytes());
        
        let sequence = hasher.finalize().to_vec();
        
        // Generate additional entropy for extended sequence
        let mut extended_sequence = sequence.clone();
        for _ in 0..8 {
            let mut hasher = Sha256::new();
            hasher.update(&extended_sequence);
            hasher.update(&rand::random::<u64>().to_le_bytes());
            extended_sequence.extend_from_slice(&hasher.finalize()[..4]);
        }
        
        Ok(DigitalDNA {
            sequence: extended_sequence,
            generation: 0,
            mutations: Vec::new(),
            fitness: 1.0,
            parent_hash: None,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            keypair: DNAKeypair {
                public_key: verifying_key.to_bytes(),
                secret_key: signing_key.to_bytes(),
                key_generation: 0,
                derivation_path: vec![0],
            },
            metadata: DNAMetadata {
                species: "TRON".to_string(),
                biological_age: 0,
                mutation_rate: 0.01, // 1% mutation rate
                crossover_compatibility: 0.8,
                adaptation_score: 0.5,
                reproductive_success: 0.0,
                neural_complexity: 0.1,
            },
        })
    }
    
    /// Create DNA from existing signing key (for testing/reproduction)
    pub fn from_signing_key(signing_key: SigningKey) -> Result<Self, DNAError> {
        let verifying_key = signing_key.verifying_key();
        let mut dna = Self::generate_new()?;
        dna.keypair = DNAKeypair {
            public_key: verifying_key.to_bytes(),
            secret_key: signing_key.to_bytes(),
            key_generation: 0,
            derivation_path: vec![0],
        };
        Ok(dna)
    }
    
    /// Apply a mutation to the DNA
    pub fn mutate(&mut self, mutation: Mutation) -> Result<(), DNAError> {
        match &mutation {
            Mutation::PointMutation { position, new_value, .. } => {
                if *position < self.sequence.len() {
                    self.sequence[*position] = *new_value;
                } else {
                    return Err(DNAError::InvalidMutationPosition(*position));
                }
            },
            Mutation::Insertion { position, sequence, .. } => {
                if *position <= self.sequence.len() {
                    self.sequence.splice(*position..*position, sequence.iter().cloned());
                } else {
                    return Err(DNAError::InvalidMutationPosition(*position));
                }
            },
            Mutation::Deletion { position, length, .. } => {
                if *position < self.sequence.len() && *position + *length <= self.sequence.len() {
                    self.sequence.drain(*position..*position + *length);
                } else {
                    return Err(DNAError::InvalidMutationPosition(*position));
                }
            },
            Mutation::Duplication { start, end, insert_at, .. } => {
                if *start < self.sequence.len() && *end <= self.sequence.len() && *start < *end {
                    let duplicated = self.sequence[*start..*end].to_vec();
                    let insert_pos = (*insert_at).min(self.sequence.len());
                    self.sequence.splice(insert_pos..insert_pos, duplicated);
                } else {
                    return Err(DNAError::InvalidMutationRange(*start, *end));
                }
            },
            Mutation::Inversion { start, end, .. } => {
                if *start < self.sequence.len() && *end <= self.sequence.len() && *start < *end {
                    self.sequence[*start..*end].reverse();
                } else {
                    return Err(DNAError::InvalidMutationRange(*start, *end));
                }
            },
            Mutation::Translocation { from_start, from_end, to_position, .. } => {
                if *from_start < self.sequence.len() && *from_end <= self.sequence.len() && *from_start < *from_end {
                    let translocated = self.sequence.drain(*from_start..*from_end).collect::<Vec<_>>();
                    let insert_pos = (*to_position).min(self.sequence.len());
                    self.sequence.splice(insert_pos..insert_pos, translocated);
                } else {
                    return Err(DNAError::InvalidMutationRange(*from_start, *from_end));
                }
            },
            Mutation::KeyEvolution { new_generation, .. } => {
                self.evolve_keys(*new_generation)?;
            }
        }
        
        self.mutations.push(mutation);
        self.generation += 1;
        self.metadata.biological_age += 1;
        
        // Slight fitness cost for mutations (natural selection pressure)
        self.fitness *= 0.98;
        
        // Update metadata
        self.metadata.mutation_rate = (self.metadata.mutation_rate * 0.9) + (0.1 * 0.01);
        self.metadata.adaptation_score *= 0.95;
        
        Ok(())
    }
    
    /// Generate a random mutation based on current mutation rate
    pub fn generate_random_mutation(&self) -> Mutation {
        let mutation_type = rand::random::<u8>() % 7;
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        match mutation_type {
            0 => Mutation::PointMutation {
                position: rand::random::<usize>() % self.sequence.len(),
                old_value: 0, // Will be filled in when applied
                new_value: rand::random::<u8>(),
                timestamp,
            },
            1 => Mutation::Insertion {
                position: rand::random::<usize>() % (self.sequence.len() + 1),
                sequence: (0..rand::random::<usize>() % 8 + 1)
                    .map(|_| rand::random::<u8>())
                    .collect(),
                timestamp,
            },
            2 => Mutation::Deletion {
                position: rand::random::<usize>() % self.sequence.len(),
                length: rand::random::<usize>() % 4 + 1,
                timestamp,
            },
            3 => {
                let start = rand::random::<usize>() % self.sequence.len();
                let end = start + rand::random::<usize>() % 8 + 1;
                Mutation::Duplication {
                    start,
                    end: end.min(self.sequence.len()),
                    insert_at: rand::random::<usize>() % (self.sequence.len() + 1),
                    timestamp,
                }
            },
            4 => {
                let start = rand::random::<usize>() % self.sequence.len();
                let end = start + rand::random::<usize>() % 8 + 1;
                Mutation::Inversion {
                    start,
                    end: end.min(self.sequence.len()),
                    timestamp,
                }
            },
            5 => {
                let from_start = rand::random::<usize>() % self.sequence.len();
                let from_end = from_start + rand::random::<usize>() % 4 + 1;
                Mutation::Translocation {
                    from_start,
                    from_end: from_end.min(self.sequence.len()),
                    to_position: rand::random::<usize>() % (self.sequence.len() + 1),
                    timestamp,
                }
            },
            _ => Mutation::KeyEvolution {
                old_generation: self.keypair.key_generation,
                new_generation: self.keypair.key_generation + 1,
                timestamp,
            },
        }
    }
    
    /// Perform biological crossover with another DNA
    pub fn crossover(&self, other: &DigitalDNA) -> Result<DigitalDNA, DNAError> {
        // Check compatibility
        if self.metadata.crossover_compatibility < 0.5 || other.metadata.crossover_compatibility < 0.5 {
            return Err(DNAError::CrossoverIncompatible);
        }
        
        // Find crossover points
        let min_len = self.sequence.len().min(other.sequence.len());
        if min_len < 4 {
            return Err(DNAError::SequenceTooShort);
        }
        
        let crossover_point1 = rand::random::<usize>() % (min_len / 2);
        let crossover_point2 = (min_len / 2) + rand::random::<usize>() % (min_len / 2);
        
        // Create new sequence with crossover
        let mut new_sequence = Vec::new();
        new_sequence.extend_from_slice(&self.sequence[..crossover_point1]);
        new_sequence.extend_from_slice(&other.sequence[crossover_point1..crossover_point2]);
        new_sequence.extend_from_slice(&self.sequence[crossover_point2..]);
        
        // Create new DNA from crossover
        let mut child = DigitalDNA::generate_new()?;
        child.sequence = new_sequence;
        child.generation = self.generation.max(other.generation) + 1;
        child.parent_hash = Some(self.get_hash());
        
        // Combine metadata
        child.metadata.species = if rand::random::<bool>() { 
            self.metadata.species.clone() 
        } else { 
            other.metadata.species.clone() 
        };
        child.metadata.mutation_rate = (self.metadata.mutation_rate + other.metadata.mutation_rate) / 2.0;
        child.metadata.crossover_compatibility = (self.metadata.crossover_compatibility + other.metadata.crossover_compatibility) / 2.0;
        child.metadata.adaptation_score = (self.metadata.adaptation_score + other.metadata.adaptation_score) / 2.0;
        child.metadata.neural_complexity = (self.metadata.neural_complexity + other.metadata.neural_complexity) / 2.0;
        
        // Inherit fitness from fitter parent
        child.fitness = self.fitness.max(other.fitness) * 0.95; // Slight regression
        
        Ok(child)
    }
    
    /// Evolve cryptographic keys to next generation
    pub fn evolve_keys(&mut self, new_generation: u64) -> Result<(), DNAError> {
        // Create new keypair with evolved entropy
        let mut seed = [0u8; 32];
        let mut hasher = Sha256::new();
        hasher.update(&self.keypair.secret_key);
        hasher.update(&new_generation.to_le_bytes());
        hasher.update(&self.sequence);
        seed.copy_from_slice(&hasher.finalize());
        
        let mut csprng = rand::rngs::StdRng::from_seed(seed);
        let mut secret_bytes = [0u8; 32];
        csprng.fill_bytes(&mut secret_bytes);
        let new_signing_key = SigningKey::from_bytes(&secret_bytes);
        let new_verifying_key = new_signing_key.verifying_key();
        
        self.keypair = DNAKeypair {
            public_key: new_verifying_key.to_bytes(),
            secret_key: new_signing_key.to_bytes(),
            key_generation: new_generation,
            derivation_path: {
                let mut path = self.keypair.derivation_path.clone();
                path.push(new_generation as u32);
                path
            },
        };
        
        Ok(())
    }
    
    /// Get cryptographic hash of DNA
    pub fn get_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.sequence);
        hasher.update(&self.generation.to_le_bytes());
        hasher.update(&self.keypair.public_key);
        format!("{:x}", hasher.finalize())
    }
    
    /// Sign data with DNA's private key
    pub fn sign_data(&self, data: &[u8]) -> Result<Vec<u8>, DNAError> {
        let signing_key = SigningKey::from_bytes(&self.keypair.secret_key);
        
        Ok(signing_key.sign(data).to_bytes().to_vec())
    }
    
    /// Verify signature with DNA's public key
    pub fn verify_signature(&self, data: &[u8], signature: &[u8]) -> bool {
        if let Ok(verifying_key) = VerifyingKey::from_bytes(&self.keypair.public_key) {
            if signature.len() == 64 {
                let sig_bytes: [u8; 64] = signature.try_into().unwrap();
                let sig = Signature::from_bytes(&sig_bytes);
                return verifying_key.verify(data, &sig).is_ok();
            }
        }
        false
    }
    
    /// Calculate genetic distance from another DNA
    pub fn genetic_distance(&self, other: &DigitalDNA) -> f64 {
        let min_len = self.sequence.len().min(other.sequence.len());
        let max_len = self.sequence.len().max(other.sequence.len());
        
        if max_len == 0 {
            return 0.0;
        }
        
        let mut differences = 0;
        for i in 0..min_len {
            if self.sequence[i] != other.sequence[i] {
                differences += 1;
            }
        }
        
        // Add length difference
        differences += max_len - min_len;
        
        differences as f64 / max_len as f64
    }
    
    /// Update fitness based on performance
    pub fn update_fitness(&mut self, performance_score: f64) {
        // Exponential moving average
        self.fitness = (self.fitness * 0.9) + (performance_score * 0.1);
        
        // Update metadata
        self.metadata.adaptation_score = (self.metadata.adaptation_score * 0.8) + (performance_score * 0.2);
        
        // Clamp fitness to reasonable range
        self.fitness = self.fitness.max(0.0).min(2.0);
    }
    
    /// Get DNA information summary
    pub fn get_info(&self) -> DNAInfo {
        DNAInfo {
            hash: self.get_hash(),
            generation: self.generation,
            sequence_length: self.sequence.len(),
            fitness: self.fitness,
            mutation_count: self.mutations.len(),
            biological_age: self.metadata.biological_age,
            species: self.metadata.species.clone(),
            mutation_rate: self.metadata.mutation_rate,
            adaptation_score: self.metadata.adaptation_score,
            neural_complexity: self.metadata.neural_complexity,
            created_at: self.created_at,
            key_generation: self.keypair.key_generation,
        }
    }
}

/// Summary information about DNA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNAInfo {
    pub hash: String,
    pub generation: u64,
    pub sequence_length: usize,
    pub fitness: f64,
    pub mutation_count: usize,
    pub biological_age: u64,
    pub species: String,
    pub mutation_rate: f64,
    pub adaptation_score: f64,
    pub neural_complexity: f64,
    pub created_at: u64,
    pub key_generation: u64,
}

/// DNA-related errors
#[derive(Debug, thiserror::Error)]
pub enum DNAError {
    #[error("Invalid secret key format")]
    InvalidSecretKey,
    #[error("Invalid public key format")]
    InvalidPublicKey,
    #[error("Invalid mutation position: {0}")]
    InvalidMutationPosition(usize),
    #[error("Invalid mutation range: {0} to {1}")]
    InvalidMutationRange(usize, usize),
    #[error("Crossover incompatible - compatibility too low")]
    CrossoverIncompatible,
    #[error("Sequence too short for crossover")]
    SequenceTooShort,
    #[error("Key evolution failed")]
    KeyEvolutionFailed,
    #[error("Cryptographic operation failed: {0}")]
    CryptographicError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dna_generation() {
        let dna = DigitalDNA::generate_new().unwrap();
        assert!(!dna.sequence.is_empty());
        assert_eq!(dna.generation, 0);
        assert_eq!(dna.fitness, 1.0);
        assert!(dna.mutations.is_empty());
    }

    #[test]
    fn test_dna_hash() {
        let dna = DigitalDNA::generate_new().unwrap();
        let hash1 = dna.get_hash();
        let hash2 = dna.get_hash();
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA256 hex string
    }

    #[test]
    fn test_dna_signing() {
        let dna = DigitalDNA::generate_new().unwrap();
        let message = b"Hello, digital world!";
        
        let signature = dna.sign_data(message).unwrap();
        assert!(dna.verify_signature(message, &signature));
        
        // Test with different message
        let wrong_message = b"Hello, analog world!";
        assert!(!dna.verify_signature(wrong_message, &signature));
    }

    #[test]
    fn test_point_mutation() {
        let mut dna = DigitalDNA::generate_new().unwrap();
        let original_sequence = dna.sequence.clone();
        
        let mutation = Mutation::PointMutation {
            position: 0,
            old_value: original_sequence[0],
            new_value: 255,
            timestamp: 0,
        };
        
        dna.mutate(mutation).unwrap();
        
        assert_eq!(dna.sequence[0], 255);
        assert_eq!(dna.generation, 1);
        assert_eq!(dna.mutations.len(), 1);
        assert!(dna.fitness < 1.0);
    }

    #[test]
    fn test_insertion_mutation() {
        let mut dna = DigitalDNA::generate_new().unwrap();
        let original_len = dna.sequence.len();
        
        let mutation = Mutation::Insertion {
            position: 0,
            sequence: vec![1, 2, 3],
            timestamp: 0,
        };
        
        dna.mutate(mutation).unwrap();
        
        assert_eq!(dna.sequence.len(), original_len + 3);
        assert_eq!(dna.sequence[0], 1);
        assert_eq!(dna.sequence[1], 2);
        assert_eq!(dna.sequence[2], 3);
    }

    #[test]
    fn test_deletion_mutation() {
        let mut dna = DigitalDNA::generate_new().unwrap();
        let original_len = dna.sequence.len();
        
        let mutation = Mutation::Deletion {
            position: 0,
            length: 2,
            timestamp: 0,
        };
        
        dna.mutate(mutation).unwrap();
        
        assert_eq!(dna.sequence.len(), original_len - 2);
    }

    #[test]
    fn test_dna_crossover() {
        let dna1 = DigitalDNA::generate_new().unwrap();
        let dna2 = DigitalDNA::generate_new().unwrap();
        
        let child = dna1.crossover(&dna2).unwrap();
        
        assert_eq!(child.generation, dna1.generation.max(dna2.generation) + 1);
        assert!(child.parent_hash.is_some());
        assert!(!child.sequence.is_empty());
        assert!(child.fitness <= dna1.fitness.max(dna2.fitness));
    }

    #[test]
    fn test_genetic_distance() {
        let dna1 = DigitalDNA::generate_new().unwrap();
        let dna2 = DigitalDNA::generate_new().unwrap();
        
        let distance = dna1.genetic_distance(&dna2);
        assert!(distance >= 0.0 && distance <= 1.0);
        
        // Distance to self should be 0
        let self_distance = dna1.genetic_distance(&dna1);
        assert_eq!(self_distance, 0.0);
    }

    #[test]
    fn test_fitness_update() {
        let mut dna = DigitalDNA::generate_new().unwrap();
        let initial_fitness = dna.fitness;
        
        dna.update_fitness(0.8);
        assert_ne!(dna.fitness, initial_fitness);
        
        dna.update_fitness(1.2);
        assert!(dna.fitness <= 2.0); // Should be clamped
    }

    #[test]
    fn test_key_evolution() {
        let mut dna = DigitalDNA::generate_new().unwrap();
        let _original_generation = dna.keypair.key_generation;
        let original_public_key = dna.keypair.public_key;
        
        dna.evolve_keys(1).unwrap();
        
        assert_eq!(dna.keypair.key_generation, 1);
        assert_ne!(dna.keypair.public_key, original_public_key);
        assert_eq!(dna.keypair.derivation_path.len(), 2);
    }

    #[test]
    fn test_random_mutation_generation() {
        let dna = DigitalDNA::generate_new().unwrap();
        
        // Generate several mutations to test variety
        for _ in 0..10 {
            let mutation = dna.generate_random_mutation();
            // Just verify it doesn't panic and creates valid mutations
            match mutation {
                Mutation::PointMutation { position, .. } => {
                    assert!(position < dna.sequence.len());
                },
                Mutation::Insertion { position, .. } => {
                    assert!(position <= dna.sequence.len());
                },
                Mutation::Deletion { position, length, .. } => {
                    assert!(position < dna.sequence.len());
                    assert!(length > 0);
                },
                _ => {
                    // Other mutations are valid by construction
                }
            }
        }
    }

    #[test]
    fn test_dna_info() {
        let dna = DigitalDNA::generate_new().unwrap();
        let info = dna.get_info();
        
        assert_eq!(info.hash, dna.get_hash());
        assert_eq!(info.generation, dna.generation);
        assert_eq!(info.sequence_length, dna.sequence.len());
        assert_eq!(info.fitness, dna.fitness);
        assert_eq!(info.species, "TRON");
    }
} 