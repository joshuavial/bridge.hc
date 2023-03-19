use hdi::prelude::*;
//use web3::types::Address;

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct AuthorityList {
    pub percentage_for_consensus: u32,
    pub authorities: Vec<(AgentPubKey, String)>,
}

pub fn validate_create_authority_list(
    _action: EntryCreationAction,
    _authority_list: AuthorityList,
) -> ExternResult<ValidateCallbackResult> {
    //TODO 
    //only progenitor
    //must be seq 4
    //no more than one create ever
    //percentage_for_consensus must be > 50 <= 100
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_authority_list(
    _action: Update,
    _authority_list: AuthorityList,
    _original_action: EntryCreationAction,
    _original_authority_list: AuthorityList,
) -> ExternResult<ValidateCallbackResult> {
    //TODO 
    //must have quorum of countersignatures of current authorities
    //previous auth list
    //count signatures
    //check each countersigner is valid auth list participant
    //does have quorum
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_authority_list(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_authority_list: AuthorityList,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Authority Lists cannot be deleted",
    )))
}
