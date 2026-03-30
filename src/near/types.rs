//! NEAR RPC types for transaction execution outcomes.
//!
//! This module contains types needed to work with NEAR transaction execution results,
//! copied from near-kit-rs for use in hot-near-kit-bindings.

use serde::Deserialize;
use std::collections::BTreeMap;

// ============================================================================
// Re-exports from external crates
// ============================================================================
pub use near_account_id::AccountId;
pub use near_gas::NearGas as Gas;
pub use near_token::NearToken;

// ============================================================================
// CryptoHash
// ============================================================================

/// 32-byte cryptographic hash.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CryptoHash([u8; 32]);

impl CryptoHash {
    /// Create a new CryptoHash from bytes.
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Get the underlying bytes slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl<'de> Deserialize<'de> for CryptoHash {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let bytes = bs58::decode(&s)
            .into_vec()
            .map_err(serde::de::Error::custom)?;
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom(format!(
                "expected 32 bytes, got {}",
                bytes.len()
            )));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(Self(arr))
    }
}

// ============================================================================
// PublicKey and Signature
// ============================================================================

/// Public key type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicKey {
    Ed25519([u8; 32]),
    Secp256k1([u8; 64]),
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl PublicKey {
    fn from_str(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(format!("invalid public key format: {s}"));
        }
        match parts[0] {
            "ed25519" => {
                let bytes = bs58::decode(parts[1])
                    .into_vec()
                    .map_err(|e| format!("invalid ed25519 key: {e}"))?;
                if bytes.len() != 32 {
                    return Err(format!("ed25519 key must be 32 bytes, got {}", bytes.len()));
                }
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&bytes);
                Ok(Self::Ed25519(arr))
            }
            "secp256k1" => {
                let bytes = bs58::decode(parts[1])
                    .into_vec()
                    .map_err(|e| format!("invalid secp256k1 key: {e}"))?;
                if bytes.len() != 64 {
                    return Err(format!("secp256k1 key must be 64 bytes, got {}", bytes.len()));
                }
                let mut arr = [0u8; 64];
                arr.copy_from_slice(&bytes);
                Ok(Self::Secp256k1(arr))
            }
            _ => Err(format!("unknown key type: {}", parts[0])),
        }
    }
}

/// Cryptographic signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Signature {
    Ed25519([u8; 64]),
    Secp256k1([u8; 65]),
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Signature {
    fn from_str(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(format!("invalid signature format: {s}"));
        }
        match parts[0] {
            "ed25519" => {
                let bytes = bs58::decode(parts[1])
                    .into_vec()
                    .map_err(|e| format!("invalid ed25519 signature: {e}"))?;
                if bytes.len() != 64 {
                    return Err(format!(
                        "ed25519 signature must be 64 bytes, got {}",
                        bytes.len()
                    ));
                }
                let mut arr = [0u8; 64];
                arr.copy_from_slice(&bytes);
                Ok(Self::Ed25519(arr))
            }
            "secp256k1" => {
                let bytes = bs58::decode(parts[1])
                    .into_vec()
                    .map_err(|e| format!("invalid secp256k1 signature: {e}"))?;
                if bytes.len() != 65 {
                    return Err(format!(
                        "secp256k1 signature must be 65 bytes, got {}",
                        bytes.len()
                    ));
                }
                let mut arr = [0u8; 65];
                arr.copy_from_slice(&bytes);
                Ok(Self::Secp256k1(arr))
            }
            _ => Err(format!("unknown signature type: {}", parts[0])),
        }
    }
}

// ============================================================================
// FinalExecutionOutcome and related types
// ============================================================================

/// Transaction execution status.
#[derive(Debug, Clone, Default, Deserialize)]
pub enum FinalExecutionStatus {
    #[default]
    NotStarted,
    Started,
    Failure(TxExecutionError),
    SuccessValue(String),
}

/// Final execution outcome from send_tx RPC.
#[derive(Debug, Clone, Deserialize)]
pub struct FinalExecutionOutcome {
    pub status: FinalExecutionStatus,
    pub transaction: TransactionView,
    pub transaction_outcome: ExecutionOutcomeWithId,
    pub receipts_outcome: Vec<ExecutionOutcomeWithId>,
}

impl FinalExecutionOutcome {
    pub fn is_success(&self) -> bool {
        matches!(&self.status, FinalExecutionStatus::SuccessValue(_))
    }

    pub fn is_failure(&self) -> bool {
        matches!(&self.status, FinalExecutionStatus::Failure(_))
    }

    pub fn failure_message(&self) -> Option<String> {
        match &self.status {
            FinalExecutionStatus::Failure(err) => Some(err.to_string()),
            _ => None,
        }
    }

    pub fn failure_error(&self) -> Option<&TxExecutionError> {
        match &self.status {
            FinalExecutionStatus::Failure(err) => Some(err),
            _ => None,
        }
    }

    pub fn transaction_hash(&self) -> &CryptoHash {
        &self.transaction_outcome.id
    }

    pub fn total_gas_used(&self) -> Gas {
        let tx_gas = self.transaction_outcome.outcome.gas_burnt.as_gas();
        let receipt_gas: u64 = self
            .receipts_outcome
            .iter()
            .map(|r| r.outcome.gas_burnt.as_gas())
            .sum();
        Gas::from_gas(tx_gas + receipt_gas)
    }
}

/// Transaction view in outcome.
#[derive(Debug, Clone, Deserialize)]
pub struct TransactionView {
    pub signer_id: AccountId,
    pub public_key: PublicKey,
    pub nonce: u64,
    pub receiver_id: AccountId,
    pub hash: CryptoHash,
    #[serde(default)]
    pub actions: Vec<ActionView>,
    pub signature: Signature,
    #[serde(default)]
    pub priority_fee: Option<u64>,
    #[serde(default)]
    pub nonce_index: Option<u16>,
}

/// Execution outcome with ID.
#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionOutcomeWithId {
    pub id: CryptoHash,
    pub outcome: ExecutionOutcome,
    #[serde(default)]
    pub proof: Vec<MerklePathItem>,
    pub block_hash: CryptoHash,
}

/// Execution outcome details.
#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionOutcome {
    pub executor_id: AccountId,
    pub gas_burnt: Gas,
    pub tokens_burnt: NearToken,
    pub logs: Vec<String>,
    pub receipt_ids: Vec<CryptoHash>,
    pub status: ExecutionStatus,
    #[serde(default)]
    pub metadata: Option<ExecutionMetadata>,
}

/// Execution status for receipts.
#[derive(Debug, Clone)]
pub enum ExecutionStatus {
    Unknown,
    Failure(ActionError),
    SuccessValue(String),
    SuccessReceiptId(CryptoHash),
}

impl<'de> serde::Deserialize<'de> for ExecutionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        enum Raw {
            Unknown,
            Failure(TxExecutionError),
            SuccessValue(String),
            SuccessReceiptId(CryptoHash),
        }

        match Raw::deserialize(deserializer)? {
            Raw::Unknown => Ok(Self::Unknown),
            Raw::Failure(TxExecutionError::ActionError(e)) => Ok(Self::Failure(e)),
            Raw::Failure(TxExecutionError::InvalidTxError(e)) => Err(
                serde::de::Error::custom(format!(
                    "unexpected InvalidTxError in receipt execution status: {e}"
                )),
            ),
            Raw::SuccessValue(v) => Ok(Self::SuccessValue(v)),
            Raw::SuccessReceiptId(h) => Ok(Self::SuccessReceiptId(h)),
        }
    }
}

/// Merkle path item.
#[derive(Debug, Clone, Deserialize)]
pub struct MerklePathItem {
    pub hash: CryptoHash,
    pub direction: MerkleDirection,
}

/// Merkle direction.
#[derive(Debug, Clone, Deserialize)]
pub enum MerkleDirection {
    Left,
    Right,
}

/// Execution metadata.
#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionMetadata {
    pub version: u32,
    #[serde(default)]
    pub gas_profile: Option<Vec<GasProfileEntry>>,
}

/// Gas profile entry.
#[derive(Debug, Clone, Deserialize)]
pub struct GasProfileEntry {
    pub cost_category: String,
    pub cost: String,
    #[serde(deserialize_with = "gas_dec_format")]
    pub gas_used: Gas,
}

fn gas_dec_format<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Gas, D::Error> {
    use serde::de::Error;
    
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum GasValue {
        String(String),
        Number(u64),
    }
    
    let value = match GasValue::deserialize(deserializer)? {
        GasValue::String(s) => s.parse::<u64>().map_err(Error::custom)?,
        GasValue::Number(n) => n,
    };
    
    Ok(Gas::from_gas(value))
}

// ============================================================================
// Action view types
// ============================================================================

/// Action view in transaction.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ActionView {
    CreateAccount,
    DeployContract { code: String },
    FunctionCall {
        method_name: String,
        args: String,
        gas: Gas,
        deposit: NearToken,
    },
    Transfer { deposit: NearToken },
    Stake {
        stake: NearToken,
        public_key: PublicKey,
    },
    AddKey {
        public_key: PublicKey,
        access_key: AccessKeyDetails,
    },
    DeleteKey { public_key: PublicKey },
    DeleteAccount { beneficiary_id: AccountId },
    Delegate {
        delegate_action: DelegateActionView,
        signature: Signature,
    },
    #[serde(rename = "DeployGlobalContract")]
    DeployGlobalContract { code: String },
    #[serde(rename = "DeployGlobalContractByAccountId")]
    DeployGlobalContractByAccountId { code: String },
    #[serde(rename = "UseGlobalContract")]
    UseGlobalContract { code_hash: CryptoHash },
    #[serde(rename = "UseGlobalContractByAccountId")]
    UseGlobalContractByAccountId { account_id: AccountId },
    #[serde(rename = "DeterministicStateInit")]
    DeterministicStateInit {
        code: GlobalContractIdentifierView,
        #[serde(default)]
        data: BTreeMap<String, String>,
        deposit: NearToken,
    },
    TransferToGasKey {
        public_key: PublicKey,
        deposit: NearToken,
    },
    WithdrawFromGasKey {
        public_key: PublicKey,
        amount: NearToken,
    },
}

/// Access key details.
#[derive(Debug, Clone, Deserialize)]
pub struct AccessKeyDetails {
    pub nonce: u64,
    pub permission: AccessKeyPermissionView,
}

/// Access key permission.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AccessKeyPermissionView {
    FunctionCallAccessKey(FunctionCallPermissionView),
    FullAccess,
}

/// Function call permission.
#[derive(Debug, Clone, Deserialize)]
pub struct FunctionCallPermissionView {
    #[serde(default)]
    pub allowance: Option<NearToken>,
    pub receiver_id: AccountId,
    #[serde(default)]
    pub method_names: Vec<String>,
}

/// Delegate action view.
#[derive(Debug, Clone, Deserialize)]
pub struct DelegateActionView {
    pub sender_id: AccountId,
    pub receiver_id: AccountId,
    pub actions: Vec<ActionView>,
    pub nonce: u64,
    pub max_block_height: u64,
    pub public_key: PublicKey,
}

/// Global contract identifier.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "GlobalContractIdCompat")]
pub enum GlobalContractIdentifierView {
    CodeHash(CryptoHash),
    AccountId(AccountId),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum GlobalContractIdCompat {
    CodeHash { hash: CryptoHash },
    AccountId { account_id: AccountId },
    DeprecatedCodeHash(CryptoHash),
    DeprecatedAccountId(AccountId),
}

impl From<GlobalContractIdCompat> for GlobalContractIdentifierView {
    fn from(compat: GlobalContractIdCompat) -> Self {
        match compat {
            GlobalContractIdCompat::CodeHash { hash }
            | GlobalContractIdCompat::DeprecatedCodeHash(hash) => Self::CodeHash(hash),
            GlobalContractIdCompat::AccountId { account_id }
            | GlobalContractIdCompat::DeprecatedAccountId(account_id) => {
                Self::AccountId(account_id)
            }
        }
    }
}

// ============================================================================
// Error types
// ============================================================================

/// Error returned by NEAR RPC for transaction/receipt failures.
#[derive(Debug, Clone, Deserialize)]
pub enum TxExecutionError {
    ActionError(ActionError),
    InvalidTxError(InvalidTxError),
}

impl std::fmt::Display for TxExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ActionError(e) => write!(f, "ActionError: {e}"),
            Self::InvalidTxError(e) => write!(f, "InvalidTxError: {e}"),
        }
    }
}

impl std::error::Error for TxExecutionError {}

/// Action execution error.
#[derive(Debug, Clone, Deserialize)]
pub struct ActionError {
    #[serde(default)]
    pub index: Option<u64>,
    pub kind: ActionErrorKind,
}

impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ActionError(kind={:?})", self.kind)
    }
}

/// Specific kind of action error.
#[derive(Debug, Clone, Deserialize)]
pub enum ActionErrorKind {
    AccountAlreadyExists { account_id: AccountId },
    AccountDoesNotExist { account_id: AccountId },
    CreateAccountOnlyByRegistrar {
        account_id: AccountId,
        predecessor_id: AccountId,
        registrar_account_id: AccountId,
    },
    CreateAccountNotAllowed {
        account_id: AccountId,
        predecessor_id: AccountId,
    },
    ActorNoPermission {
        account_id: AccountId,
        actor_id: AccountId,
    },
    DeleteKeyDoesNotExist {
        account_id: AccountId,
        public_key: PublicKey,
    },
    AddKeyAlreadyExists {
        account_id: AccountId,
        public_key: PublicKey,
    },
    DeleteAccountStaking { account_id: AccountId },
    LackBalanceForState {
        account_id: AccountId,
        amount: NearToken,
    },
    TriesToUnstake { account_id: AccountId },
    TriesToStake {
        account_id: AccountId,
        balance: NearToken,
        locked: NearToken,
        stake: NearToken,
    },
    InsufficientStake {
        account_id: AccountId,
        minimum_stake: NearToken,
        stake: NearToken,
    },
    FunctionCallError(FunctionCallError),
    NewReceiptValidationError(ReceiptValidationError),
    OnlyImplicitAccountCreationAllowed { account_id: AccountId },
    DeleteAccountWithLargeState { account_id: AccountId },
    DelegateActionInvalidSignature,
    DelegateActionSenderDoesNotMatchTxReceiver {
        receiver_id: AccountId,
        sender_id: AccountId,
    },
    DelegateActionExpired,
    DelegateActionAccessKeyError(InvalidAccessKeyError),
    DelegateActionInvalidNonce {
        ak_nonce: u64,
        delegate_nonce: u64,
    },
    DelegateActionNonceTooLarge {
        delegate_nonce: u64,
        upper_bound: u64,
    },
    GlobalContractDoesNotExist {
        identifier: GlobalContractIdentifierView,
    },
    GlobalContractAlreadyDeployed {
        identifier: GlobalContractIdentifierView,
    },
    GlobalContractPublishError {
        identifier: GlobalContractIdentifierView,
        reason: String,
    },
    DeterministicStateInitError {
        reason: String,
    },
    NotEnoughGasKeyBalanceForAction {
        account_id: AccountId,
        balance: NearToken,
        cost: NearToken,
        public_key: PublicKey,
    },
    InsufficientAccountBalanceForStateUsage {
        account_id: AccountId,
        balance: NearToken,
        cost: NearToken,
    },
    StorageError(StorageError),
}

impl std::fmt::Display for ActionErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Transaction validation error.
#[derive(Debug, Clone, Deserialize)]
pub enum InvalidTxError {
    InvalidAccessKeyError(InvalidAccessKeyError),
    InvalidSignerId { signer_id: String },
    SignerDoesNotExist { signer_id: AccountId },
    InvalidNonce { ak_nonce: u64, tx_nonce: u64 },
    NonceTooLarge { tx_nonce: u64, upper_bound: u64 },
    InvalidReceiverId { receiver_id: String },
    InvalidSignature,
    NotEnoughBalance {
        balance: NearToken,
        cost: NearToken,
        signer_id: AccountId,
    },
    LackBalanceForState {
        amount: NearToken,
        signer_id: AccountId,
    },
    CostOverflow,
    InvalidChain,
    Expired,
    ActionsValidation(ActionsValidationError),
    TransactionSizeExceeded { limit: u64, size: u64 },
    InvalidTransactionVersion,
    StorageError(StorageError),
    ShardCongested {
        congestion_level: f64,
        shard_id: u64,
    },
    ShardStuck {
        missed_chunks: u64,
        shard_id: u64,
    },
    InvalidNonceIndex {
        num_nonces: u16,
        #[serde(default)]
        tx_nonce_index: Option<u16>,
    },
    NotEnoughGasKeyBalance {
        balance: NearToken,
        cost: NearToken,
        signer_id: AccountId,
    },
    NotEnoughBalanceForDeposit {
        balance: NearToken,
        deposit: NearToken,
        signer_id: AccountId,
    },
    MaxNumberOfNoncesExceeded,
    InvalidAction,
}

impl std::fmt::Display for InvalidTxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Invalid access key error.
#[derive(Debug, Clone, Deserialize)]
pub enum InvalidAccessKeyError {
    AccessKeyNotFound {
        account_id: AccountId,
        public_key: PublicKey,
    },
    ReceiverMismatch {
        ak_receiver: String,
        tx_receiver: AccountId,
    },
    MethodNameMismatch { method_name: String },
    RequiresFullAccess,
    NotEnoughAllowance {
        account_id: AccountId,
        allowance: NearToken,
        cost: NearToken,
        public_key: PublicKey,
    },
    DepositWithFunctionCall,
}

impl std::fmt::Display for InvalidAccessKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Function call error.
#[derive(Debug, Clone, Deserialize)]
pub enum FunctionCallError {
    WasmUnknownError,
    #[serde(rename = "_EVMError")]
    EvmError,
    CompilationError(CompilationError),
    LinkError { msg: String },
    MethodResolveError(MethodResolveError),
    WasmTrap(WasmTrap),
    HostError(HostError),
    ExecutionError(String),
}

impl std::fmt::Display for FunctionCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExecutionError(msg) => write!(f, "execution error: {msg}"),
            _ => write!(f, "{self:?}"),
        }
    }
}

/// Compilation error.
#[derive(Debug, Clone, Deserialize)]
pub enum CompilationError {
    CodeDoesNotExist { account_id: AccountId },
    PrepareError(PrepareError),
    WasmerCompileError { msg: String },
}

/// Prepare error.
#[derive(Debug, Clone, Deserialize)]
pub enum PrepareError {
    Serialization,
    Deserialization,
    InternalMemoryDeclared,
    GasInstrumentation,
    StackHeightInstrumentation,
    Instantiate,
    Memory,
    TooManyFunctions,
    TooManyLocals,
}

/// Method resolve error.
#[derive(Debug, Clone, Deserialize)]
pub enum MethodResolveError {
    MethodEmptyName,
    MethodNotFound,
}

/// Wasm trap.
#[derive(Debug, Clone, Deserialize)]
pub enum WasmTrap {
    Unreachable,
    IncorrectCallIndirectSignature,
    MemoryOutOfBounds,
    AmbiguousTrap,
    IntegerOverflow,
    IntegerDivideByZero,
    InvalidConversionToInteger,
    UnreachableCodeReached,
    ExecutionLimitReached,
    StackOverflow,
    GenericTrap(String),
}

/// Host error.
#[derive(Debug, Clone, Deserialize)]
pub enum HostError {
    #[serde(rename = "GasExceeded")]
    GasLimitExceeded { gas_limit: u64 },
    #[serde(rename = "GasExceededOnSpecificShard")]
    GasLimitExceededOnSpecificShard { gas_limit: u64, shard_id: u64 },
    #[serde(rename = "MaxNumberOfLogsExceeded")]
    MaxNumberOfLogsExceeded { limit: u64 },
    #[serde(rename = "MaxTotalLogLengthExceeded")]
    MaxTotalLogLengthExceeded { limit: u64 },
    #[serde(rename = "MaxNumberOfEventsExceeded")]
    MaxNumberOfEventsExceeded { limit: u64 },
    #[serde(rename = "MaxTotalEventLengthExceeded")]
    MaxTotalEventLengthExceeded { limit: u64 },
    #[serde(rename = "MaxNumberOfKeysExceeded")]
    MaxNumberOfKeysExceeded { limit: u64 },
    #[serde(rename = "MaxKeyLengthExceeded")]
    MaxKeyLengthExceeded { limit: u64 },
    #[serde(rename = "MaxValueLengthExceeded")]
    MaxValueLengthExceeded { limit: u64 },
    #[serde(rename = "MaxStorageKeySizeExceeded")]
    MaxStorageKeySizeExceeded { limit: u64 },
    #[serde(rename = "MaxStorageValueSizeExceeded")]
    MaxStorageValueSizeExceeded { limit: u64 },
    #[serde(rename = "MaxNumberPromisesExceeded")]
    MaxNumberPromisesExceeded { limit: u64 },
    #[serde(rename = "MaxNumberInputDatasExceeded")]
    MaxNumberInputDatasExceeded { limit: u64 },
    #[serde(rename = "MaxInputDataSizeExceeded")]
    MaxInputDataSizeExceeded { limit: u64 },
    #[serde(rename = "MaxNumberOfBalancesExceeded")]
    MaxNumberOfBalancesExceeded { limit: u64 },
    #[serde(rename = "MaxTotalBalanceLengthExceeded")]
    MaxTotalBalanceLengthExceeded { limit: u64 },
    #[serde(rename = "MaxReceiptSizeExceeded")]
    MaxReceiptSizeExceeded { limit: u64 },
    #[serde(rename = "ContractSizeExceeded")]
    ContractSizeExceeded { limit: u64 },
    #[serde(rename = "Deprecated")]
    Deprecated { msg: String },
    #[serde(rename = "SmartContractPanic")]
    SmartContractPanic { msg: String },
    #[serde(rename = "HostErrorInStruct")]
    HostErrorInStruct { msg: String },
    #[serde(rename = "ProhibitedInView")]
    ProhibitedInView { method_name: String },
    #[serde(rename = "NumberPromisesExceeded")]
    NumberPromisesExceeded { number_of_promises: u64, limit: u64 },
    #[serde(rename = "InvalidAccountId")]
    InvalidAccountId { account_id: String },
    #[serde(rename = "InvalidPublicKey")]
    InvalidPublicKey { public_key: String },
    #[serde(rename = "InvalidUtf8")]
    InvalidUtf8 { msg: String },
    #[serde(rename = "InvalidUtf16")]
    InvalidUtf16 { msg: String },
    #[serde(rename = "InvalidReturnToken")]
    InvalidReturnToken,
    #[serde(rename = "IteratorError")]
    IteratorError { msg: String },
    #[serde(rename = "InvalidPromiseResultIndex")]
    InvalidPromiseResultIndex { result_idx: u64 },
    #[serde(rename = "InvalidRegisterId")]
    InvalidRegisterId { register_id: u64 },
    #[serde(rename = "IteratorWasInvalidated")]
    IteratorWasInvalidated { iterator_idx: u64 },
    #[serde(rename = "MemoryAccessViolation")]
    MemoryAccessViolation,
    #[serde(rename = "VMLogicErr")]
    VMLogicErr { msg: String },
}

/// Actions validation error.
#[derive(Debug, Clone, Deserialize)]
pub enum ActionsValidationError {
    UnknownAction,
    InvalidAccountId { account_id: String },
    InvalidPublicKey,
    InvalidFullPublicKey,
    DuplicatePublicKey,
    InvalidAmount,
    InvalidMethodName { method_name: String },
    InvalidDataReceiverId { account_id: String },
    InvalidSignerId { account_id: String },
    InvalidReceiverId { account_id: String },
    InvalidAccessKey,
    InvalidContractAccount,
    ContractSizeExceeded { limit: u64, size: u64 },
    FunctionCallZeroBytesCode,
    FunctionCallEmptyMethodName,
    DelegateActionInvalidSignature,
    DelegateActionInvalidReceiverId,
    InvalidGlobalContractIdentifier,
    DuplicateGlobalContractIdentifier,
    DeterministicStateInitInvalidDataKey,
    DeterministicStateInitInvalidDataValue,
    TransferToGasKeyZeroAmount,
    WithdrawFromGasKeyZeroAmount,
}

impl std::fmt::Display for ActionsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Receipt validation error.
#[derive(Debug, Clone, Deserialize)]
pub enum ReceiptValidationError {
    InvalidPredecessorId { account_id: String },
    InvalidReceiverId { account_id: String },
    InvalidSignerId { account_id: String },
    InvalidDataReceiverId { account_id: String },
    ReturnedValueLengthExceeded { length: u64, limit: u64 },
    NumberInputDataDependenciesExceeded {
        limit: u64,
        number_of_input_data_dependencies: u64,
    },
    ActionsValidation(ActionsValidationError),
    ReceiptSizeExceeded { limit: u64, size: u64 },
    InvalidRefundTo { account_id: String },
}

impl std::fmt::Display for ReceiptValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Storage error.
#[derive(Debug, Clone, Deserialize)]
pub enum StorageError {
    StorageInternalError,
    MissingTrieValue(MissingTrieValue),
    UnexpectedTrieValue,
    StorageInconsistentState(String),
    FlatStorageBlockNotSupported(String),
    MemTrieLoadingError(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Missing trie value error.
#[derive(Debug, Clone, Deserialize)]
pub enum MissingTrieValue {
    State(StateNotFound),
    FlatState(FlatStateNotFound),
}

/// State not found.
#[derive(Debug, Clone, Deserialize)]
pub struct StateNotFound {
    pub hash: CryptoHash,
}

/// Flat state not found.
#[derive(Debug, Clone, Deserialize)]
pub struct FlatStateNotFound {
    pub hash: CryptoHash,
}
