// src/composition.rs

#[derive(Debug, PartialEq)]
pub struct CommonStats {
    pub name: String,
    pub lifepoints: u32,
    pub score: u32,
}

#[derive(Debug, PartialEq)]
pub enum PlayerKind {
    Human { potions: u32, has_medic_kit: bool },
    HealerNPC { mana_multiplier: f32 },
    BerserkerNPC { rage_threshold: u32 },
}

#[derive(Debug, PartialEq)]
pub struct Player {
    pub stats: CommonStats,
    pub kind: PlayerKind,
}

impl Player {
    pub fn new(name: String, lifepoints: u32, score: u32, kind: PlayerKind) -> Self {
        Self {
            stats: CommonStats { name, lifepoints, score },
            kind,
        }
    }

    pub fn heal(&mut self) {
        match &mut self.kind {
            PlayerKind::Human { potions, has_medic_kit } => {
                if *has_medic_kit {
                    self.stats.lifepoints += 80;
                    *has_medic_kit = false;
                } else if *potions > 0 {
                    self.stats.lifepoints += 40;
                    *potions -= 1;
                }
            }
            PlayerKind::HealerNPC { mana_multiplier } => {
                let heal_amount = (100.0 * (*mana_multiplier)) as u32;
                self.stats.lifepoints += heal_amount;
            }
            PlayerKind::BerserkerNPC { rage_threshold } => {
                if self.stats.lifepoints < *rage_threshold {
                    self.stats.lifepoints += 25;
                }
            }
        }
    }
}

fn main() {
    println!("--- Running Composition Implementation ---");
    let mut party = vec![
        Player::new("Alice".to_string(), 40, 100, PlayerKind::Human { potions: 1, has_medic_kit: true }),
        Player::new("Bob".to_string(), 20, 50, PlayerKind::HealerNPC { mana_multiplier: 1.5 }),
    ];

    for player in party.iter_mut() {
        println!("Before Heal -> Name: {}, HP: {}", player.stats.name, player.stats.lifepoints);
        player.heal();
        println!("After Heal  -> Name: {}, HP: {}\n", player.stats.name, player.stats.lifepoints);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polymorphic_vector_healing() {
        let mut party = vec![
            Player::new("Alice".to_string(), 40, 100,
                        PlayerKind::Human { potions: 1, has_medic_kit: true }),
            Player::new("Bob".to_string(), 20, 50,
                        PlayerKind::HealerNPC { mana_multiplier: 1.5 }),
        ];

        for player in party.iter_mut() {
            player.heal();
        }
        assert_eq!(party[0].stats.lifepoints, 120);
        assert_eq!(party[1].stats.lifepoints, 170);
    }
}