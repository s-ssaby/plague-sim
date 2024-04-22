/// Represents a pathogen that can spontaneously spawn into populations without any infected individuals
/// Spontaneous generation occurs only when the following conditions hold:
/// * At least one healthy individual exists in the population
/// * No infected individuals exist in the population
/// * Random chance allows its creation
pub struct SpontaneousPathogen<T> where T: Pathogen {
    pub spawn_chance: f32,
    pub pathogen: T
}

impl<T> SpontaneousPathogen<T> where T: Pathogen {
    pub fn new(spawn_chance: f32, pathogen: T) -> Self {
        Self {spawn_chance, pathogen}
    }
}

impl<P> Pathogen for SpontaneousPathogen<P> where P: Pathogen {
    fn calculate_population<T>(&self, population: T) -> T where T: PopulationType {
        let prev_population = population.population();
        let new_population;
        // spontaneous generation 
        if prev_population.healthy > 0 && prev_population.infected == 0 && get_random() as f32 <= self.spawn_chance {
            // spawn pathogen into population
            new_population = Population {healthy: prev_population.healthy - 1, infected: 1, dead: prev_population.dead, recovered: prev_population.recovered};
        } else {
            // pathogen acts regularly
            new_population = self.pathogen.calculate_population(prev_population);
        }
        let mut output_population = population;
        output_population.set_population(new_population);
        output_population
    }
}
