// Represents a disease that can spread from person to person

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pathogen {
    pub name: String,
    // probability of transmission when interacting with another person
    pub infectivity: f64,
    // probability of dying each day
    pub lethality: f64,
    // probability of recovering each day
    pub recovery_rate: f64   
}

impl Pathogen {
    pub fn new(name: String, infectivity: f64, lethality: f64, recovery_rate: f64) -> Result<Self, String> {
        if infectivity < 0.0 || infectivity > 1.0 {
            return Err(format!("Infectivity must be between 0 and 1, not {}", infectivity));
        }
        if lethality < 0.0 || lethality > 1.0 {
            return Err(format!("Lethality must be between 0 and 1, not {}", lethality));
        }
        if recovery_rate < 0.0 || recovery_rate > 1.0 {
            return Err(format!("Recovery rate must be between 0 and 1, not {}", recovery_rate));
        }
        let sum = recovery_rate + infectivity;
        if sum > 1.0 {
            return Err(format!("Sum of recovery rate and lethality rate cannot exceed 1, sum is {}", sum));
        }

        Ok(Self {name, infectivity, lethality, recovery_rate})
    }
}