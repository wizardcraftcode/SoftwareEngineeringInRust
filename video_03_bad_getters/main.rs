use crate::game_play::Player;

mod game_play {

    // don't implement copy or clone - that would make us be able to have multiple objects with the same name
    #[derive(Debug)]
    pub struct Player {
        name: String,
        pub(crate) score: u32,
        achievements: Vec<u32>
    }

    // Things we built in this video
    impl Player {
        #[cfg(test)]
        pub(crate) fn loaded_for_test(name: &str) -> Self {
            Player { name: name.to_owned(), score: 0, achievements: vec![42, 64]}
        }

        pub fn unlock_achievement(&mut self, p0: u32) {
            self.achievements.push(p0);
        }

        // This made the test not compile, because the test needed to update the
        // Player, but the Player was immutable.
        pub fn achievements(&self) -> &[u32] {
            &self.achievements
        }

        // This worked, but returning the clone makes an unneeded copy and the clone
        // could be out of date very quickly.
        // pub fn achievements(&self) -> Vec<u32> {
        //     self.achievements.clone()
        // }

        pub fn achievement_count(&self) -> usize {
            self.achievements.len()
        }

        pub fn has_unlocked_achievement(&self, achievement_id: u32) -> bool {
            self.achievements.contains(&achievement_id)
        }


    }

     impl Player {

        pub fn new(name: &str) -> Self {
            if name.trim().len() == 0 {
                panic!("Player name cannot be empty");
            }
            Player { name: name.to_owned(), score: 0, achievements: vec!()}
        }

         pub fn add_to_score(&mut self, points: u32) {
             self.score += points;
         }

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

//***********************************************************************
// new tests in this video
//***********************************************************************
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

    }



fn main() {
    let mut p = Player::new("Player");
    p.add_to_score(100);
p.unlock_achievement(44);
    let has_it = p.has_unlocked_achievement(44);
    println!{"This should be true: {}", has_it};
    println!("{:?}", p);
}
