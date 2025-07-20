use genesis_protocol::*;
use std::time::Instant;

/// Custom fitness evaluator trait implementation
/// This shows how to create your own fitness function
trait CustomFitnessEvaluator {
    fn evaluate_fitness(&self, organism: &TRON, environment: &CustomEnvironment) -> f64;
    fn compare_fitness(&self, a: &TRON, b: &TRON) -> std::cmp::Ordering;
}

/// Custom environment configuration
/// This defines the environmental conditions that affect organism fitness
#[derive(Debug, Clone)]
struct CustomEnvironment {
    /// Temperature factor (0.0 = cold, 1.0 = hot)
    pub temperature: f64,
    /// Resource availability (0.0 = scarce, 1.0 = abundant)
    pub resource_availability: f64,
    /// Competition level (0.0 = none, 1.0 = intense)
    pub competition_level: f64,
    /// Environmental hazards (0.0 = safe, 1.0 = dangerous)
    pub hazard_level: f64,
    /// Social cooperation bonus (0.0 = none, 1.0 = high)
    pub cooperation_bonus: f64,
    /// Innovation reward (0.0 = none, 1.0 = high)
    pub innovation_reward: f64,
}

impl CustomEnvironment {
    pub fn new() -> Self {
        CustomEnvironment {
            temperature: 0.5,
            resource_availability: 0.7,
            competition_level: 0.3,
            hazard_level: 0.2,
            cooperation_bonus: 0.8,
            innovation_reward: 0.6,
        }
    }
    
    /// Create a harsh environment
    pub fn harsh_environment() -> Self {
        CustomEnvironment {
            temperature: 0.8,
            resource_availability: 0.2,
            competition_level: 0.9,
            hazard_level: 0.7,
            cooperation_bonus: 0.9, // Cooperation becomes more important
            innovation_reward: 0.8, // Innovation becomes more valuable
        }
    }
    
    /// Create a favorable environment
    pub fn favorable_environment() -> Self {
        CustomEnvironment {
            temperature: 0.3,
            resource_availability: 0.9,
            competition_level: 0.1,
            hazard_level: 0.1,
            cooperation_bonus: 0.3,
            innovation_reward: 0.4,
        }
    }
    
    /// Create a dynamic environment that changes over time
    pub fn dynamic_environment(cycle: u64) -> Self {
        let cycle_factor = (cycle as f64 * 0.1) % 1.0;
        CustomEnvironment {
            temperature: 0.3 + (cycle_factor * 0.4),
            resource_availability: 0.5 + (cycle_factor * 0.3),
            competition_level: 0.2 + (cycle_factor * 0.6),
            hazard_level: 0.1 + (cycle_factor * 0.5),
            cooperation_bonus: 0.6 + (cycle_factor * 0.3),
            innovation_reward: 0.5 + (cycle_factor * 0.4),
        }
    }
}

/// Example fitness evaluator for survival-focused evolution
struct SurvivalFitnessEvaluator {
    pub survival_weight: f64,
    pub energy_efficiency_weight: f64,
    pub adaptation_weight: f64,
}

impl SurvivalFitnessEvaluator {
    pub fn new() -> Self {
        SurvivalFitnessEvaluator {
            survival_weight: 0.4,
            energy_efficiency_weight: 0.3,
            adaptation_weight: 0.3,
        }
    }
}

impl CustomFitnessEvaluator for SurvivalFitnessEvaluator {
    fn evaluate_fitness(&self, organism: &TRON, environment: &CustomEnvironment) -> f64 {
        let mut fitness = 0.0;
        
        // Survival component - based on health and energy
        let survival_score = (organism.health + organism.energy) / 2.0;
        fitness += survival_score * self.survival_weight;
        
        // Energy efficiency - organisms that maintain energy in harsh conditions
        let energy_efficiency = if environment.resource_availability < 0.5 {
            organism.energy * 1.5 // Bonus for maintaining energy in scarcity
        } else {
            organism.energy
        };
        fitness += energy_efficiency * self.energy_efficiency_weight;
        
        // Adaptation - organisms that can handle environmental changes
        let adaptation_score = organism.dna.metadata.adaptation_score;
        fitness += adaptation_score * self.adaptation_weight;
        
        // Environmental factors
        if environment.hazard_level > 0.5 && organism.health < 0.7 {
            fitness *= 0.8; // Penalty for low health in dangerous environments
        }
        
        if environment.temperature > 0.7 && organism.energy < 0.6 {
            fitness *= 0.9; // Penalty for low energy in hot environments
        }
        
        fitness.max(0.0).min(1.0)
    }
    
    fn compare_fitness(&self, a: &TRON, b: &TRON) -> std::cmp::Ordering {
        // For this example, we'll use the basic DNA fitness
        // In a real implementation, you'd evaluate both organisms in the same environment
        a.dna.fitness.partial_cmp(&b.dna.fitness).unwrap()
    }
}

/// Example fitness evaluator for cooperation-focused evolution
struct CooperationFitnessEvaluator {
    pub cooperation_weight: f64,
    pub social_network_weight: f64,
    pub communication_weight: f64,
}

impl CooperationFitnessEvaluator {
    pub fn new() -> Self {
        CooperationFitnessEvaluator {
            cooperation_weight: 0.5,
            social_network_weight: 0.3,
            communication_weight: 0.2,
        }
    }
}

impl CustomFitnessEvaluator for CooperationFitnessEvaluator {
    fn evaluate_fitness(&self, organism: &TRON, environment: &CustomEnvironment) -> f64 {
        let mut fitness = 0.0;
        
        // Cooperation component - based on social connections
        let social_connections = organism.social_network.family.len() + organism.social_network.friends.len();
        let cooperation_score = (social_connections as f64 / 10.0).min(1.0);
        fitness += cooperation_score * self.cooperation_weight;
        
        // Social network strength - based on reputation and relationships
        let social_network_score = organism.social_network.reputation;
        fitness += social_network_score * self.social_network_weight;
        
        // Communication ability - based on neural connections
        let communication_score = (organism.synapses.len() as f64 / 100.0).min(1.0);
        fitness += communication_score * self.communication_weight;
        
        // Environmental bonus for cooperation
        fitness *= (1.0 + environment.cooperation_bonus);
        
        fitness.max(0.0).min(1.0)
    }
    
    fn compare_fitness(&self, a: &TRON, b: &TRON) -> std::cmp::Ordering {
        a.dna.fitness.partial_cmp(&b.dna.fitness).unwrap()
    }
}

/// Example fitness evaluator for innovation-focused evolution
struct InnovationFitnessEvaluator {
    pub learning_weight: f64,
    pub creativity_weight: f64,
    pub problem_solving_weight: f64,
}

impl InnovationFitnessEvaluator {
    pub fn new() -> Self {
        InnovationFitnessEvaluator {
            learning_weight: 0.4,
            creativity_weight: 0.3,
            problem_solving_weight: 0.3,
        }
    }
}

impl CustomFitnessEvaluator for InnovationFitnessEvaluator {
    fn evaluate_fitness(&self, organism: &TRON, environment: &CustomEnvironment) -> f64 {
        let mut fitness = 0.0;
        
        // Learning ability - based on memory and neural activity
        let learning_score = organism.memory.capacity as f64 / 1000.0;
        fitness += learning_score * self.learning_weight;
        
        // Creativity - based on consciousness level and neural complexity
        let creativity_score = organism.consciousness_level * organism.dna.metadata.neural_complexity;
        fitness += creativity_score * self.creativity_weight;
        
        // Problem solving - based on behaviors and adaptation
        let problem_solving_score = organism.behaviors.len() as f64 / 10.0;
        fitness += problem_solving_score * self.problem_solving_weight;
        
        // Environmental bonus for innovation
        fitness *= (1.0 + environment.innovation_reward);
        
        fitness.max(0.0).min(1.0)
    }
    
    fn compare_fitness(&self, a: &TRON, b: &TRON) -> std::cmp::Ordering {
        a.dna.fitness.partial_cmp(&b.dna.fitness).unwrap()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Genesis Protocol - Custom Fitness & Environment Example");
    println!("==========================================================");
    println!();
    
    // Create different environments
    println!("🌍 Creating Different Environments:");
    let favorable_env = CustomEnvironment::favorable_environment();
    let harsh_env = CustomEnvironment::harsh_environment();
    let dynamic_env = CustomEnvironment::dynamic_environment(5);
    
    println!("   ✅ Favorable Environment: Resources {:.1}%, Hazards {:.1}%", 
             favorable_env.resource_availability * 100.0, favorable_env.hazard_level * 100.0);
    println!("   ✅ Harsh Environment: Resources {:.1}%, Hazards {:.1}%", 
             harsh_env.resource_availability * 100.0, harsh_env.hazard_level * 100.0);
    println!("   ✅ Dynamic Environment: Cycle 5, Temperature {:.1}%", 
             dynamic_env.temperature * 100.0);
    println!();
    
    // Create fitness evaluators
    println!("🎯 Creating Fitness Evaluators:");
    let survival_evaluator = SurvivalFitnessEvaluator::new();
    let cooperation_evaluator = CooperationFitnessEvaluator::new();
    let innovation_evaluator = InnovationFitnessEvaluator::new();
    
    println!("   ✅ Survival Evaluator: Survival {:.1}%, Energy {:.1}%, Adaptation {:.1}%", 
             survival_evaluator.survival_weight * 100.0,
             survival_evaluator.energy_efficiency_weight * 100.0,
             survival_evaluator.adaptation_weight * 100.0);
    println!("   ✅ Cooperation Evaluator: Cooperation {:.1}%, Social {:.1}%, Communication {:.1}%", 
             cooperation_evaluator.cooperation_weight * 100.0,
             cooperation_evaluator.social_network_weight * 100.0,
             cooperation_evaluator.communication_weight * 100.0);
    println!("   ✅ Innovation Evaluator: Learning {:.1}%, Creativity {:.1}%, Problem Solving {:.1}%", 
             innovation_evaluator.learning_weight * 100.0,
             innovation_evaluator.creativity_weight * 100.0,
             innovation_evaluator.problem_solving_weight * 100.0);
    println!();
    
    // Create test organisms
    println!("🧬 Creating Test Organisms:");
    let mut organisms = Vec::new();
    
    for i in 0..5 {
        let mut organism = TRON::create_new()?;
        
        // Create different organism types
        match i {
            0 => {
                // Survival specialist
                organism.health = 0.9;
                organism.energy = 0.8;
                organism.dna.metadata.adaptation_score = 0.7;
                organism.id = format!("survival_specialist_{}", &organism.id[..8]);
            },
            1 => {
                // Social specialist
                organism.social_network.family.push("family_member_1".to_string());
                organism.social_network.friends.push("friend_1".to_string());
                organism.social_network.reputation = 0.8;
                organism.synapses.push(Synapse::new("test_connection".to_string()));
                organism.id = format!("social_specialist_{}", &organism.id[..8]);
            },
            2 => {
                // Innovation specialist
                organism.consciousness_level = 0.8;
                organism.dna.metadata.neural_complexity = 0.7;
                organism.memory.capacity = 800;
                organism.behaviors.push(Behavior {
                    behavior_id: "innovative_behavior".to_string(),
                    name: "Creative Problem Solving".to_string(),
                    trigger: BehaviorTrigger::Custom { condition: "problem_detected".to_string() },
                    action: BehaviorAction::Explore,
                    success_rate: 0.8,
                    learned_at: 0,
                    usage_count: 5,
                    confidence: 0.7,
                });
                organism.id = format!("innovation_specialist_{}", &organism.id[..8]);
            },
            3 => {
                // Balanced organism
                organism.health = 0.7;
                organism.energy = 0.7;
                organism.social_network.reputation = 0.5;
                organism.consciousness_level = 0.5;
                organism.id = format!("balanced_organism_{}", &organism.id[..8]);
            },
            4 => {
                // Weak organism
                organism.health = 0.3;
                organism.energy = 0.2;
                organism.social_network.reputation = 0.1;
                organism.consciousness_level = 0.2;
                organism.id = format!("weak_organism_{}", &organism.id[..8]);
            },
        }
        
        organisms.push(organism);
    }
    
    println!("   ✅ Created {} test organisms with different specializations", organisms.len());
    println!();
    
    // Test fitness evaluation in different environments
    println!("📊 Fitness Evaluation Results:");
    println!("==============================");
    
    for (env_name, environment) in [
        ("Favorable", &favorable_env),
        ("Harsh", &harsh_env),
        ("Dynamic", &dynamic_env),
    ] {
        println!();
        println!("🌍 Environment: {}", env_name);
        println!("   Temperature: {:.1}%, Resources: {:.1}%, Hazards: {:.1}%", 
                 environment.temperature * 100.0,
                 environment.resource_availability * 100.0,
                 environment.hazard_level * 100.0);
        println!();
        
        for organism in &organisms {
            let survival_fitness = survival_evaluator.evaluate_fitness(organism, environment);
            let cooperation_fitness = cooperation_evaluator.evaluate_fitness(organism, environment);
            let innovation_fitness = innovation_evaluator.evaluate_fitness(organism, environment);
            
            println!("   🧬 {}: Survival {:.3}, Cooperation {:.3}, Innovation {:.3}", 
                     &organism.id[..20], survival_fitness, cooperation_fitness, innovation_fitness);
        }
    }
    
    println!();
    
    // Demonstrate environment-driven evolution
    println!("🔄 Environment-Driven Evolution Simulation:");
    println!("===========================================");
    
    let mut evolution_engine = EvolutionEngine::new()?;
    let mut population = organisms.clone();
    
    for cycle in 1..=5 {
        println!();
        println!("🔄 Cycle {}:", cycle);
        
        // Create environment for this cycle
        let cycle_environment = CustomEnvironment::dynamic_environment(cycle);
        println!("   Environment: Resources {:.1}%, Competition {:.1}%, Cooperation Bonus {:.1}%", 
                 cycle_environment.resource_availability * 100.0,
                 cycle_environment.competition_level * 100.0,
                 cycle_environment.cooperation_bonus * 100.0);
        
        // Evaluate fitness for each organism
        let mut fitness_scores = Vec::new();
        for organism in &population {
            let survival_fitness = survival_evaluator.evaluate_fitness(organism, &cycle_environment);
            let cooperation_fitness = cooperation_evaluator.evaluate_fitness(organism, &cycle_environment);
            let innovation_fitness = innovation_evaluator.evaluate_fitness(organism, &cycle_environment);
            
            // Combined fitness score
            let combined_fitness = (survival_fitness + cooperation_fitness + innovation_fitness) / 3.0;
            
            fitness_scores.push((organism.id.clone(), combined_fitness));
            
            // Update organism's DNA fitness
            if let Some(organism_mut) = population.iter_mut().find(|o| o.id == organism.id) {
                organism_mut.dna.update_fitness(combined_fitness);
            }
        }
        
        // Show top performers
        fitness_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        println!("   Top 3 Organisms:");
        for (i, (id, fitness)) in fitness_scores.iter().take(3).enumerate() {
            println!("      {}. {}: Fitness {:.3}", i + 1, &id[..20], fitness);
        }
        
        // Apply selection pressure
        evolution_engine.selection_pressure = 0.3 + (cycle as f64 * 0.1);
        let _eliminated = evolution_engine.apply_selection_pressure(&mut population)?;
        
        println!("   Population after selection: {} organisms", population.len());
    }
    
    println!();
    println!("🌟 Custom Fitness & Environment Example Complete!");
    println!("   Key Achievements:");
    println!("   • 🎯 Created custom fitness evaluators");
    println!("   • 🌍 Defined different environment types");
    println!("   • 🧬 Demonstrated environment-driven evolution");
    println!("   • 📊 Showed fitness variation across environments");
    println!("   • 🔄 Simulated dynamic environmental changes");
    
    Ok(())
} 