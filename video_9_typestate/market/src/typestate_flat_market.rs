pub mod typestate_marketplace {
    use std::time::Instant;

    // ==========================================
    // 0. Domain Types
    // ==========================================
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Player {
        pub id: u32,
        pub name: String,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Item {
        pub id: u32,
        pub name: String,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RefundReason {
        UserCanceled,
        BuyerInsufficientFunds,
        EscrowTimeout,
    }

    /// The fields shared by every state from `Offered` onward. Factoring
    /// this out (rather than repeating `seller`/`buyer`/`item`/`price` in
    /// each state struct, as the flat enum's variants did) is what keeps
    /// the transition impls below from turning into a wall of
    /// field-by-field copies.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TradeDetails {
        pub seller: Player,
        pub buyer: Player,
        pub item: Item,
        pub price: u32,
    }

    // ==========================================
    // 1. States
    // ==========================================
    // `Draft` has no buyer yet, so it can't hold a `TradeDetails`.

    pub struct Draft {
        seller: Player,
        item: Item,
        price: u32,
    }

    pub struct Offered(TradeDetails);

    pub struct Accepted(TradeDetails);

    pub struct AwaitingFunds(TradeDetails);

    pub struct AwaitingItemTransfer(TradeDetails);

    pub struct HeldInEscrow {
        details: TradeDetails,
        locked_at: Instant,
    }

    pub struct Settled(TradeDetails);

    pub struct Aborted {
        reason: RefundReason,
    }

    /// A trade, parameterized by its current state. The flat enum needed
    /// `TradeError::InvalidTransition { current_state: TradeWorkflow }` so
    /// callers could recover the original value after a rejected
    /// transition; here there's nothing to recover, because rejected
    /// transitions never type-check in the first place. `TradeError` is
    /// gone entirely.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Trade<State> {
        state: State,
    }

    // ==========================================
    // 2. Shared accessors
    // ==========================================
    // A small trait so `seller()`/`buyer()`/`item()`/`price()` can be
    // written once and apply to every state that carries a `TradeDetails`,
    // instead of five near-identical impl blocks.
    pub trait HasDetails {
        fn details(&self) -> &TradeDetails;
    }
    impl HasDetails for Offered {
        fn details(&self) -> &TradeDetails {
            &self.0
        }
    }
    impl HasDetails for Accepted {
        fn details(&self) -> &TradeDetails {
            &self.0
        }
    }
    impl HasDetails for AwaitingFunds {
        fn details(&self) -> &TradeDetails {
            &self.0
        }
    }
    impl HasDetails for AwaitingItemTransfer {
        fn details(&self) -> &TradeDetails {
            &self.0
        }
    }
    impl HasDetails for HeldInEscrow {
        fn details(&self) -> &TradeDetails {
            &self.details
        }
    }
    impl HasDetails for Settled {
        fn details(&self) -> &TradeDetails {
            &self.0
        }
    }

    impl<S: HasDetails> Trade<S> {
        pub fn seller(&self) -> &Player {
            &self.state.details().seller
        }
        pub fn buyer(&self) -> &Player {
            &self.state.details().buyer
        }
        pub fn item(&self) -> &Item {
            &self.state.details().item
        }
        pub fn price(&self) -> u32 {
            self.state.details().price
        }
    }


    impl Trade<HeldInEscrow> {
        pub fn locked_at(&self) -> Instant {
            self.state.locked_at
        }
    }

    impl Trade<Aborted> {
        pub fn reason(&self) -> RefundReason {
            self.state.reason
        }
    }

    // ==========================================
    // 3. Linear transitions (one impl block per state)
    // ==========================================
    impl Trade<Draft> {
        pub fn new(seller: Player, item: Item, price: u32) -> Self {
            Trade {
                state: Draft { seller, item, price },
            }
        }

        pub fn offer_to_buyer(self, buyer: Player) -> Trade<Offered> {
            Trade {
                state: Offered(TradeDetails {
                    seller: self.state.seller,
                    buyer,
                    item: self.state.item,
                    price: self.state.price,
                }),
            }
        }
    }

    impl Trade<Offered> {
        pub fn accept(self) -> Trade<Accepted> {
            Trade {
                state: Accepted(self.state.0),
            }
        }
    }

    impl Trade<Accepted> {
        pub fn request_funds(self) -> Trade<AwaitingFunds> {
            Trade {
                state: AwaitingFunds(self.state.0),
            }
        }
    }

    impl Trade<AwaitingFunds> {
        pub fn receive_funds(self) -> Trade<AwaitingItemTransfer> {
            Trade {
                state: AwaitingItemTransfer(self.state.0),
            }
        }
    }

    impl Trade<AwaitingItemTransfer> {
        pub fn lock_in_escrow(self) -> Trade<HeldInEscrow> {
            Trade {
                state: HeldInEscrow {
                    details: self.state.0,
                    locked_at: Instant::now(),
                },
            }
        }
    }

    impl Trade<HeldInEscrow> {
        pub fn settle(self) -> Trade<Settled> {
            Trade {
                state: Settled(self.state.details),
            }
        }
    }

    // ==========================================
    // 4. Cancel: one method shared by several states
    // ==========================================
    // The original `cancel` was a single match arm covering five variants
    // (`Draft | Offered | Accepted | AwaitingFunds |
    // AwaitingItemTransfer`) and rejecting three (`HeldInEscrow | Settled
    // | Aborted`). A typestate equivalent of "this method exists on
    // several, but not all, states" is a marker trait implemented only by
    // the states it should apply to, plus a single generic impl block.
    pub trait Cancelable {}
    impl Cancelable for Draft {}
    impl Cancelable for Offered {}
    impl Cancelable for Accepted {}
    impl Cancelable for AwaitingFunds {}
    impl Cancelable for AwaitingItemTransfer {}
    // Deliberately NOT implemented for HeldInEscrow, Settled, or Aborted:
    // that omission is what makes `.cancel()` on those a compile error.

    impl<S: Cancelable> Trade<S> {
        pub fn cancel(self, reason: RefundReason) -> Trade<Aborted> {
            Trade {
                state: Aborted { reason },
            }
        }
    }

    // ==========================================
    // 5. Unit Tests
    // ==========================================
    #[cfg(test)]
    mod tests {
        use super::*;

        fn setup_entities() -> (Player, Player, Item) {
            let seller = Player {
                id: 1,
                name: "Merchant_Bob".to_string(),
            };
            let buyer = Player {
                id: 2,
                name: "Slayer_Alice".to_string(),
            };
            let item = Item {
                id: 99,
                name: "Excalibur".to_string(),
            };
            (seller, buyer, item)
        }

        #[test]
        fn test_typestate_happy_path() {
            let (seller, buyer, item) = setup_entities();

            // No `?` anywhere: every step here is infallible, because the
            // method called only exists on the state that's actually
            // reachable from the previous one.
            let trade = Trade::<Draft>::new(seller.clone(), item.clone(), 500)
                .offer_to_buyer(buyer.clone())
                .accept()
                .request_funds()
                .receive_funds()
                .lock_in_escrow()
                .settle();

            assert_eq!(trade.seller(), &seller);
            assert_eq!(trade.buyer(), &buyer);
            assert_eq!(trade.item(), &item);
            assert_eq!(trade.price(), 500);
        }

        #[test]
        fn test_cancel_before_escrow() {
            let (seller, buyer, item) = setup_entities();

            // `Cancelable` is implemented for Draft, Offered, Accepted,
            // AwaitingFunds, and AwaitingItemTransfer, so `.cancel()`
            // works from any of them -- here it's called partway through,
            // right after `accept()`.
            let trade = Trade::<Draft>::new(seller, item, 500)
                .offer_to_buyer(buyer)
                .accept()
                .cancel(RefundReason::UserCanceled);

            assert_eq!(trade.reason(), RefundReason::UserCanceled);
        }

        // The original suite had two more tests:
        //
        //   test_invalid_transition_returns_original_state
        //     -> called `.accept()` on a freshly-created Draft and
        //        checked the returned `Err` held the original Draft.
        //        `Trade<Draft>` has no `accept()` method, so there's
        //        nothing to call and nothing to recover.
        //
        //   test_cancel_after_escrow_returns_error_with_escrow_state
        //     -> called `.cancel()` on a `HeldInEscrow` trade and
        //        checked the `Err` held the escrow state.
        //        `HeldInEscrow` doesn't implement `Cancelable`, so
        //        `.cancel()` isn't a method `Trade<HeldInEscrow>` has.
        //
        // Both are now compile errors rather than runtime `Err`s.
        // Uncomment either block in a real crate to see it rejected:
        //
        // let (seller, _buyer, item) = setup_entities();
        // let trade = Trade::<Draft>::new(seller, item, 500);
        // trade.accept();
        // // error[E0599]: no method named `accept` found for struct
        // // `Trade<Draft>` in the current scope
        //
        // let (seller, buyer, item) = setup_entities();
        // let locked_trade = Trade::<Draft>::new(seller, item, 500)
        //     .offer_to_buyer(buyer)
        //     .accept()
        //     .request_funds()
        //     .receive_funds()
        //     .lock_in_escrow();
        // locked_trade.cancel(RefundReason::UserCanceled);
        // // error[E0599]: no method named `cancel` found for struct
        // // `Trade<HeldInEscrow>` in the current scope
    }
}