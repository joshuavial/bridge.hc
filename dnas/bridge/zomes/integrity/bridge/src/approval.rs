use hdi::prelude::*;
use super::authority_list::AuthorityList;
use super::transaction::Transaction;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ProposedEntry {
    Transaction(Transaction),
    AuthorityList(AuthorityList),
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct Approval {
    pub proposed_entry: ProposedEntry,
    pub approved_by: Vec<AgentPubKey>,
}
pub fn validate_create_approval(
    _action: EntryCreationAction,
    _approval: Approval,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_approval(
    _action: Update,
    _approval: Approval,
    _original_action: EntryCreationAction,
    _original_approval: Approval,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_approval(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_approval: Approval,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_approval_updates(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = ActionHash::from(base_address);
    let record = must_get_valid_record(action_hash)?;
    let _approval: crate::Approval = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // Check the entry type for the given action hash
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _approval: crate::Approval = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_approval_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("ApprovalUpdates links cannot be deleted"),
        ),
    )
}
