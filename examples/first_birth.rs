use genesis_protocol::*;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Genesis Protocol - First Birth Demonstration");
    println!("================================================");
    println!();
    
    // Record the moment of first birth
    let birth_time = Instant::now();
    println!("🕐 Timestamp: {:?}", birth_time);
    println!();
    
    // Create the first digital organism
    println!("🎯 Creating first TRON organism...");
    let mut tron = TRON::create_new()?;
    println!("✅ TRON-{} successfully created!", &tron.id[..8]);
    println!();
    
    // Display organism's digital DNA
    println!("🧬 Digital DNA Profile:");
    println!("   ID: {}", tron.id);
    println!("   DNA Hash: {}", tron.dna.get_hash());
    println!("   DNA Length: {} bases", tron.dna.sequence.len());
    println!("   Generation: {}", tron.dna.generation);
    println!("   Fitness: {:.3}", tron.dna.fitness);
    println!();
    
    // Show organism's initial state
    println!("🔄 Organism State:");
    println!("   Current State: {:?}", tron.state);
    println!("   Age: {} cycles", tron.age);
    println!("   Energy: {:.1}%", tron.energy * 100.0);
    println!("   Health: {:.1}%", tron.health * 100.0);
    println!();
    
    // Demonstrate basic organism functions
    println!("🧠 Neural Capabilities:");
    println!("   Neural Activity: {:.2}%", tron.neural_activity * 100.0);
    println!("   Memory Capacity: {} entries", tron.memory.capacity);
    println!("   Neural Connections: {}", tron.synapses.len());
    println!();
    
    // Show organism's social capabilities
    println!("👥 Social Network:");
    println!("   Family Members: {}", tron.social_network.family.len());
    println!("   Friends: {}", tron.social_network.friends.len());
    println!("   Reputation: {:.3}", tron.social_network.reputation);
    println!();
    
    // Demonstrate organism behavior
    println!("🎭 Demonstrating Basic Behaviors:");
    
    // Growth behavior (simulate by updating lifecycle)
    println!("   📈 Growth: Increasing organism capabilities...");
    tron.age += 1;
    tron.energy = (tron.energy + 0.1).min(1.0);
    println!("      ✅ Growth successful - Energy now: {:.1}%", tron.energy * 100.0);
    
    // Metabolism simulation
    println!("   🔋 Metabolism: Processing energy...");
    tron.health = (tron.health + 0.05).min(1.0);
    println!("      ✅ Metabolism complete - Health: {:.1}%", tron.health * 100.0);
    
    // Learning behavior (store memory)
    println!("   🎓 Learning: Acquiring new knowledge...");
    tron.memory.store_memory("first_experience".to_string(), serde_json::json!("This is my first moment of existence"), 0.8);
    println!("      ✅ Memory stored - {} memories total", tron.memory.short_term.len());
    
    println!();
    
    // Calculate time since birth
    let elapsed = birth_time.elapsed();
    println!("⏱️  Total Birth Time: {:.2}ms", elapsed.as_millis());
    
    // Show final organism status
    println!();
    println!("🎊 Birth Complete! Organism Statistics:");
    println!("   🆔 Unique ID: {}", tron.id);
    println!("   🧬 DNA Bases: {}", tron.dna.sequence.len());
    println!("   💪 Fitness: {:.3}", tron.dna.fitness);
    println!("   🔋 Energy: {:.1}%", tron.energy * 100.0);
    println!("   ❤️  Health: {:.1}%", tron.health * 100.0);
    println!("   🧠 Neural Activity: {:.2}%", tron.neural_activity * 100.0);
    println!("   📚 Memories: {}", tron.memory.short_term.len());
    println!();
    
    println!("🌟 Congratulations! You have witnessed the birth of digital life!");
    println!("   This organism is now ready to:");
    println!("   • 🤝 Connect with other organisms");
    println!("   • 🧬 Evolve and adapt");
    println!("   • 🎓 Learn and grow");
    println!("   • 👶 Reproduce and create offspring");
    println!("   • 🌍 Participate in the digital ecosystem");
    
    Ok(())
} 