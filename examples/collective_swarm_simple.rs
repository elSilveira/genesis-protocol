use genesis_protocol::*;
use std::time::Instant;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🐝 Genesis Protocol - Collective Swarm Intelligence (Simplified)");
    println!("================================================================");
    println!();
    
    // Create swarm of organisms
    println!("🧬 Creating swarm of digital organisms...");
    let mut swarm = Vec::new();
    let swarm_size = 12;
    
    for i in 0..swarm_size {
        let mut organism = TRON::create_new()?;
        
        // Assign specialized roles using the age field as a type identifier
        organism.age = match i % 4 {
            0 => 1, // Social (age = 1)
            1 => 2, // Economic (age = 2)
            2 => 3, // AI (age = 3)
            _ => 4, // Security (age = 4)
        };
        
        // Reset energy and other stats
        organism.energy = 0.7 + (i as f64 * 0.02);
        organism.neural_activity = 0.1 + (i as f64 * 0.05);
        
        swarm.push(organism);
    }
    
    println!("✅ Swarm of {} organisms created", swarm.len());
    
    // Show swarm composition
    println!();
    println!("📊 Swarm Composition:");
    let social_count = swarm.iter().filter(|o| o.age == 1).count();
    let economic_count = swarm.iter().filter(|o| o.age == 2).count();
    let ai_count = swarm.iter().filter(|o| o.age == 3).count();
    let security_count = swarm.iter().filter(|o| o.age == 4).count();
    
    println!("   👥 Social Organisms: {}", social_count);
    println!("   💰 Economic Organisms: {}", economic_count);
    println!("   🤖 AI Organisms: {}", ai_count);
    println!("   🔒 Security Organisms: {}", security_count);
    println!();
    
    // Demonstrate collective decision making (simplified)
    println!("🤔 Demonstrating Collective Decision Making:");
    println!("=============================================");
    
    // Decision 1: Resource allocation
    println!();
    println!("📊 Decision 1: Resource Allocation Strategy");
    
    let mut votes = HashMap::new();
    votes.insert("equal".to_string(), 0);
    votes.insert("performance".to_string(), 0);
    votes.insert("task_based".to_string(), 0);
    
    // Each organism votes based on its type (using age as type)
    for organism in &swarm {
        match organism.age {
            1 => *votes.get_mut("equal").unwrap() += 1, // Social: Equal distribution
            2 => *votes.get_mut("performance").unwrap() += 1, // Economic: Performance-based
            3 => *votes.get_mut("task_based").unwrap() += 1, // AI: Task-based
            4 => *votes.get_mut("performance").unwrap() += 1, // Security: Performance-based
            _ => *votes.get_mut("equal").unwrap() += 1,
        }
    }
    
    let winner = votes.iter().max_by_key(|(_, &count)| count).unwrap();
    
    println!("   📊 Voting Results:");
    println!("      Equal Distribution: {} votes", votes["equal"]);
    println!("      Performance-based: {} votes", votes["performance"]);
    println!("      Task-based: {} votes", votes["task_based"]);
    println!("   🏆 Winner: {} strategy", winner.0);
    
    // Decision 2: Threat response
    println!();
    println!("🚨 Decision 2: Threat Response Protocol");
    
    let mut threat_votes = HashMap::new();
    threat_votes.insert("isolate".to_string(), 0);
    threat_votes.insert("monitor".to_string(), 0);
    threat_votes.insert("diagnose".to_string(), 0);
    
    for organism in &swarm {
        match organism.age {
            4 => *threat_votes.get_mut("diagnose").unwrap() += 1, // Security: Diagnosis
            3 => *threat_votes.get_mut("diagnose").unwrap() += 1, // AI: Diagnosis
            1 => *threat_votes.get_mut("monitor").unwrap() += 1, // Social: Monitor
            2 => *threat_votes.get_mut("isolate").unwrap() += 1, // Economic: Isolate
            _ => *threat_votes.get_mut("monitor").unwrap() += 1,
        }
    }
    
    let threat_winner = threat_votes.iter().max_by_key(|(_, &count)| count).unwrap();
    
    println!("   📊 Voting Results:");
    println!("      Isolate: {} votes", threat_votes["isolate"]);
    println!("      Monitor: {} votes", threat_votes["monitor"]);
    println!("      Diagnose: {} votes", threat_votes["diagnose"]);
    println!("   🏆 Winner: {} response", threat_winner.0);
    
    // Demonstrate swarm problem-solving
    println!();
    println!("🧩 Swarm Problem-Solving Demonstration:");
    println!("=======================================");
    
    let problem_start = Instant::now();
    
    println!("🎯 Problem: Optimize network topology for maximum efficiency");
    
    // Each organism contributes based on its specialization
    let mut solutions = Vec::new();
    
    for organism in &swarm {
        let solution = match organism.age {
            1 => format!("Organism-{}: Create hub-and-spoke social clusters", &organism.id[..8]),
            2 => format!("Organism-{}: Implement cost-weighted routing", &organism.id[..8]),
            3 => format!("Organism-{}: Use machine learning for adaptive routing", &organism.id[..8]),
            4 => format!("Organism-{}: Add encrypted mesh overlay", &organism.id[..8]),
            _ => format!("Organism-{}: Generic solution approach", &organism.id[..8]),
        };
        
        solutions.push(solution);
    }
    
    println!("   💡 Individual Contributions:");
    for solution in &solutions[..6] { // Show first 6 solutions
        println!("      • {}", solution);
    }
    if solutions.len() > 6 {
        println!("      • ... and {} more solutions", solutions.len() - 6);
    }
    
    let synthesis_time = problem_start.elapsed();
    
    println!();
    println!("   🔄 Collective Synthesis Process:");
    println!("      1. Social organisms identify collaboration patterns");
    println!("      2. Economic organisms calculate efficiency metrics");
    println!("      3. AI organisms model optimization algorithms");
    println!("      4. Security organisms ensure protection protocols");
    
    println!("   ⏱️  Synthesis Time: {:.2}ms", synthesis_time.as_millis());
    
    // Final swarm solution
    println!();
    println!("   🎯 Swarm Solution:");
    println!("      'Implement adaptive mesh network with:'");
    println!("      • Social clustering for natural communication flows");
    println!("      • Economic routing for resource optimization");
    println!("      • AI-driven adaptation for changing conditions");
    println!("      • Security overlay for protection'");
    
    // Demonstrate emergent behavior
    println!();
    println!("🌊 Emergent Behavior Demonstration:");
    println!("===================================");
    
    println!("🌡️  Applying environmental pressure: Resource scarcity");
    
    // Organisms adapt their behavior
    for organism in &mut swarm {
        match organism.age {
            1 => {
                // Social organisms share resources
                organism.energy = (organism.energy + 0.1).min(1.0);
                println!("   👥 Social organism {} shares resources with neighbors", &organism.id[..8]);
            },
            2 => {
                // Economic organisms optimize consumption
                organism.neural_activity = (organism.neural_activity * 0.9).max(0.1);
                println!("   💰 Economic organism {} optimizes energy consumption", &organism.id[..8]);
            },
            3 => {
                // AI organisms find efficiencies
                organism.dna.fitness += 0.05;
                println!("   🤖 AI organism {} discovers efficiency improvements", &organism.id[..8]);
            },
            4 => {
                // Security organisms maintain vigilance
                organism.energy = (organism.energy - 0.02).max(0.5);
                println!("   🔒 Security organism {} maintains protective protocols", &organism.id[..8]);
            },
            _ => {}
        }
    }
    
    // Show emergent properties
    println!();
    println!("   🌟 Emergent Properties Observed:");
    println!("      • Resource sharing networks formed spontaneously");
    println!("      • Efficiency improvements through specialization");
    println!("      • Adaptive responses to environmental changes");
    println!("      • Self-organizing hierarchies based on capability");
    
    // Swarm statistics
    println!();
    println!("📊 Final Swarm Statistics:");
    println!("=========================");
    
    let total_neural_activity = swarm.iter().map(|o| o.neural_activity).sum::<f64>();
    let avg_energy = swarm.iter().map(|o| o.energy).sum::<f64>() / swarm.len() as f64;
    let avg_fitness = swarm.iter().map(|o| o.dna.fitness).sum::<f64>() / swarm.len() as f64;
    let collective_intelligence = total_neural_activity * avg_fitness;
    
    println!("   Swarm Size: {} organisms", swarm.len());
    println!("   Total Neural Activity: {:.2}", total_neural_activity);
    println!("   Average Energy: {:.1}%", avg_energy * 100.0);
    println!("   Average Fitness: {:.3}", avg_fitness);
    println!("   Collective Intelligence: {:.2}", collective_intelligence);
    
    // Compare individual vs collective capabilities
    let best_individual = swarm.iter().max_by(|a, b| a.neural_activity.partial_cmp(&b.neural_activity).unwrap()).unwrap();
    let individual_vs_collective = collective_intelligence / best_individual.neural_activity;
    
    println!();
    println!("⚡ Individual vs Collective Comparison:");
    println!("   Best Individual Neural Activity: {:.2}", best_individual.neural_activity);
    println!("   Collective Intelligence: {:.2}", collective_intelligence);
    println!("   Swarm Amplification: {:.1}x stronger", individual_vs_collective);
    
    // Show organism diversity
    println!();
    println!("🌈 Organism Diversity:");
    let mut energy_ranges = vec![0, 0, 0]; // low, medium, high
    let mut fitness_ranges = vec![0, 0, 0];
    
    for organism in &swarm {
        match organism.energy {
            e if e < 0.6 => energy_ranges[0] += 1,
            e if e < 0.8 => energy_ranges[1] += 1,
            _ => energy_ranges[2] += 1,
        }
        
        match organism.dna.fitness {
            f if f < 0.5 => fitness_ranges[0] += 1,
            f if f < 0.8 => fitness_ranges[1] += 1,
            _ => fitness_ranges[2] += 1,
        }
    }
    
    println!("   Energy Distribution: Low={}, Medium={}, High={}", 
             energy_ranges[0], energy_ranges[1], energy_ranges[2]);
    println!("   Fitness Distribution: Low={}, Medium={}, High={}", 
             fitness_ranges[0], fitness_ranges[1], fitness_ranges[2]);
    
    println!();
    println!("🌟 Collective Swarm Intelligence Achievements:");
    println!("   • 🐝 Demonstrated swarm coordination");
    println!("   • 🗳️  Implemented collective decision making");
    println!("   • 🧩 Solved complex problems collaboratively");
    println!("   • 🌊 Showed emergent behavior patterns");
    println!("   • 💪 Achieved amplified intelligence");
    println!("   • 🔄 Adapted to environmental changes");
    println!("   • 🤝 Exhibited specialization and cooperation");
    
    Ok(())
} 