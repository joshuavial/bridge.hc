use hdi::prelude::holo_hash::{ActionHashB64, AgentPubKeyB64};
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionParty {
    pub agent_pub_key: AgentPubKeyB64,
    pub previous_transaction_hash: Option<ActionHashB64>,
    pub resulting_balance: f64,
}

#[hdk_entry_helper]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub spender: TransactionParty,
    pub recipient: TransactionParty,
    pub amount: f64,
    pub info: SerializedBytes,
}

pub fn validate_create_transaction(
    _action: EntryCreationAction,
    _transaction: Transaction,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_transaction(
    _action: Update,
    _transaction: Transaction,
    _original_action: EntryCreationAction,
    _original_transaction: Transaction,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_transaction(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_transaction: Transaction,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Transactions cannot be deleted",
    )))
}
