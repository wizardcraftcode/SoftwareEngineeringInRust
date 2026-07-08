// src/pure_enum.rs

#[derive(Debug, PartialEq)]
pub enum Player {
    Human {
        name: String,
        lifepoints: u32,
        score: u32,
        potions: u32,
        has_medic_kit: bool,
    },
    HealerNPC {
        name: String,
        lifepoints: u32,
        score: u32,
        mana_multiplier: f32,
    },
    BerserkerNPC {
        name: String,
        lifepoints: u32,
        score: u32,
        rage_threshold: u32,
    },
}

impl Player {
    pub fn name(&self) -> &str {
        match self {
            Player::Human { name, .. } => name,
            Player::HealerNPC { name, .. } => name,
            Player::BerserkerNPC { name, .. } => name,
        }
    }

    pub fn lifepoints(&self) -> u32 {
        match self {
            Player::Human { lifepoints, .. } => *lifepoints,
            Player::HealerNPC { lifepoints, .. } => *lifepoints,
            Player::BerserkerNPC { lifepoints, .. } => *lifepoints,
        }
    }

    pub fn heal(&mut self) {
        match self {
            Player::Human { lifepoints, potions, has_medic_kit, .. } => {
                if *has_medic_kit {
                    *lifepoints += 80;
                    *has_medic_kit = false;
                } else if *potions > 0 {
                    *lifepoints += 40;
                    *potions -= 1;
                }
            }
            Player::HealerNPC { lifepoints, mana_multiplier, .. } => {
                // Uses an f32 multiplier to calculate healing dynamics dynamically
                let heal_amount = (100.0 * (*mana_multiplier)) as u32;
                *lifepoints += heal_amount;
            }
            Player::BerserkerNPC { lifepoints, rage_threshold, .. } => {
                if *lifepoints < *rage_threshold {
                    *lifepoints += 25;
                }
            }
        }
    }
}

fn heal_all(party: &mut Vec<Player>) {
    for player in party.iter_mut() {
        player.heal();
    }
}

fn main() {
    println!("--- Running Pure Enum Implementation ---");
    let mut party = vec![
        Player::Human { name: "Alice".to_string(), lifepoints: 40, score: 100, potions: 1, has_medic_kit: true },
        Player::HealerNPC { name: "Bob".to_string(), lifepoints: 20, score: 50, mana_multiplier: 1.5 },
    ];

    for player in party.iter_mut() {
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
        let mut party = vec![
            Player::Human { name: "Alice".to_string(), lifepoints: 40, score: 100,
                potions: 1, has_medic_kit: true,
            },
            Player::HealerNPC { name: "Bob".to_string(), lifepoints: 20, score: 50,
                mana_multiplier: 1.5,
            },
        ];

        heal_all(&mut party);

        // Alice should prioritize the medic kit (+80)
        assert_eq!(party[0].lifepoints(), 120);
        // Bob scales by his f32 multiplier (100 * 1.5 = +150)
        assert_eq!(party[1].lifepoints(), 170);
    }


}