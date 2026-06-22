use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

// Global thread-safe counter
static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
#[derive(Debug)]
struct Player {
    name: String,
    id: usize,
    score: u32,
}

impl Player {

    fn new(name: String) -> Self {
        let id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        Self{ name, id, score:0}
    }

    // fn name(&self) -> String {
    //     self.name.to_string()
    // }

    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub(crate) fn score(&self) -> u32 {
        self.score
    }

    fn add_to_score_transformation(self, points: u32) -> Self {
        Self {
            score: self.score + points,
            ..self
        }
    }

    fn add_to_score_mutation(&mut self, points: u32) {
        self.score += points;
    }
}



mod test_player {
    use crate::Player;

    #[test]
    fn create_player() {
        let player = Player::new("Fred".to_string());
        assert_eq!(player.score(), 0);
        assert_eq!(player.name, "Fred".to_string());
    }

    fn add_to_score(player: &mut Player, points: u32) {
        player.score += points;
    }

    #[test]
    fn add_to_score_test_1() {
        let mut player = Player::new("Fred".to_string());
        add_to_score(&mut player, 42);
        assert_eq!(player.score, 42);
    }

    #[test]
    fn add_to_score_2() {
        let mut player = Player::new("Fred".to_string());
        player.add_to_score_mutation(42);
        assert_eq!(player.score, 42);
    }

    #[test]
    fn add_to_score_3() {
        let player = Player::new("Fred".to_string());
        let player = player.add_to_score_transformation(42);
        assert_eq!(player.score, 42);
    }

    #[test]
    fn player_score_increases() {
        let mut player = Player::new("Fred".to_string());
        player.add_to_score_mutation(10);
        player.add_to_score_mutation(5);
        assert_eq!(player.score, 15);
    }
    #[test]
    fn adding_score_twice() {
        let mut player = Player::new("Fred".to_string());
        player = player.add_to_score_transformation(10).add_to_score_transformation(16);
        assert_eq!(player.score, 15);
    }

}



fn main() {
    //This creates a new player with no points
    let mut player = Player::new("Fred".to_string());

    //We'd do something like this to change a players points
    player = player.add_to_score_transformation(42);
    player.add_to_score_mutation(10);


}
