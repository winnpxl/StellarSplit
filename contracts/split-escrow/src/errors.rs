use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    InvalidAmount = 4,
    InvalidFeeBps = 5,
    SplitNotFound = 6,
    SplitNotPending = 7,
    SplitNotReady = 8,
    TreasuryNotSet = 9,
    SplitCancelled = 10,
    NoFundsAvailable = 11,
    EscrowNotRefundable = 12,
    InvalidBridgeStatus = 13,
    OracleNotAuthorized = 14,
    VerificationNotFound = 15,
    BridgeNotFound = 16,
    PriceSubmissionInvalid = 17,
    ProofInvalid = 18,
    SplitNotFunded = 19,
    SplitReleased = 20,
    EscrowNotFound = 21,
    InvalidEscrowStatus = 22,
    EscrowExpired = 23,
}