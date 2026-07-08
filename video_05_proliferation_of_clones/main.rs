#![allow(unused)]
use crate::GamePlay::{compute_winner, Player, WinningOutcome, WinningOutcome1};
use prost_types::Timestamp;

mod GamePlay {

    #[derive(Debug, Clone, PartialEq)]
    struct GameTime{
        year: u32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Snapshot{
        game_time: GameTime,
        score: u32,
        achievements: Vec<u32>,
    }

    // don't implement copy or clone
    // that would make us be able to have multiple objects
    // for the same player
    #[derive(Debug,Clone, PartialEq)]
    pub struct Player {
        pub name: String,
        score: u32,
        achievements: Vec<u32>,
        snapshots: Vec<Snapshot>
    }

    #[derive(Debug, PartialEq)]
    pub enum WinningOutcome1 {
        Winner,
        Loser,
        Draw,
    }
    impl Player {

            fn boost(&mut self, points: u32) {
                self.score += points;
                todo!();
            }
            fn reset_score(&mut self) {
                todo!();
                todo!();
            }
            fn damage(&mut self, life_points: i32) {

                self.score -= life_points as u32;
                todo!();
            }
            fn promote(&mut self) {
                self.score *= 2;
                todo!();
            }
            fn heal(&mut self, life_points: i32) {
                self.score += life_points as u32;
                todo!();
            }


        pub fn compute_winner(&self, p0: &Player) -> WinningOutcome1 {
            if self.score > p0.score{
                WinningOutcome1::Winner
            } else if self.score < p0.score{
                WinningOutcome1::Loser
            } else {
                WinningOutcome1::Draw
            }
        }
    }


    // This one is the free function version
    #[derive(Debug, PartialEq)]
    pub enum WinningOutcome {
        FirstPlayerWins,
        SecondPlayerWins,
        Draw,
    }
    pub fn compute_winner(a: &Player, b: &Player) -> WinningOutcome {
        if a.score > b.score {
            WinningOutcome::FirstPlayerWins
        } else if b.score > a.score {
            WinningOutcome::SecondPlayerWins
        } else {
            WinningOutcome::Draw
        }
    }


    // from video 2 but updated for adding achievements to Player
    impl Player {



        pub fn score(&self) -> u32 {
            self.score
        }

        fn add_to_score_transformation(self, points: u32) -> Self {
            Self {
                score: self.score + points,
                ..self
            }
        }

        pub fn add_to_score_mutation(&mut self, points: u32) {
            self.score += points;
        }
    }

    // from video 3
    impl Player{
        // private and only used for testing
        fn loaded_for_test(name: &str) -> Self {
            Player { name: name.to_owned(), score: 0, achievements: vec!(42, 64), snapshots:vec![]}
        }

        pub fn unlock_achievement(&mut self, p0: u32) {
            self.achievements.push(p0);
        }

        pub fn achievements(&self) -> Vec<u32> {
            self.achievements.clone()
        }

        pub fn achievement_count(&self) -> usize {
            self.achievements.len()
        }

        pub fn has_unlocked_achievement(&self, achievement_id: u32) -> bool {
            self.achievements.contains(&achievement_id)
        }
    }

    // from video 5
    impl Player {
        // pub fn new(name: String) -> Self {
        //     Self{ name, score:0, achievements: vec![], snapshots:vec![]}
        // }

        pub fn new(name: &str) -> Self {
            Self{ name:name.to_owned(), score:0, achievements: vec![], snapshots:vec![] }
        }
        pub fn take_snapshot_borrowed(&mut self, game_time: &GameTime) {
            self.snapshots.push(
                Snapshot{game_time: game_time.clone(), score: self.score,
                    achievements: self.achievements.clone()})
        }

        pub fn take_snapshot_owned(&mut self, game_time: GameTime) {
            self.snapshots.push(
                Snapshot{game_time: game_time, score: self.score,
                    achievements: self.achievements.clone()})
        }


    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn score_starts_at_zero() {
            let player = Player::new("Player");
            assert_eq!(player.score, 0);
        }


        //*********************************************************************************
        // From video 2
        //*********************************************************************************
        fn add_to_score(player: &mut Player, points: u32) {
            player.score += points;
        }

        #[test]
        fn add_to_score_test_1() {
            let mut player = Player::new("Fred");
            add_to_score(&mut player, 42);
            assert_eq!(player.score, 42);
        }

        #[test]
        fn add_to_score_2() {
            let mut player = Player::new("Fred");
            player.add_to_score_mutation(42);
            assert_eq!(player.score, 42);
        }

        #[test]
        fn add_to_score_3() {
            let player = Player::new("Fred");
            let player = player.add_to_score_transformation(42);
            assert_eq!(player.score, 42);
        }

        #[test]
        fn player_score_increases() {
            let mut player = Player::new("Fred");
            player.add_to_score_mutation(10);
            player.add_to_score_mutation(5);
            assert_eq!(player.score, 15);
        }
        #[test]
        fn adding_score_twice() {
            let mut player = Player::new("Fred");
            player = player.add_to_score_transformation(10).add_to_score_transformation(16);
            assert_eq!(player.score, 15);
        }

        //*********************************************************************************
        // From video 3
        //*********************************************************************************
        #[test]
        fn add_achievement_broken() {
            let mut player = Player::loaded_for_test("Player");

            let achievements_before = player.achievements();
            player.unlock_achievement(48);
            let achievements_after = player.achievements();
            assert_eq!(achievements_before.len()+1, achievements_after.len());
            assert!(achievements_after.contains(&48));
        }

        #[test]
        fn add_achievement_awkward_scopes() {
            let mut player = Player::loaded_for_test("Player");

            let achievements_len_before;
            {
                // We isolate the borrow to this temporary block
                let achievements_before = player.achievements();
                achievements_len_before = achievements_before.len();
            } // The shared borrow of 'player' is forced to end here!

            player.unlock_achievement(48);

            let achievements_after = player.achievements();
            assert_eq!(achievements_len_before + 1, achievements_after.len());
            assert!(achievements_after.contains(&48));
        }

        #[test]
        fn add_achievement_awkward_drop() {
            let mut player = Player::loaded_for_test("Player");

            let achievements_before = player.achievements();
            let temp_len = achievements_before.len();
            drop(achievements_before); // Force-kill the borrow!

            player.unlock_achievement(48);
            let achievements_after = player.achievements();

            assert_eq!(temp_len+1, achievements_after.len());
            assert!(achievements_after.contains(&48));
        }

        #[test]
        fn add_achievement_narrowed_api() {
            let mut player = Player::loaded_for_test("Player");
            let achievement_count_before = player.achievement_count();
            player.unlock_achievement(48);
            let achievement_count_after = player.achievement_count();
            assert_eq!(achievement_count_before+1, achievement_count_after);
            assert!(player.has_unlocked_achievement(48));
        }

        //*********************************************************************************
        // From video 4
        //*********************************************************************************
        #[test]
        fn player_can_boost_score() {
            let mut player = Player::new("Alice");
            boost(&mut player, 10);
            assert_eq!(player.score, 10);
        }
        #[test]
        fn player_complex_workflow() {
            let mut player = Player::new("Alice");
            boost(&mut player, 10);
            heal(&mut player, 5);
            damage(&mut player, 3);
            promote(&mut player);
            reset_score(&mut player);
            // and so on
        }

        #[test]
        fn player_complex_workflow_method() {
            let mut player = Player::new("Alice");
            player.boost(10);
            player.heal(5);
            player.damage(3);
            player.promote();
            player.reset_score();
            // and so on
        }

        fn boost(p: &mut Player, points: u32) {
            p.score += points;
        }
        fn reset_score(p: &mut Player) {
            todo!();
            todo!();
        }
        fn damage(p: &mut Player, life_points: i32) {

            p.score -= life_points as u32;
            todo!();
        }
        fn promote(p: &mut Player) {
            p.score *= 2;
            todo!();
        }
        fn heal(p: &mut Player, life_points: i32) {
            p.score += life_points as u32;
            todo!();
        }

        #[test]
        fn winner_method(){
            let mut player1 = Player::new("Player1");
            let mut player2 = Player::new("Player2");
            player1.add_to_score_mutation(100);
            player2.add_to_score_mutation(50);
            assert_eq!(player1.compute_winner(&player2), WinningOutcome1::Winner);
        }

        #[test]
        fn winner_free_function(){
            let mut player1 = Player::new("Player1");
            let mut player2 = Player::new("Player2");
            player1.add_to_score_mutation(100);
            player2.add_to_score_mutation(50);
            assert_eq!(compute_winner(&player1,&player2), WinningOutcome::FirstPlayerWins);
        }

        //*********************************************************************************
        // From video 5
        //*********************************************************************************
        #[test]
        fn player_can_take_snapshot() {
            let mut alice = Player::new("Player");
            let mut bob = Player::new("Player");
            let gametime = (GameTime{year: 2024, month: 1, day: 1, hour: 12, minute: 0, second: 0});
            alice.take_snapshot_borrowed(&gametime);
            bob.take_snapshot_borrowed(&gametime);

        }

        #[test]
        fn player_can_take_snapshot_owned() {
            let mut alice = Player::new("Player");
            let mut bob = Player::new("Player");
            let gametime = GameTime{year: 2024, month: 1, day: 1, hour: 12, minute: 0, second: 0};
            alice.take_snapshot_owned(gametime.clone());
            bob.take_snapshot_owned(gametime);
        }

    }
}
struct Game{
    players: Vec<Player>,
}

impl Game {
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
}

fn main() {
    let mut player1 = Player::new("Player1");
    let mut player2 = Player::new("Player2");
    player1.add_to_score_mutation(100);
    player2.add_to_score_mutation(50);

    match(player1.compute_winner(&player2)){
        WinningOutcome1::Winner => println!("{0} wins", player1.name),
        WinningOutcome1::Loser => println!("{0} wins", player2.name),
        WinningOutcome1::Draw => println!("Draw"),
    }

    match(compute_winner(&player1,&player2)){
        WinningOutcome::FirstPlayerWins => println!("{0} wins", player1.name),
        WinningOutcome::SecondPlayerWins => println!("{0} wins", player2.name),
        WinningOutcome::Draw => println!("Draw"),
    }

    let current_player = Player::new("Alice");
    log_player(current_player.clone());
    let mut game = Game{players: vec![]};
    game.add_player(current_player.clone());
    persist_player(current_player.clone());


}

fn persist_player(p0: Player) {
    todo!()
}

fn log_player(p: Player) {
    todo!()
}
