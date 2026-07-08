#[derive(Debug, PartialEq)]
pub struct CommonStats {
    pub name: String,
    pub lifepoints: u32,
    pub score: u32,
}

#[derive(Debug, PartialEq)]
pub struct Human {
    pub stats: CommonStats,
    potions: u32,
    has_medic_kit: bool,
}
impl PlayerBehavior for Human{
    fn heal(&mut self){
        if self.has_medic_kit {
            self.stats.lifepoints += 80;
            self.has_medic_kit = false;
        } else if self.potions > 0 {
            self.stats.lifepoints += 40;
            self.potions -= 1;
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HealerNPC {
    pub stats: CommonStats,
    mana_multiplier: f32,
}
impl PlayerBehavior for HealerNPC{
    fn heal(&mut self){
        let heal_amount = (100.0 * (self.mana_multiplier)) as u32;
        self.stats.lifepoints += heal_amount;
    }
}

#[derive(Debug, PartialEq)]
pub enum Players {
    Human(Human),
    HealerNPC(HealerNPC),
}

impl PlayerBehavior for Players{
    fn heal(&mut self){
        match self{
            Players::Human(Human) => self.heal(),
            Players::HealerNPC(HealerNPC) => self.heal(),
        }
    }
}
pub trait PlayerBehavior{
    fn heal(&mut self);
}

fn heal_player_static(whom: &mut impl PlayerBehavior){
    whom.heal();
}

fn heal_player_dyn(whom: &mut dyn PlayerBehavior){
    whom.heal();
}

fn main() {
    println!("Hello, world!");
}
