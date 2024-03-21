use std::ops::Add;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Copy, Serialize, Deserialize)]
/** Represents any group of people */
pub struct Population {
    pub healthy: u32,
    pub infected: u32,
    pub dead: u32,
    pub recovered: u32
}

impl Add for Population {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let tot_healthy = self.healthy + rhs.healthy;
        let tot_infected = self.infected + rhs.infected;
        let tot_dead = self.dead + rhs.dead;
        let tot_recovered = self.recovered + rhs.recovered;
        Population { healthy: tot_healthy, infected: tot_infected, dead: tot_dead, recovered: tot_recovered }
    }

}

impl Population {
    /* Creates a population of healthy people */
    pub fn new (initial_pop: u32) -> Self{
        Self {healthy: initial_pop, dead: 0, recovered: 0, infected: 0}
    }

    /// Creates a new population by scaling this population by a scalar factor
    /// 
    /// Note: Scaling will always round down (truncates)
    /// 
    /// Use scale for scaling operations that round to the nearest integer
    pub fn scale_truncate(&self, scalar: f64) -> Population {
        let new_healthy = (scalar*(self.healthy as f64)) as u32;
        let new_dead = (scalar*(self.dead as f64)) as u32;
        let new_recovered = (scalar*(self.recovered as f64)) as u32;
        let new_infected = (scalar*(self.infected as f64)) as u32;
        Self { healthy: new_healthy, infected: new_infected, dead: new_dead, recovered: new_recovered }
    }

    /// Creates a new population by scaling this population by a scalar factor
    /// 
    /// Note: Scaling will always round to the nearest integer
    /// 
    /// Use scale_truncate for scaling operations that always round down (truncation)
    pub fn scale(&self, scalar: f64) -> Population {
        let new_healthy = (scalar*(self.healthy as f64)).round() as u32;
        let new_dead = (scalar*(self.dead as f64)).round() as u32;
        let new_recovered = (scalar*(self.recovered as f64)).round() as u32;
        let new_infected = (scalar*(self.infected as f64)).round() as u32;
        Self { healthy: new_healthy, infected: new_infected, dead: new_dead, recovered: new_recovered }
    }


    /* Returns all non-dead people in population */
    pub fn get_alive(&self) -> u32 {
        self.healthy + self.infected + self.recovered
    }

    /** Returns total population, including dead */
    pub fn get_total(&self) -> u32 {
        self.dead + self.healthy + self.recovered + self.infected
    }

    // Calculates population resulting from removing a group from this population
    // Errors if group cannot be extracted from this population
    pub fn emigrate(&mut self, group: Self) -> Result<Population, String> {
        if group.healthy > self.healthy {
            Err(format!("Cannot remove {} healthy people from {} healthy people", group.healthy, self.healthy))
        } else if group.dead > self.dead {
            Err(format!("Cannot remove {} dead people from {} dead people", group.dead, self.dead))
        } else if group.recovered > self.recovered {
            Err(format!("Cannot remove {} recovered people from {} recovered people", group.recovered, self.recovered))
        } else if group.infected > self.infected {
            Err(format!("Cannot remove {} infected people from {} infected people", group.infected, self.infected))
        } else {
            let new_healthy = self.healthy - group.healthy;
            let new_dead = self.dead - group.dead;
            let new_recovered = self.recovered - group.recovered;
            let new_infected = self.infected - group.infected;
            Ok(Population { healthy: new_healthy, infected: new_infected, dead: new_dead, recovered: new_recovered })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Population;

    #[test]
    fn scale_truncate() {
        let population = Population {healthy: 150, infected: 75, dead: 111, recovered: 2};
        let halved_population = population.scale_truncate(0.5);
        let expected_population = Population {healthy: 75, infected: 37, dead: 55, recovered: 1};
        assert_eq!(halved_population, expected_population);

        let trisected_population = population.scale_truncate(0.333333);
        let expected_population = Population {healthy: 49, infected: 24, dead: 36, recovered: 0};
        assert_eq!(trisected_population, expected_population);
    }

    #[test]
    fn scale() {
        let population = Population {healthy: 150, infected: 75, dead: 111, recovered: 2};
        let halved_population = population.scale(0.5);
        let expected_population = Population {healthy: 75, infected: 38, dead: 56, recovered: 1};
        assert_eq!(halved_population, expected_population);

        let trisected_population = population.scale(0.333333);
        let expected_population = Population {healthy: 50, infected: 25, dead: 37, recovered: 1};
        assert_eq!(trisected_population, expected_population);
    }
}
