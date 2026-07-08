// Observations
//
// Memory Layout Visual:  In this open_party code, your vector is simply an array of
// uniform 16-byte Fat Pointers (8 bytes for the heap data address, 8 bytes for the vtable metadata
// address).
//
// The Extensibility Paradox:
// If you want to add a Rogue tomorrow, you just type struct Rogue; impl Player for Rogue. But if
// you want to add a new operation—like pub fn save_to_database()—you have to break the Player
// trait definition and update every single file and struct in your entire codebase to fulfill the
// new compiler requirements.

// src/open_party

// The Polymorphic Contract (Open Polymorphism)
pub trait Player {
    // This is the only metadata hook concrete types MUST implement
    fn stats(&self) -> &PlayerStats;

    // This is a mutable hook if you want traits to modify stats directly
    fn stats_mut(&mut self) -> &mut PlayerStats;

    fn heal(&mut self);

    // 2. The Blanket Implementations (Shared behavior!)
    fn name(&self) -> &str {
        &self.stats().name
    }

    fn lifepoints(&self) -> u32 {
        self.stats().lifepoints
    }
}

#[derive(Debug, PartialEq)]
pub struct PlayerStats {
    pub name: String,
    pub lifepoints: u32,
    pub score: u32,
}

// Variant 1: Concrete Struct for Human
#[derive(Debug, PartialEq)]
pub struct Human {
    pub stats: PlayerStats, // Composition from Video 6
    pub potions: u32,
    pub has_medic_kit: bool,
}

impl Player for Human {
    // We only implement the structural hooks!
    fn stats(&self) -> &PlayerStats { &self.stats }
    fn stats_mut(&mut self) -> &mut PlayerStats { &mut self.stats }

    fn heal(&mut self) {
        if self.potions > 0 {
            self.potions -= 1;
            self.stats_mut().lifepoints += 20;
        } else if self.has_medic_kit {
            self.stats_mut().lifepoints += 50;
        }
    }
}

// Variant 2: Concrete Struct for HealerNPC
#[derive(Debug, PartialEq)]
pub struct HealerNPC {
    pub stats: PlayerStats, // The shared parts from Video 6!
    pub mana_multiplier: f64,
}

impl Player for HealerNPC {
    fn stats(&self) -> &PlayerStats { &self.stats }
    fn stats_mut(&mut self) -> &mut PlayerStats { &mut self.stats }
    fn heal(&mut self) {
        let heal_amount = (100.0 * self.mana_multiplier) as u32;
        self.stats.lifepoints += heal_amount;
    }

}

// Variant 3: Concrete Struct for BerserkerNPC
// Added seamlessly without modifying a central enum definition.
#[derive(Debug, PartialEq)]
pub struct BerserkerNPC {
    pub stats: PlayerStats, // The shared parts from Video 6!
    pub rage_threshold: u32,
}

impl Player for BerserkerNPC {
    fn stats(&self) -> &PlayerStats { &self.stats }
    fn stats_mut(&mut self) -> &mut PlayerStats { &mut self.stats }
    fn heal(&mut self) {
        if self.stats.lifepoints < self.rage_threshold {
            self.stats.lifepoints += 25;
        }
    }
}

fn main() {
    println!("--- Running Boxed Trait Object Implementation ---");

    // The collection size is uniform because it stores only pointers (vtable + heap address)
    let mut party: Vec<Box<dyn Player>> = vec![
        Box::new(Human { stats:PlayerStats{name: "Alice".to_string(), lifepoints: 40, score: 100}, potions: 1, has_medic_kit: true }),
        Box::new(HealerNPC {stats:PlayerStats{ name: "Bob".to_string(), lifepoints: 20, score: 50}, mana_multiplier: 1.5 }),
    ];

    for player in party.iter_mut() {
        // Dynamic dispatch happens behind the scenes here
        println!("Before Heal -> Name: {}, HP: {}", player.name(), player.lifepoints());
        player.heal();
        println!("After Heal  -> Name: {}, HP: {}\n", player.name(), player.lifepoints());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polymorphic_vector_healing() {
        let mut party: Vec<Box<dyn Player>> = vec![
            Box::new(Human {
                stats:PlayerStats{name: "Alice".to_string(),
                lifepoints: 40,
                score: 100},
                potions: 1,
                has_medic_kit: true,
            }),
            Box::new(HealerNPC {
                stats:PlayerStats{name: "Bob".to_string(),
                lifepoints: 20,
                score: 50},
                mana_multiplier: 1.5,
            }),
        ];

        for player in party.iter_mut() {
            player.heal();
        }

        assert_eq!(party[0].lifepoints(), 60);
        assert_eq!(party[1].lifepoints(), 170);
    }
}