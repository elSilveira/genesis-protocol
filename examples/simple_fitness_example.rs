use genesis_protocol::*;

/// Simple example showing how to create custom fitness functions
/// This demonstrates the basic pattern for fitness evaluation

// 1. Define your environment
#[derive(Debug, Clone)]
struct SimpleEnvironment {
    pub resource_scarcity: f64,    // 0.0 = abundant, 1.0 = scarce
    pub competition_level: f64,     // 0.0 = none, 1.0 = intense
    pub survival_difficulty: f64,   // 0.0 = easy, 1.0 = hard
}

impl SimpleEnvironment {
    pub fn easy() -> Self {
        SimpleEnvironment {
            resource_scarcity: 0.1,
            competition_level: 0.1,
            survival_difficulty: 0.1,
        }
    }
    
    pub fn hard() -> Self {
        SimpleEnvironment {
            resource_scarcity: 0.9,
            competition_level: 0.9,
            survival_difficulty: 0.9,
        }
    }
    
    pub fn moderate() -> Self {
        SimpleEnvironment {
            resource_scarcity: 0.5,
            competition_level: 0.5,
            survival_difficulty: 0.5,
        }
    }
}

// 2. Define your fitness function
fn calculate_fitness(organism: &TRON, environment: &SimpleEnvironment) -> f64 {
    let mut fitness = 0.0;
    
    // Base fitness from organism's DNA
    fitness += organism.dna.fitness * 0.3;
    
    // Health component
    fitness += organism.health * 0.2;
    
    // Energy component (more important in resource-scarce environments)
    let energy_bonus = if environment.resource_scarcity > 0.5 {
        organism.energy * 1.5  // Energy is more valuable when resources are scarce
    } else {
        organism.energy
    };
    fitness += energy_bonus * 0.2;
    
    // Social component (more important in competitive environments)
    let social_bonus = if environment.competition_level > 0.5 {
        organism.social_network.reputation * 1.3  // Social skills matter more in competition
    } else {
        organism.social_network.reputation
    };
    fitness += social_bonus * 0.15;
    
    // Adaptation component (more important in difficult environments)
    let adaptation_bonus = if environment.survival_difficulty > 0.5 {
        organism.dna.metadata.adaptation_score * 1.4  // Adaptation matters more when survival is hard
    } else {
        organism.dna.metadata.adaptation_score
    };
    fitness += adaptation_bonus * 0.15;
    
    // Clamp fitness to 0.0-1.0 range
    fitness.max(0.0).min(1.0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Simple Fitness Function Example");
    println!("==================================");
    println!();
    
    // Create test organisms
    println!("🧬 Creating test organisms...");
    let mut organisms = Vec::new();
    
    for i in 0..3 {
        let mut organism = TRON::create_new()?;
        
        match i {
            0 => {
                // Strong organism
                organism.health = 0.9;
                organism.energy = 0.8;
                organism.social_network.reputation = 0.7;
                organism.dna.metadata.adaptation_score = 0.6;
                organism.id = format!("strong_{}", &organism.id[..8]);
            },
            1 => {
                // Average organism
                organism.health = 0.5;
                organism.energy = 0.5;
                organism.social_network.reputation = 0.5;
                organism.dna.metadata.adaptation_score = 0.5;
                organism.id = format!("average_{}", &organism.id[..8]);
            },
            2 => {
                // Weak organism
                organism.health = 0.2;
                organism.energy = 0.3;
                organism.social_network.reputation = 0.2;
                organism.dna.metadata.adaptation_score = 0.3;
                organism.id = format!("weak_{}", &organism.id[..8]);
            },
        }
        
        organisms.push(organism);
    }
    
    println!("✅ Created {} test organisms", organisms.len());
    println!();
    
    // Test fitness in different environments
    println!("📊 Fitness Evaluation Results:");
    println!("==============================");
    
    let environments = [
        ("Easy", SimpleEnvironment::easy()),
        ("Moderate", SimpleEnvironment::moderate()),
        ("Hard", SimpleEnvironment::hard()),
    ];
    
    for (env_name, environment) in environments.iter() {
        println!();
        println!("🌍 Environment: {}", env_name);
        println!("   Resources: {:.1}% scarce", environment.resource_scarcity * 100.0);
        println!("   Competition: {:.1}% intense", environment.competition_level * 100.0);
        println!("   Difficulty: {:.1}% hard", environment.survival_difficulty * 100.0);
        println!();
        
        for organism in &organisms {
            let fitness = calculate_fitness(organism, environment);
            println!("   🧬 {}: Fitness = {:.3}", &organism.id[..15], fitness);
        }
    }
    
    println!();
    println!("💡 Key Concepts Demonstrated:");
    println!("   • Environment affects fitness calculation");
    println!("   • Different traits matter more in different conditions");
    println!("   • Fitness is dynamic and context-dependent");
    println!("   • You can customize the fitness function for your needs");
    
    Ok(())
} 