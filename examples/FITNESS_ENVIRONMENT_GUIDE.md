# 🧬 Fitness Functions & Environment Guide

This guide explains how to specify custom fitness functions and environments in the Genesis Protocol.

## 📋 Overview

In the Genesis Protocol, **fitness functions** determine how well an organism performs in a given environment, while **environments** define the conditions that organisms must adapt to. Together, they drive the evolution of digital organisms.

## 🎯 Key Concepts

### Fitness Function
A fitness function evaluates how well an organism performs by:
- Analyzing organism traits (health, energy, social skills, etc.)
- Considering environmental conditions
- Returning a fitness score (0.0 to 1.0, where higher is better)

### Environment
An environment defines the conditions that affect organism survival:
- Resource availability
- Competition levels
- Environmental hazards
- Social dynamics
- Innovation opportunities

## 🛠️ How to Create Custom Fitness Functions

### 1. Basic Pattern

```rust
fn my_fitness_function(organism: &TRON, environment: &MyEnvironment) -> f64 {
    let mut fitness = 0.0;
    
    // Add different components
    fitness += organism.health * 0.3;
    fitness += organism.energy * 0.2;
    fitness += organism.social_network.reputation * 0.1;
    
    // Apply environment modifiers
    if environment.is_harsh() {
        fitness *= 0.8; // Penalty in harsh environments
    }
    
    fitness.max(0.0).min(1.0) // Clamp to valid range
}
```

### 2. Environment-Aware Fitness

```rust
fn adaptive_fitness(organism: &TRON, environment: &Environment) -> f64 {
    let mut fitness = 0.0;
    
    // Health is always important
    fitness += organism.health * 0.25;
    
    // Energy matters more when resources are scarce
    let energy_weight = if environment.resource_scarcity > 0.5 {
        0.3 // Higher weight in scarce environments
    } else {
        0.15 // Lower weight in abundant environments
    };
    fitness += organism.energy * energy_weight;
    
    // Social skills matter more in competitive environments
    let social_weight = if environment.competition_level > 0.5 {
        0.25
    } else {
        0.1
    };
    fitness += organism.social_network.reputation * social_weight;
    
    // Adaptation matters more in difficult environments
    let adaptation_weight = if environment.survival_difficulty > 0.5 {
        0.2
    } else {
        0.1
    };
    fitness += organism.dna.metadata.adaptation_score * adaptation_weight;
    
    fitness.max(0.0).min(1.0)
}
```

## 🌍 How to Define Environments

### 1. Simple Environment Structure

```rust
#[derive(Debug, Clone)]
struct MyEnvironment {
    pub resource_scarcity: f64,    // 0.0 = abundant, 1.0 = scarce
    pub competition_level: f64,     // 0.0 = none, 1.0 = intense
    pub hazard_level: f64,          // 0.0 = safe, 1.0 = dangerous
    pub cooperation_bonus: f64,     // 0.0 = none, 1.0 = high
}
```

### 2. Environment Factory Methods

```rust
impl MyEnvironment {
    pub fn easy() -> Self {
        MyEnvironment {
            resource_scarcity: 0.1,
            competition_level: 0.1,
            hazard_level: 0.1,
            cooperation_bonus: 0.2,
        }
    }
    
    pub fn harsh() -> Self {
        MyEnvironment {
            resource_scarcity: 0.9,
            competition_level: 0.9,
            hazard_level: 0.8,
            cooperation_bonus: 0.9, // Cooperation becomes more valuable
        }
    }
    
    pub fn dynamic(cycle: u64) -> Self {
        let factor = (cycle as f64 * 0.1) % 1.0;
        MyEnvironment {
            resource_scarcity: 0.3 + (factor * 0.4),
            competition_level: 0.2 + (factor * 0.6),
            hazard_level: 0.1 + (factor * 0.5),
            cooperation_bonus: 0.5 + (factor * 0.4),
        }
    }
}
```

## 🔧 Integration with Evolution Engine

### 1. Using Custom Fitness in Evolution

```rust
// Create your environment
let environment = MyEnvironment::harsh();

// Create organisms
let mut organisms = vec![TRON::create_new()?, TRON::create_new()?];

// Evaluate fitness for each organism
for organism in &mut organisms {
    let fitness = my_fitness_function(organism, &environment);
    organism.dna.update_fitness(fitness);
}

// Apply selection pressure
let mut evolution_engine = EvolutionEngine::new()?;
evolution_engine.selection_pressure = 0.5;
let eliminated = evolution_engine.apply_selection_pressure(&mut organisms)?;
```

### 2. Dynamic Environment Evolution

```rust
let mut evolution_engine = EvolutionEngine::new()?;
let mut organisms = create_initial_population()?;

for cycle in 1..=10 {
    // Create environment for this cycle
    let environment = MyEnvironment::dynamic(cycle);
    
    // Evaluate fitness in current environment
    for organism in &mut organisms {
        let fitness = my_fitness_function(organism, &environment);
        organism.dna.update_fitness(fitness);
    }
    
    // Evolve population
    evolution_engine.evolve_organism(&mut organisms[0])?;
    evolution_engine.apply_selection_pressure(&mut organisms)?;
}
```

## 📊 Available Organism Traits

When creating fitness functions, you can evaluate these organism traits:

### Basic Traits
- `organism.health` - Health level (0.0-1.0)
- `organism.energy` - Energy level (0.0-1.0)
- `organism.age` - Age in cycles
- `organism.consciousness_level` - Consciousness level (0.0-1.0)

### DNA Traits
- `organism.dna.fitness` - Current DNA fitness
- `organism.dna.generation` - Evolution generation
- `organism.dna.metadata.adaptation_score` - Adaptation ability
- `organism.dna.metadata.neural_complexity` - Neural complexity
- `organism.dna.metadata.reproductive_success` - Reproductive success

### Social Traits
- `organism.social_network.reputation` - Social reputation
- `organism.social_network.family.len()` - Number of family members
- `organism.social_network.friends.len()` - Number of friends

### Neural Traits
- `organism.synapses.len()` - Number of neural connections
- `organism.neural_activity` - Current neural activity level
- `organism.memory.capacity` - Memory capacity

### Behavioral Traits
- `organism.behaviors.len()` - Number of learned behaviors
- `organism.performance.learning_rate` - Learning rate
- `organism.performance.adaptation_speed` - Adaptation speed

## 🎨 Example Use Cases

### 1. Survival-Focused Evolution
```rust
fn survival_fitness(organism: &TRON, env: &Environment) -> f64 {
    let mut fitness = 0.0;
    
    // Health and energy are crucial for survival
    fitness += organism.health * 0.4;
    fitness += organism.energy * 0.3;
    
    // Adaptation helps in changing environments
    fitness += organism.dna.metadata.adaptation_score * 0.2;
    
    // Age penalty (older organisms have lower survival)
    let age_penalty = 1.0 - (organism.age as f64 / 1000.0);
    fitness *= age_penalty;
    
    fitness.max(0.0).min(1.0)
}
```

### 2. Social Cooperation Evolution
```rust
fn cooperation_fitness(organism: &TRON, env: &Environment) -> f64 {
    let mut fitness = 0.0;
    
    // Social reputation is key
    fitness += organism.social_network.reputation * 0.4;
    
    // Number of connections matters
    let connection_score = (organism.synapses.len() as f64 / 100.0).min(1.0);
    fitness += connection_score * 0.3;
    
    // Family and friends
    let social_network_size = organism.social_network.family.len() + 
                             organism.social_network.friends.len();
    let social_score = (social_network_size as f64 / 10.0).min(1.0);
    fitness += social_score * 0.2;
    
    // Cooperation bonus from environment
    fitness *= (1.0 + env.cooperation_bonus);
    
    fitness.max(0.0).min(1.0)
}
```

### 3. Innovation and Learning Evolution
```rust
fn innovation_fitness(organism: &TRON, env: &Environment) -> f64 {
    let mut fitness = 0.0;
    
    // Consciousness and neural complexity
    fitness += organism.consciousness_level * 0.3;
    fitness += organism.dna.metadata.neural_complexity * 0.2;
    
    // Learning ability
    fitness += organism.performance.learning_rate * 0.2;
    
    // Memory capacity
    let memory_score = (organism.memory.capacity as f64 / 1000.0).min(1.0);
    fitness += memory_score * 0.15;
    
    // Number of learned behaviors
    let behavior_score = (organism.behaviors.len() as f64 / 10.0).min(1.0);
    fitness += behavior_score * 0.15;
    
    // Innovation reward from environment
    fitness *= (1.0 + env.innovation_reward);
    
    fitness.max(0.0).min(1.0)
}
```

## 🚀 Running the Examples

### Simple Example
```bash
cd deployment-archive/examples
cargo run --example simple_fitness_example
```

### Advanced Example
```bash
cargo run --example custom_fitness_environment
```

## 💡 Best Practices

1. **Keep fitness functions simple** - Complex functions can be hard to debug
2. **Use environment modifiers** - Make fitness context-dependent
3. **Balance different traits** - Don't over-emphasize any single trait
4. **Test in different environments** - Ensure your function works across conditions
5. **Document your fitness function** - Explain what traits are important and why
6. **Use meaningful weights** - Make sure weights sum to reasonable values
7. **Clamp fitness values** - Always ensure fitness is between 0.0 and 1.0

## 🔍 Debugging Fitness Functions

### Common Issues
- **Fitness always 0.0**: Check if you're accessing organism traits correctly
- **Fitness always 1.0**: Check if your weights are too high
- **No evolution**: Check if fitness differences are too small
- **Unstable evolution**: Check if fitness changes too dramatically

### Debugging Tips
```rust
// Add debug output to your fitness function
fn debug_fitness(organism: &TRON, env: &Environment) -> f64 {
    let health_score = organism.health * 0.3;
    let energy_score = organism.energy * 0.2;
    
    println!("Debug: health={:.3}, energy={:.3}", health_score, energy_score);
    
    health_score + energy_score
}
```

## 📚 Further Reading

- See `simple_fitness_example.rs` for a basic implementation
- See `custom_fitness_environment.rs` for advanced patterns
- See `evolution_simulation.rs` for integration examples
- Check the protocol specification for detailed trait descriptions

---

**Remember**: The key to successful evolution is creating fitness functions that reward the behaviors you want to see emerge in your digital organisms! 🧬✨ 