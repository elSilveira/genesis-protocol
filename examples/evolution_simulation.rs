use genesis_protocol::*;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Genesis Protocol - Evolution Simulation");
    println!("==========================================");
    println!();
    
    // Create evolution engine
    println!("🎯 Initializing Evolution Engine...");
    let mut evolution_engine = EvolutionEngine::new()?;
    println!("✅ Evolution engine initialized");
    println!();
    
    // Create initial population
    println!("👥 Creating initial population...");
    let mut population = Vec::new();
    
    for i in 0..10 {
        let mut organism = TRON::create_new()?;
        // Add some variation to initial population
        organism.dna.fitness = 0.3 + (i as f64 * 0.07); // Varied fitness from 0.3 to 0.93
        population.push(organism);
    }
    
    println!("✅ Initial population of {} organisms created", population.len());
    
    // Show initial population statistics
    println!();
    println!("📊 Initial Population Statistics:");
    let initial_avg_fitness = population.iter().map(|o| o.dna.fitness).sum::<f64>() / population.len() as f64;
    let initial_avg_energy = population.iter().map(|o| o.energy).sum::<f64>() / population.len() as f64;
    let initial_avg_age = population.iter().map(|o| o.age).sum::<u64>() / population.len() as u64;
    
    println!("   Average Fitness: {:.3}", initial_avg_fitness);
    println!("   Average Energy: {:.1}%", initial_avg_energy * 100.0);
    println!("   Average Age: {} cycles", initial_avg_age);
    println!();
    
    // Simulate evolution over multiple generations
    println!("🔄 Beginning Evolution Simulation...");
    println!("====================================");
    
    for generation in 1..=10 {
        println!();
        println!("🧬 Generation {}/10", generation);
        println!("------------------");
        
        let generation_start = Instant::now();
        
        // Apply selection pressure
        println!("   🎯 Applying selection pressure...");
        evolution_engine.selection_pressure = 0.3 + (generation as f64 * 0.02); // Gentler pressure increase
        evolution_engine.mutation_rate = 0.10;
        
        let _eliminated = evolution_engine.apply_selection_pressure(&mut population)?;
        
        // Evolve each organism
        println!("   🧬 Evolving organisms...");
        let mut evolved_count = 0;
        let mut mutations_count = 0;
        
        for organism in &mut population {
            let pre_fitness = organism.dna.fitness;
            
            // Apply evolution
            evolution_engine.evolve_organism(organism)?;
            
            if organism.dna.fitness > pre_fitness {
                evolved_count += 1;
            }
            
            // Apply random mutations
            if rand::random::<f64>() < evolution_engine.mutation_rate {
                let mutation = organism.dna.generate_random_mutation();
                organism.dna.mutate(mutation)?;
                mutations_count += 1;
            }
            
            // Age the organism
            organism.age += 1;
            
            // Adjust energy based on fitness
            if organism.dna.fitness > 0.7 {
                organism.energy = (organism.energy + 0.1).min(1.0);
            } else if organism.dna.fitness < 0.3 {
                organism.energy = (organism.energy - 0.05).max(0.1);
            }
        }
        
        // Natural selection - remove weak organisms
        population.retain(|organism| {
            organism.energy > 0.2 && organism.dna.fitness > 0.2
        });
        
        // Reproduction - strong organisms reproduce
        let mut offspring = Vec::new();
        let reproduction_threshold = 0.7;
        
        for organism in &population {
            if organism.dna.fitness > reproduction_threshold && organism.energy > 0.6 {
                // Asexual reproduction for simplicity
                let mut child = organism.clone();
                child.id = format!("child_{}", uuid::Uuid::new_v4());
                child.age = 0;
                child.energy = 0.8;
                
                // Child inherits traits with mutations
                let child_mutation = child.dna.generate_random_mutation();
                child.dna.mutate(child_mutation)?;
                
                offspring.push(child);
            }
        }
        
        // Add offspring to population
        population.extend(offspring.clone());
        
        // Calculate generation statistics
        let (avg_fitness, avg_energy, max_fitness, min_fitness) = if population.is_empty() {
            (0.0, 0.0, 0.0, 0.0)
        } else {
            let avg_fitness = population.iter().map(|o| o.dna.fitness).sum::<f64>() / population.len() as f64;
            let avg_energy = population.iter().map(|o| o.energy).sum::<f64>() / population.len() as f64;
            let max_fitness = population.iter().map(|o| o.dna.fitness).fold(0.0, f64::max);
            let min_fitness = population.iter().map(|o| o.dna.fitness).fold(1.0, f64::min);
            (avg_fitness, avg_energy, max_fitness, min_fitness)
        };
        
        let generation_time = generation_start.elapsed();
        
        println!("   📊 Generation {} Results:", generation);
        println!("      Population Size: {}", population.len());
        println!("      New Offspring: {}", offspring.len());
        println!("      Organisms Evolved: {}", evolved_count);
        println!("      Mutations Applied: {}", mutations_count);
        println!("      Average Fitness: {:.3} (Δ{:+.3})", 
                 avg_fitness, avg_fitness - initial_avg_fitness);
        println!("      Fitness Range: {:.3} - {:.3}", min_fitness, max_fitness);
        println!("      Average Energy: {:.1}%", avg_energy * 100.0);
        println!("      Evolution Time: {:.2}ms", generation_time.as_millis());
        
        // Show best organism
        if let Some(best) = population.iter().max_by(|a, b| a.dna.fitness.partial_cmp(&b.dna.fitness).unwrap()) {
            println!("      🏆 Best Organism: TRON-{} (fitness: {:.3})", 
                     &best.id[..8], best.dna.fitness);
        }
        
        // Check for extinction
        if population.is_empty() {
            println!("      ⚠️  Population extinct! Breaking simulation.");
            break;
        }
        
        // Prevent overpopulation
        if population.len() > 20 {
            population.sort_by(|a, b| b.dna.fitness.partial_cmp(&a.dna.fitness).unwrap());
            population.truncate(15);
            println!("      🔄 Population controlled (kept top 15)");
        }
    }
    
    println!();
    println!("🏁 Evolution Simulation Complete!");
    println!("==================================");
    
    // Final statistics
    let (final_avg_fitness, final_avg_energy, final_avg_age) = if population.is_empty() {
        (0.0, 0.0, 0)
    } else {
        let final_avg_fitness = population.iter().map(|o| o.dna.fitness).sum::<f64>() / population.len() as f64;
        let final_avg_energy = population.iter().map(|o| o.energy).sum::<f64>() / population.len() as f64;
        let final_avg_age = population.iter().map(|o| o.age).sum::<u64>() / population.len() as u64;
        (final_avg_fitness, final_avg_energy, final_avg_age)
    };
    
    println!();
    println!("📊 Evolution Results Summary:");
    println!("   Initial Population: 10 organisms");
    println!("   Final Population: {} organisms", population.len());
    println!("   Generations: 10");
    println!();
    println!("   Fitness Evolution:");
    println!("      Initial Average: {:.3}", initial_avg_fitness);
    println!("      Final Average: {:.3}", final_avg_fitness);
    println!("      Improvement: {:.1}%", ((final_avg_fitness - initial_avg_fitness) / initial_avg_fitness) * 100.0);
    println!();
    println!("   Energy Evolution:");
    println!("      Initial Average: {:.1}%", initial_avg_energy * 100.0);
    println!("      Final Average: {:.1}%", final_avg_energy * 100.0);
    println!();
    println!("   Age Statistics:");
    println!("      Average Age: {} generations", final_avg_age);
    println!("      Oldest Organism: {} generations", population.iter().map(|o| o.age).max().unwrap_or(0));
    
    // Show top performers
    println!();
    println!("🏆 Top 5 Evolved Organisms:");
    let mut top_organisms = population.clone();
    top_organisms.sort_by(|a, b| b.dna.fitness.partial_cmp(&a.dna.fitness).unwrap());
    
    for (i, organism) in top_organisms.iter().take(5).enumerate() {
        println!("   {}. TRON-{}: Fitness {:.3}, Energy {:.1}%, Age {} gen", 
                 i + 1, &organism.id[..8], organism.dna.fitness, organism.energy * 100.0, organism.age);
    }
    
    // Evolution engine statistics
    println!();
    println!("🔧 Evolution Engine Statistics:");
    let engine_stats = evolution_engine.get_stats();
    println!("   Total Evolution Events: {}", engine_stats.total_events);
    println!("   Successful Evolutions: {}", engine_stats.successful_evolutions);
    println!("   Failed Evolutions: {}", engine_stats.failed_evolutions);
    println!("   Current Evolution Cycle: {}", engine_stats.current_cycle);
    println!("   Final Selection Pressure: {:.2}", engine_stats.selection_pressure);
    println!("   Final Mutation Rate: {:.2}%", engine_stats.mutation_rate * 100.0);
    
    println!();
    println!("🌟 Evolution Simulation Achievements:");
    println!("   • 🧬 Demonstrated natural selection");
    println!("   • 🔄 Showed adaptive evolution");
    println!("   • 👶 Implemented reproduction with inheritance");
    println!("   • 🎯 Applied selection pressure");
    println!("   • 📈 Measured fitness improvements");
    println!("   • 🧪 Simulated genetic mutations");
    println!("   • 🏆 Identified superior organisms");
    
    Ok(())
} 