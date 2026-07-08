// Imagine this file lives in a completely separate crate or module!
// It imports the central abstractions from your core engine.
use crate::open_party::{Player, PlayerStats};

pub struct Rogue {
    // Composition: Carrying the uniform stats struct mandated by the trait
    pub stats: PlayerStats,
    pub stealth_points: u32,
    pub critical_multiplier: f64,
}

impl Player for Rogue {
    // 1. Fulfill the structural metadata hook for read access
    fn stats(&self) -> &PlayerStats {
        &self.stats
    }

    // 2. Fulfill the structural metadata hook for mutable access
    fn stats_mut(&mut self) -> &mut PlayerStats {
        &mut self.stats
    }

    // 3. Implement the custom behavior unique to the Rogue type
    fn heal(&mut self) {
        // Rogues use their stealth resources to amplify healing
        if self.stealth_points > 10 {
            self.stealth_points -= 10;
            self.stats_mut().lifepoints += 15;
        } else {
            // Standard baseline recovery
            self.stats_mut().lifepoints += 5;
        }
    }
}