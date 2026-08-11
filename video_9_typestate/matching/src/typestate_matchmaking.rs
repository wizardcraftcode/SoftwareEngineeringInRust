#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Player {
    pub id: u32,
    pub name: String,
}

/// State: no player is currently searching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Empty;

/// State: a player is searching for a match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Searching {
    player: Player,
}

/// State: two players have been matched into a room.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matched {
    player: Player,
    partner: Player,
    room_code: String,
}

/// A matchmaking slot, parameterized by its current state.
///
/// Compare this to the enum version: there, `start_search`,
/// `cancel_search`, and `complete_match` all had to accept `self` in any
/// state and return `Result<Self, MatchmakingError>`, because the type
/// system couldn't tell you which variant you actually had. Here, each
/// state is its own type, so a `MatchmakingSlot<Searching>` and a
/// `MatchmakingSlot<Matched>` are as distinct as `String` and `u32` --
/// the compiler enforces the state machine's edges for you, and the
/// transition methods below are infallible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchmakingSlot<State> {
    state: State,
}

impl MatchmakingSlot<Empty> {
    pub fn new() -> Self {
        MatchmakingSlot { state: Empty }
    }

    /// Empty -> Searching.
    pub fn start_search(self, player: Player) -> MatchmakingSlot<Searching> {
        MatchmakingSlot {
            state: Searching { player },
        }
    }
}

impl Default for MatchmakingSlot<Empty> {
    fn default() -> Self {
        Self::new()
    }
}

impl MatchmakingSlot<Searching> {
    /// Searching -> Empty.
    pub fn cancel_search(self) -> MatchmakingSlot<Empty> {
        MatchmakingSlot { state: Empty }
    }

    /// Searching -> Matched.
    pub fn complete_match(self, partner: Player, room_code: String) -> MatchmakingSlot<Matched> {
        MatchmakingSlot {
            state: Matched {
                player: self.state.player,
                partner,
                room_code,
            },
        }
    }

    pub fn player(&self) -> &Player {
        &self.state.player
    }
}

impl MatchmakingSlot<Matched> {
    // Notice there's no `cancel_search` impl here. The old enum version
    // needed `test_cant_cancel_matched` to *prove at runtime* that this
    // was rejected. In the typestate version that test is unnecessary:
    // `MatchmakingSlot<Matched>` has no `cancel_search` method, so calling
    // it is a compile error, not a caught-and-recovered runtime error.

    pub fn player(&self) -> &Player {
        &self.state.player
    }

    pub fn partner(&self) -> &Player {
        &self.state.partner
    }

    pub fn room_code(&self) -> &str {
        &self.state.room_code
    }
}

/// Only slots that are provably `Matched` -- at compile time -- can be
/// passed in here. The old version's "ghost room" bug (an
/// `InvalidState(Matched)` recovered from a failed `cancel_search` still
/// getting counted as an active room by `allocate_active_rooms`) can't be
/// expressed anymore: there is no `Vec<MatchmakingSlot<Matched>>` you can
/// build except by actually completing a match.
pub fn allocate_active_rooms(slots: &[MatchmakingSlot<Matched>]) -> Vec<String> {
    slots
        .iter()
        .map(|slot| format!("Room {} cleanly allocated", slot.room_code()))
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

        let slot = MatchmakingSlot::<Empty>::new();

        // Step into Searching. Infallible: no `Result`/`expect` needed,
        // `start_search` only exists on `MatchmakingSlot<Empty>`.
        let slot = slot.start_search(alice);
        assert_eq!(slot.player(), &expected_alice);

        // Step into Matched.
        let matched_slot = slot.complete_match(bob, "ROOM-123".to_string());

        assert_eq!(matched_slot.player(), &expected_alice);
        assert_eq!(matched_slot.partner(), &expected_bob);
        assert_eq!(matched_slot.room_code(), "ROOM-123");
    }

    #[test]
    fn test_cancel_search_returns_to_empty() {
        let (alice, next_player) = setup_players();

        let slot = MatchmakingSlot::<Empty>::new().start_search(alice);
        let slot: MatchmakingSlot<Empty> = slot.cancel_search();

        // Proves we're really back to Empty: the only thing left to do
        // is start a new search.
        let _slot = slot.start_search(next_player);
    }

    #[test]
    fn test_allocate_active_rooms() {
        let (alice, bob) = setup_players();
        let matched = MatchmakingSlot::<Empty>::new()
            .start_search(alice)
            .complete_match(bob, "ROOM-404".to_string());

        let cluster_slots = vec![matched];
        let active_rooms = allocate_active_rooms(&cluster_slots);

        assert_eq!(active_rooms.len(), 1);
        assert_eq!(active_rooms[0], "Room ROOM-404 cleanly allocated");
    }

    // Three of the original tests don't have an equivalent here, and
    // that's the point of the typestate pattern rather than a gap:
    //
    //   test_cant_match_empty
    //     -> `MatchmakingSlot<Empty>` has no `complete_match` method.
    //   test_cant_cancel_matched
    //     -> `MatchmakingSlot<Matched>` has no `cancel_search` method.
    //   test_system_loop_allocates_ghost_rooms
    //     -> `allocate_active_rooms` only accepts
    //        `&[MatchmakingSlot<Matched>]`, so a slot that never
    //        completed a match can't be passed to it at all.
    //
    // These states are unreachable by construction now, so there's
    // nothing left to assert at runtime. Uncomment either block below
    // in a real crate to see the compiler reject it directly:
    //
    // let (_, bob) = setup_players();
    // let slot = MatchmakingSlot::<Empty>::new();
    // slot.complete_match(bob, "ROOM-123".to_string());
    // // error[E0599]: no method named `complete_match` found for struct
    // // `MatchmakingSlot<Empty>` in the current scope
    //
    // let (alice, bob) = setup_players();
    // let slot = MatchmakingSlot::<Empty>::new()
    //     .start_search(alice)
    //     .complete_match(bob, "ROOM-123".to_string());
    // slot.cancel_search();
    // // error[E0599]: no method named `cancel_search` found for struct
    // // `MatchmakingSlot<Matched>` in the current scope
}