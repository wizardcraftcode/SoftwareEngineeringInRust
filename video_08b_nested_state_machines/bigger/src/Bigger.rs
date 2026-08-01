pub mod flat_marketplace {
    use std::time::Instant;

    // ==========================================
    // 0. Domain & Error Types
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

    /// Custom error type retaining ownership of the original state on failed transitions
    #[derive(Debug, PartialEq, Eq)]
    pub enum TradeError {
        InvalidTransition {
            current_state: TradeWorkflow,
        },
    }

    // ==========================================
    // 1. Flat State Machine Enum
    // ==========================================
    #[derive(Debug, PartialEq, Eq)]
    pub enum TradeWorkflow {
        Draft { seller: Player, item: Item, price: u32 },
        Offered { seller: Player, buyer: Player, item: Item, price: u32 },
        Accepted { seller: Player, buyer: Player, item: Item, price: u32 },
        AwaitingFunds { seller: Player, buyer: Player, item: Item, price: u32 },
        AwaitingItemTransfer { seller: Player, buyer: Player, item: Item, price: u32 },
        HeldInEscrow { seller: Player, buyer: Player, item: Item, price: u32, locked_at: Instant },
        Settled { seller: Player, buyer: Player, item: Item, price: u32 },
        Aborted { reason: RefundReason },
    }

    impl TradeWorkflow {
        pub fn new(seller: Player, item: Item, price: u32) -> Self {
            Self::Draft { seller, item, price }
        }

        pub fn offer_to_buyer(self, buyer: Player) -> Result<Self, TradeError> {
            match self {
                Self::Draft { seller, item, price } => {
                    Ok(Self::Offered { seller, buyer, item, price })
                }
                current_state => Err(TradeError::InvalidTransition {
                    current_state,
                }),
            }
        }

        pub fn accept(self) -> Result<Self, TradeError> {
            match self {
                Self::Offered { seller, buyer, item, price } => {
                    Ok(Self::Accepted { seller, buyer, item, price })
                }
                current_state => Err(TradeError::InvalidTransition {
                    current_state,
                }),
            }
        }

        pub fn request_funds(self) -> Result<Self, TradeError> {
            match self {
                Self::Accepted { seller, buyer, item, price } => {
                    Ok(Self::AwaitingFunds { seller, buyer, item, price })
                }
                current_state => Err(TradeError::InvalidTransition {
                    current_state,
                }),
            }
        }

        pub fn receive_funds(self) -> Result<Self, TradeError> {
            match self {
                Self::AwaitingFunds { seller, buyer, item, price } => {
                    Ok(Self::AwaitingItemTransfer { seller, buyer, item, price })
                }
                current_state => Err(TradeError::InvalidTransition {
                    current_state,
                }),
            }
        }

        pub fn lock_in_escrow(self) -> Result<Self, TradeError> {
            match self {
                Self::AwaitingItemTransfer { seller, buyer, item, price } => {
                    Ok(Self::HeldInEscrow {
                        seller,
                        buyer,
                        item,
                        price,
                        locked_at: Instant::now(),
                    })
                }
                current_state => Err(TradeError::InvalidTransition {
                    current_state,
                }),
            }
        }

        pub fn settle(self) -> Result<Self, TradeError> {
            match self {
                Self::HeldInEscrow { seller, buyer, item, price, .. } => {
                    Ok(Self::Settled { seller, buyer, item, price })
                }
                current_state => Err(TradeError::InvalidTransition {
                    current_state,
                }),
            }
        }

        /// Universal cancel operation (returns Err holding state if post-escrow/terminal)
        pub fn cancel(self, reason: RefundReason) -> Result<Self, TradeError> {
            match self {
                Self::Draft { .. } |
                Self::Offered { .. } |
                Self::Accepted { .. } |
                Self::AwaitingFunds { .. } |
                Self::AwaitingItemTransfer { .. } => {
                    Ok(Self::Aborted { reason })
                }
                Self::HeldInEscrow { .. } |
                Self::Settled { .. } |
                Self::Aborted { .. } => Err(TradeError::InvalidTransition {
                    current_state: self,
                }),
            }
        }
    }

    // ==========================================
    // 2. Unit Tests
    // ==========================================
    #[cfg(test)]
    mod tests {
        use super::*;

        fn setup_entities() -> (Player, Player, Item) {
            let seller = Player { id: 1, name: "Merchant_Bob".to_string() };
            let buyer = Player { id: 2, name: "Slayer_Alice".to_string() };
            let item = Item { id: 99, name: "Excalibur".to_string() };
            (seller, buyer, item)
        }

        #[test]
        fn test_flat_happy_path_with_results() -> Result<(), TradeError> {
            let (seller, buyer, item) = setup_entities();

            let trade = TradeWorkflow::new(seller, item, 500)
                .offer_to_buyer(buyer)?
                .accept()?
                .request_funds()?
                .receive_funds()?
                .lock_in_escrow()?
                .settle()?;

            assert!(matches!(trade, TradeWorkflow::Settled { .. }));
            Ok(())
        }

        #[test]
        fn test_invalid_transition_returns_original_state() {
            let (seller, _buyer, item) = setup_entities();

            let trade = TradeWorkflow::new(seller, item, 500);

            // Out-of-order transition: trying to accept directly from Draft
            let result = trade.accept();

            assert!(matches!(
                result,
                Err(TradeError::InvalidTransition {
                    current_state: TradeWorkflow::Draft { .. }
                })
            ));
        }

        #[test]
        fn test_cancel_after_escrow_returns_error_with_escrow_state() -> Result<(), TradeError> {
            let (seller, buyer, item) = setup_entities();

            let locked_trade = TradeWorkflow::new(seller, item, 500)
                .offer_to_buyer(buyer)?
                .accept()?
                .request_funds()?
                .receive_funds()?
                .lock_in_escrow()?;

            let cancel_result = locked_trade.cancel(RefundReason::UserCanceled);

            assert!(matches!(
                cancel_result,
                Err(TradeError::InvalidTransition {
                    current_state: TradeWorkflow::HeldInEscrow { .. }
                })
            ));

            Ok(())
        }
    }
}