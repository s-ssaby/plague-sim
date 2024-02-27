// Represents a disease that can spread from person to person

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pathogen {
    pub name: String,
    // probability of transmission when interacting with another person
    pub infectivity: f32,
    // probability of dying each day
    pub lethality: f32,
    // probability of recovering each day
    pub recovery_rate: f32   
}

impl Pathogen {
    pub fn new(name: String, infectivity: f32, lethality: f32, recovery_rate: f32) -> Result<Self, String> {
        if infectivity < 0.0 || infectivity > 1.0 {
            return Err(format!("Infectivity must be between 0 and 1, not {}", infectivity));
        }
        if lethality < 0.0 || lethality > 1.0 {
            return Err(format!("Lethality must be between 0 and 1, not {}", lethality));
        }
        if recovery_rate < 0.0 || recovery_rate > 1.0 {
            return Err(format!("Recovery rate must be between 0 and 1, not {}", recovery_rate));
        }

        Ok(Self {name, infectivity, lethality, recovery_rate})
    }
}