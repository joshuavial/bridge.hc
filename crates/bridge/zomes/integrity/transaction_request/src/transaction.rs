use hdi::prelude::*;
use bridge_hc_types::*;

fn _balance_from_previous_transaction(
    for_agent: AgentPubKey,
    previous_transaction: Option<Transaction>,
) -> ExternResult<f64> {
    match previous_transaction {
        None => Ok(0.0),
        Some(txn) => {
            let party = txn.get_party(&for_agent)?;
            Ok(party.resulting_balance)
        }
    }
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
