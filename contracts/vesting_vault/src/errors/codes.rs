use soroban_sdk::contracterror;

/// Error codes returned by the vesting vault smart contract operations.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    // 🔐 General (100s)
    /// Caller is not authorized to invoke this function or perform this action
    Unauthorized = 100,
    /// Provided input parameter failed validation checks
    InvalidInput = 101,
    /// #13: Address is the zero address -- not allowed as a participant
    ZeroAddress = 102,
    /// #13: Duplicate addresses found where uniqueness is required
    DuplicateAuditor = 103,
    /// #13: Amount must be strictly positive (> 0)
    AmountMustBePositive = 104,
    /// #13: Vesting schedule ID must be strictly positive (> 0)
    InvalidVestingId = 105,
    /// #13: Timestamp exceeds the maximum reasonable date bound
    TimestampTooLarge = 106,

    // ⏳ Vesting (200s)
    /// Vesting schedule not found for the requested ID or user
    VestingNotFound = 200,
    /// Vesting vault was not found
    VaultNotFound = 201,
    /// Contract or vault has already been initialized
    AlreadyInitialized = 202,
    /// Contract or vault has not been initialized yet
    NotInitialized = 203,
    /// Contract operations are currently paused
    ContractPaused = 204,
    /// Cliff period has not yet been reached
    CliffNotReached = 205,
    /// No vested tokens available to claim at the current timestamp
    NothingToClaim = 206,
    /// All vested tokens have already been fully claimed
    AlreadyFullyClaimed = 207,
    /// Vesting vault has been revoked
    VaultRevoked = 208,
    /// Vesting vault has been frozen
    VaultFrozen = 209,
    /// Invalid vesting schedule parameters provided
    InvalidSchedule = 210,
    /// Associated milestone has not been completed
    MilestoneNotCompleted = 211,
    /// Amount provided is invalid or exceeds claimable limits
    InvalidAmount = 212,
    /// Vault has not been properly initialized
    VaultNotInitialized = 213,

    // 💰 Financial (300s)
    /// Insufficient balance to execute operation
    InsufficientBalance = 300,
    /// Insufficient funds in vault or contract escrow
    InsufficientFunds = 301,
    /// Token transfer operation failed
    TransferFailed = 302,

    // 🧪 LST Auto-Compounding (310s)
    /// #154: LST not configured for this vesting schedule
    LSTNotConfigured = 310,
    /// #154: LST auto-compounding not enabled
    LSTNotEnabled = 311,
    /// #154: LST pool shares not initialized
    LSTPoolNotInitialized = 312,
    /// #154: User has no shares in the LST pool
    NoUserShares = 313,
    /// #154: No shares to unbond
    NoSharesToUnbond = 314,
    /// #154: Unbonding already pending for this user
    UnbondingAlreadyPending = 315,
    /// #154: Unbonding queue is full (rate limit)
    UnbondingQueueFull = 316,
    /// #154: Unbonding period has not elapsed yet
    UnbondingPeriodNotElapsed = 317,
    /// #154: No unbonding request found
    NoUnbondingRequest = 318,
    /// #154: Exchange rate manipulation suspected
    ExchangeRateManipulationSuspected = 319,

    // 📜 Compliance (400s)
    /// KYC verification process has not been completed
    KycNotCompleted = 400,
    /// KYC verification credentials have expired
    KycExpired = 401,
    /// Account or recipient address is listed on a sanctions registry
    AddressSanctioned = 402,
    /// Account resides in a restricted regulatory jurisdiction
    JurisdictionRestricted = 403,
    /// Required legal disclosure signature is missing
    LegalSignatureMissing = 404,
    /// Submitted legal signature failed cryptographic validation
    LegalSignatureInvalid = 405,
    /// Automated regulatory compliance checks failed
    ComplianceCheckFailed = 406,
    /// Transaction amount exceeds anti-money laundering threshold
    AmlThresholdExceeded = 407,
    /// Compliance risk scoring evaluation is above permissible threshold
    RiskRatingTooHigh = 408,
    /// Identity or entity document verification verification failed
    DocumentVerificationFailed = 409,
    /// Investor accreditation qualification is invalid or unconfirmed
    AccreditationStatusInvalid = 410,
    /// Mandatory tax withholding or reporting validation failed
    TaxComplianceFailed = 411,
    /// Tax liquidation (swap/transfer) failed during claim processing
    TaxLiquidationFailed = 421,
    /// Regulatory compliance freeze or block is currently active
    RegulatoryBlockActive = 412,
    /// Beneficiary address has not been approved on the compliance whitelist
    WhitelistNotApproved = 413,
    /// Address violates internal or regulatory exclusion list
    BlacklistViolation = 414,
    /// Access attempted from a prohibited geographic region
    GeofencingRestriction = 415,
    /// Identity verification validity window has expired
    IdentityVerificationExpired = 416,
    /// Proof for legitimate source of funds has not been verified
    SourceOfFundsNotVerified = 417,
    /// Beneficial ownership identity has not been verified
    BeneficialOwnerNotVerified = 418,
    /// Account flagged as a Politically Exposed Person requiring enhanced due diligence
    PoliticallyExposedPerson = 419,
    /// Prohibited match detected on international sanctions watchlists
    SanctionsListHit = 420,

    // 🚨 Security (450s)
    /// Stream paused due to suspicious activity detection
    StreamPaused = 450,

    // ⚙️ System (900s)
    /// Generic integer overflow encountered during numerical processing
    Overflow = 900,
    /// #9: Integer arithmetic overflow detected in financial computation
    ArithmeticOverflow = 901,
    /// #9: Integer arithmetic underflow detected in financial computation
    ArithmeticUnderflow = 902,

    // 🗳️ Governance / DAO (500s)
    /// #223: No unvested balance found for the queried address
    NoUnvestedBalance = 500,
    /// Account has already cast a vote for this proposal
    AlreadyVoted = 501,
    /// Proposal voting window has already concluded
    VotingPeriodEnded = 502,
    /// Quorum participation threshold was not achieved
    QuorumNotMet = 503,
    /// Timelock period has not yet elapsed
    TimelockNotElapsed = 504,
    /// Path payment routing parameters have not been configured
    PathPaymentNotConfigured = 505,
    /// Path payment conversion feature is currently disabled
    PathPaymentDisabled = 506,
    /// Insufficient liquidity in liquidity pools to complete path payment
    InsufficientLiquidity = 507,

    // 🔑 Admin Recovery (600s)
    /// #226: Admin dead-man's switch not configured
    AdminSwitchNotConfigured = 600,
    /// #226: Admin inactivity timeout has not elapsed yet
    AdminInactivityNotElapsed = 601,
    /// #226: Admin switch already triggered
    AdminSwitchAlreadyTriggered = 602,
    /// #226: Recovery address cannot be the same as admin
    RecoveryAddressInvalid = 603,

    // 🔮 Oracle (700s)
    /// #228: Oracle circuit breaker is currently tripped — vault is frozen
    OracleCircuitBreakerActive = 700,
    /// #228: Price deviation exceeds the 30% threshold
    OraclePriceDeviationTooHigh = 701,

    // 🛡️ Self-Destruct Prevention (800s)
    /// #231: Cannot upgrade/delete contract while unvested balance > 0
    UpgradeBlockedByUnvestedFunds = 800,

    // 🔐 Zero-Knowledge Privacy (1000s)
    /// #269: ZK proof verification failed
    InvalidZKProof = 1000,
    /// #269: Attempted to claim more than the shielded amount
    OverClaimAttempt = 1001,
    /// #269: Master viewing key not authorized for clawback
    ViewingKeyUnauthorized = 1002,

    // 🔄 Schedule Consolidation (1100s)
    /// #276: Asset mismatch between schedules - cannot merge
    AssetMismatch = 1100,
    /// #276: Schedule IDs must belong to the calling user
    UnauthorizedScheduleAccess = 1101,
    /// #276: Cannot merge - would artificially accelerate unlock dates
    UnlockDateAcceleration = 1102,
    /// #276: At least 2 schedules required for merging
    InsufficientSchedules = 1103,
    /// #276: Schedule already merged or inactive
    ScheduleNotActive = 1104,
}