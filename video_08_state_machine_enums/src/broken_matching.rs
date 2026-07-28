pub mod broken_matchmaking {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Player {
        pub id: u32,
        pub name: String,
    }

    pub struct MatchmakingSlot {
        pub is_searching: bool,
        pub player: Option<Player>,
        pub partner: Option<Player>,
        pub room_code: Option<String>,
    }

    impl MatchmakingSlot {}

    impl MatchmakingSlot {
        pub fn new() -> Self {
            Self {
                is_searching: false,
                player: None,
                partner: None,
                room_code: None,
            }
        }

        pub fn start_search(&mut self, player: Player) {
            self.is_searching = true;
            self.player = Some(player);
        }

        pub fn cancel_search(&mut self) {
            self.is_searching = false;
            self.player = None;
        }

        pub fn complete_match(&mut self,
                              partner: Player,
                              code: String) {
            if self.is_searching {
                self.partner = Some(partner);
                self.room_code = Some(code);
                self.is_searching = false;
            }
        }
    }

    // This simulates the background system thread or global routine
    // that scans slots to spin up physical server rooms.
    pub fn allocate_active_rooms(slots: &[MatchmakingSlot]) -> Vec<String> {
        let mut active_rooms = Vec::new();
        for slot in slots {
            // The logic looks sound to a junior dev: "If they aren't searching,
            // and we have a room code, grab the partner and allocate!"
            if !slot.is_searching && slot.room_code.is_some() {
                // Bug vectors: If the player canceled, slot.player is None,
                // but slot.partner still exists! We spin up a corrupt room.
                active_rooms.push(format!(
                    "Room {} allocated for Ghost Match",
                    slot.room_code.as_ref().unwrap()
                ));
            }
        }
        active_rooms
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
        fn test_happy_path_lifecycle() {
            let (alice, bob) = setup_players();
            let mut slot = MatchmakingSlot::new();

            slot.start_search(alice.clone());
            assert!(slot.is_searching);
            assert_eq!(slot.player.as_ref(), Some(&alice));

            slot.complete_match(bob.clone(), "ROOM-123".to_string());
            assert!(!slot.is_searching);
            assert_eq!(slot.partner.as_ref(), Some(&bob));
            assert_eq!(slot.room_code.as_deref(), Some("ROOM-123"));
        }

        #[test]
        fn test_exploit_ghost_partner_state() {
            let (alice, bob) = setup_players();
            let mut slot = MatchmakingSlot::new();

            slot.start_search(alice);
            slot.complete_match(bob, "ROOM-123".to_string());

            // Simulating a cancellation request that arrives late or out of order
            slot.cancel_search();

            // CRITICAL INVARIANT VIOLATION:
            // The slot thinks it's completely idle, yet old match metadata is leaked
            assert_eq!(slot.is_searching, false);
            assert!(slot.player.is_none());

            // These should be None, but they are leaking data!
            assert!(
                slot.partner.is_some(),
                "LEAK: Ghost partner remains inside an empty slot!"
            );
            assert!(
                slot.room_code.is_some(),
                "LEAK: Room code remains active for an idle slot!"
            );
        }

        #[test]
        fn test_exploit_match_without_search() {
            let (_, bob) = setup_players();
            let mut slot = MatchmakingSlot::new();

            // Invariant Violation: Forcing a match complete on a slot that never searched
            slot.complete_match(bob.clone(), "ROOM-666".to_string());

            // The method guard protects 'is_searching', but leaves the fields inconsistent
            assert!(!slot.is_searching);
            assert!(slot.player.is_none());
            assert!(slot.partner.is_none());
            assert!(slot.room_code.is_none());
        }

        #[test]
        fn test_system_loop_allocates_ghost_rooms() {
            let alice = Player {
                id: 1,
                name: "Alice".to_string(),
            };
            let bob = Player {
                id: 2,
                name: "Bob".to_string(),
            };

            let mut slot = MatchmakingSlot::new();
            slot.start_search(alice);
            slot.complete_match(bob, "ROOM-404".to_string());

            // Player cancels or disconnects right as match finishes
            slot.cancel_search();

            let cluster_slots = vec![slot];

            // SYSTEM FAULT: The allocation system searches the idle slot pool,
            // finds our corrupt metadata, and spins up a server room for a ghost match!
            let active_rooms = allocate_active_rooms(&cluster_slots);

            assert_eq!(
                active_rooms.len(),
                0,
                "CRITICAL FAULT: The system allocated a server room for an empty slot!"
            );
        }
    }
}
