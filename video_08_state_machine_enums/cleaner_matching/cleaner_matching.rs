#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Player {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MatchmakingSlot {
    Empty,
    Searching(Player),
    Matched {
        player: Player,
        partner: Player,
        room_code: String,
    },
}

impl Default for MatchmakingSlot {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MatchmakingError {
    InvalidState(MatchmakingSlot),
}

impl MatchmakingSlot {
    /// Transitions Empty -> Searching.
    /// Returns Err(Self) if state transition is invalid.
    pub fn start_search(self, player: Player) -> Result<Self, MatchmakingError> {
        match self {
            MatchmakingSlot::Empty => Ok(MatchmakingSlot::Searching(player)),
            invalid_state => Err(MatchmakingError::InvalidState(invalid_state)),
        }
    }

    /// Transitions Searching -> Empty.
    /// Returns Err(Self) if state transition is invalid.
    pub fn cancel_search(self) -> Result<Self, MatchmakingError> {
        match self {
            MatchmakingSlot::Searching(_) =>
                Ok(MatchmakingSlot::Empty),
            invalid_state =>
                Err(MatchmakingError::InvalidState(invalid_state)),
        }
    }

    /// Transitions Searching -> Matched.
    pub fn complete_match(
        self,
        partner: Player,
        room_code: String,
    ) -> Result<Self, MatchmakingError> {
        match self {
            MatchmakingSlot::Searching(player) => Ok(MatchmakingSlot::Matched {
                player,
                partner,
                room_code,
            }),
            invalid_state => Err(MatchmakingError::InvalidState(invalid_state)),
        }
    }
}
// The equivalent allocation system loop in clean architecture
pub fn allocate_active_rooms(slots: &Vec<MatchmakingSlot>) -> Vec<String> {
    slots
        .iter()
        .filter_map(|slot| match slot {
            MatchmakingSlot::Matched { room_code, .. } => {
                Some(format!("Room {} cleanly allocated", room_code))
            }
            _ => None,
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    fn setup_players() -> (Player, Player) {
        (
            Player {
                id: 1,
                name: "Alice".to_string(),
            },
            Player {
                id: 2,
                name: "Bob".to_string(),
            },
        )
    }

    #[test]
    fn test_complete_clean_lifecycle() {
        let (alice, bob) = setup_players();
        let expected_alice = alice.clone();
        let expected_bob = bob.clone();

        let slot = MatchmakingSlot::default();
        assert_eq!(slot, MatchmakingSlot::Empty);

        // Step into Searching
        let slot = slot
            .start_search(alice)
            .expect("start_search should succeed when slot is Empty");

        // Step into Matched
        let matched_slot = slot
            .complete_match(bob, "ROOM-123".to_string())
            .expect("complete_match should succeed when slot is Searching");
        let MatchmakingSlot::Matched {
            player: p1,
            partner: p2,
            room_code: r,
        } = matched_slot
        else {
            panic!("complete_match should return a Matched slot, got: {matched_slot:?}")
        };
        assert_eq!(p1, expected_alice);
        assert_eq!(p2, expected_bob);
        assert_eq!(r, "ROOM-123");
    }

    #[test]
    fn test_cant_match_empty() {
        let (_alice, bob) = setup_players();

        let slot = MatchmakingSlot::default();
        slot.complete_match(bob, "ROOM-123".to_string())
            .expect_err("complete_match should fail when slot is Empty");
    }

    #[test]
    fn test_cant_cancel_matched() {
        let (alice, bob) = setup_players();
        let expected_alice = alice.clone();
        let expected_bob = bob.clone();

        let slot = MatchmakingSlot::default()
            .start_search(alice)
            .expect("start_search should succeed when slot is Empty")
            .complete_match(bob.clone(), "ROOM-123".to_string())
            .expect("complete_match should succeed when slot is Searching");

        let MatchmakingError::InvalidState(recovered_slot) = slot
            .cancel_search()
            .expect_err("canceling after match should fail");
        let MatchmakingSlot::Matched {
            player: p1,
            partner: p2,
            room_code: r,
        } = recovered_slot
        else {
            panic!("cancel_search should return a Matched slot, got: {recovered_slot:?}");
        };
        assert_eq!(p1, expected_alice);
        assert_eq!(p2, expected_bob);
        assert_eq!(r, "ROOM-123");
    }

    #[test]
    fn test_system_loop_allocates_ghost_rooms() {
        let (alice, bob) = setup_players();

        let slot = MatchmakingSlot::default();
        let slot = slot
            .start_search(alice)
            .expect("start_search should succeed when slot is Empty");
        let slot = slot
            .complete_match(bob, "ROOM-404".to_string())
            .expect("complete_match should succeed when slot is Searching");

        let err = slot
            .cancel_search()
            .expect_err("canceling after match should fail");
        let MatchmakingError::InvalidState(recovered_slot) = err;

        let cluster_slots = vec![recovered_slot];
        let active_rooms = allocate_active_rooms(&cluster_slots);

        assert_eq!(
            active_rooms.len(),
            1,
            "CRITICAL FAULT: The system allocated a server room for an empty slot!"
        );
    }
}
