use crate::open_party::{HealerNPC, Human, Player, PlayerStats};
use crate::rogue_plugin::Rogue;

mod open_party;
mod rogue_plugin;

fn main() {
    println!("--- Running Boxed Trait Object Implementation ---");

    // The collection size is uniform because it stores only pointers (vtable + heap address)
    let mut party: Vec<Box<dyn Player>> = vec![
        Box::new(Human {
            stats: PlayerStats {
                name: "Alice".to_string(),
                lifepoints: 40,
                score: 100,
            },
            potions: 1,
            has_medic_kit: true,
        }),
        Box::new(HealerNPC {
            stats: PlayerStats {
                name: "Bob".to_string(),
                lifepoints: 20,
                score: 50,
            },
            mana_multiplier: 1.5,
        }),
    ];

    // Add a rogue to the party even though the rogue plugin is in a different module
    party.push(Box::new(Rogue {
        stats: PlayerStats {
            name: "Shadow".to_string(),
            lifepoints: 35,
            score: 120,
        },
        stealth_points: 50,
        critical_multiplier: 2.0,
    }));

    for player in party.iter_mut() {
        // Dynamic dispatch happens behind the scenes here
        println!(
            "Before Heal -> Name: {}, HP: {}",
            player.name(),
            player.lifepoints()
        );
        player.heal();
        println!(
            "After Heal  -> Name: {}, HP: {}\n",
            player.name(),
            player.lifepoints()
        );
    }
}
