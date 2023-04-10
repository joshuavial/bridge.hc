use hdi::prelude::*;
use super::properties::*;
use types::*;

pub fn validate_create_authority_list(
    action: EntryCreationAction,
    authority_list: AuthorityList,
) -> ExternResult<ValidateCallbackResult> {
    let properties = Properties::new()?;
    if action.author().to_owned() != properties.progenitor_dht_address {
        return Ok(ValidateCallbackResult::Invalid(String::from("Only Progenitor can create an authority list")));
    }
    if action.action_seq().to_owned() != 4 {
        return Ok(ValidateCallbackResult::Invalid(String::from("The authority list must be created immediately at initialisation")));
    }
    if authority_list.percentage_for_consensus < 51 || authority_list.percentage_for_consensus > 100 {
        return Ok(ValidateCallbackResult::Invalid(String::from("Percentage for consensus must be greater than 50 and less than or equal to 100")));
    }
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
