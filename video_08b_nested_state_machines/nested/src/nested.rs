pub mod nested_marketplace {
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

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct TradeContext {
        pub seller: Player,
        pub buyer: Player,
        pub item: Item,
        pub price: u32,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum TradeError {
        InvalidTransition {
            current_state: TradeWorkflow,
        },
    }

    // ==========================================
    // 1. Micro-State Machines
    // ==========================================

    #[derive(Debug, PartialEq, Eq)]
    pub enum NegotiationState {
        Draft { seller: Player, item: Item, price: u32 },
        Offered(TradeContext),
    }

    impl NegotiationState {
        pub fn new(seller: Player, item: Item, price: u32) -> Self {
            Self::Draft { seller, item, price }
        }

        pub fn offer(self, buyer: Player) -> Result<Self, Self> {
            match self {
                Self::Draft { seller, item, price } => {
                    Ok(Self::Offered(TradeContext { seller, buyer, item, price }))
                }
                current => Err(current),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum EscrowState {
        AwaitingFunds(TradeContext),
        AwaitingItemTransfer(TradeContext),
        HeldInEscrow {
            context: TradeContext,
            locked_at: Instant,
        },
    }

    impl EscrowState {
        pub fn confirm_funds(self) -> Result<Self, Self> {
            match self {
                Self::AwaitingFunds(context) => Ok(Self::AwaitingItemTransfer(context)),
                current => Err(current),
            }
        }

        pub fn confirm_item_transfer(self) -> Result<Self, Self> {
            match self {
                Self::AwaitingItemTransfer(context) => Ok(Self::HeldInEscrow {
                    context,
                    locked_at: Instant::now(),
                }),
                current => Err(current),
            }
        }

        pub fn is_locked(&self) -> bool {
            matches!(self, Self::HeldInEscrow { .. })
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum ResolutionState {
        Settled(TradeContext),
        Aborted { reason: RefundReason },
    }

    // ==========================================
    // 2. Macro-State Machine (Orchestrator)
    // ==========================================

    #[derive(Debug, PartialEq, Eq)]
    pub enum TradeWorkflow {
        Negotiating(NegotiationState),
        Securing(EscrowState),
        Finalized(ResolutionState),
    }

    impl TradeWorkflow {
        pub fn new(seller: Player, item: Item, price: u32) -> Self {
            Self::Negotiating(NegotiationState::new(seller, item, price))
        }

        pub fn offer_to_buyer(self, buyer: Player) -> Result<Self, TradeError> {
            match self {
                Self::Negotiating(state) =>
                    match state.offer(buyer) {
                        Ok(next) => Ok(Self::Negotiating(next)),
                        Err(prev) => Err(TradeError::InvalidTransition {
                            current_state: Self::Negotiating(prev),
                    }),
                },
                current_state => Err(TradeError::InvalidTransition { current_state }),
            }
        }

        /// Auto-Advances Phase: Accepting an offer automatically transitions into Securing(AwaitingFunds)
        pub fn accept_trade(self) -> Result<Self, TradeError> {
            match self {
                Self::Negotiating(NegotiationState::Offered(context)) => {
                    Ok(Self::Securing(EscrowState::AwaitingFunds(context)))
                }
                current_state => Err(TradeError::InvalidTransition { current_state }),
            }
        }

        pub fn confirm_funds(self) -> Result<Self, TradeError> {
            match self {
                Self::Securing(state) => match state.confirm_funds() {
                    Ok(next) => Ok(Self::Securing(next)),
                    Err(prev) => Err(TradeError::InvalidTransition {
                        current_state: Self::Securing(prev),
                    }),
                },
                current_state => Err(TradeError::InvalidTransition { current_state }),
            }
        }

        pub fn confirm_item_transfer(self) -> Result<Self, TradeError> {
            match self {
                Self::Securing(state) => match state.confirm_item_transfer() {
                    Ok(next) => Ok(Self::Securing(next)),
                    Err(prev) => Err(TradeError::InvalidTransition {
                        current_state: Self::Securing(prev),
                    }),
                },
                current_state => Err(TradeError::InvalidTransition { current_state }),
            }
        }

        /// Settle trade when locked in escrow
        pub fn settle_trade(self) -> Result<Self, TradeError> {
            match self {
                Self::Securing(EscrowState::HeldInEscrow { context, .. }) => {
                    Ok(Self::Finalized(ResolutionState::Settled(context)))
                }
                current_state => Err(TradeError::InvalidTransition { current_state }),
            }
        }

        pub fn cancel(self, reason: RefundReason) -> Result<Self, TradeError> {
            match self {
                Self::Negotiating(_) => Ok(Self::Finalized(ResolutionState::Aborted { reason })),
                Self::Securing(ref escrow) if !escrow.is_locked() => {
                    Ok(Self::Finalized(ResolutionState::Aborted { reason }))
                }
                current_state => Err(TradeError::InvalidTransition { current_state }),
            }
        }
    }

    // ==========================================
    // 3. Integration Tests (TDD Approved)
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
        fn test_happy_path_execution() -> Result<(), TradeError> {
            let (seller, buyer, item) = setup_entities();

            // Notice how smooth the chain is now without manual phase-shift calls!
            let trade = TradeWorkflow::new(seller, item, 500)
                .offer_to_buyer(buyer)?
                .accept_trade()? // Automatically shifts phase into Securing phase!
                .confirm_funds()?
                .confirm_item_transfer()?
                .settle_trade()?;

            assert!(matches!(
                trade,
                TradeWorkflow::Finalized(ResolutionState::Settled(..))
            ));

            Ok(())
        }
    }
}