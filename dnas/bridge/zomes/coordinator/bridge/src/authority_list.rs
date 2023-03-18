use hdk::prelude::*;
use bridge_integrity::*;
#[hdk_extern]
pub fn create_authority_list(authority_list: AuthorityList) -> ExternResult<Record> {
    let authority_list_hash = create_entry(
        &EntryTypes::AuthorityList(authority_list.clone()),
    )?;
    let record = get(authority_list_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created AuthorityList"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_authority_list(
    original_authority_list_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    get_latest_authority_list(original_authority_list_hash)
}
fn get_latest_authority_list(
    authority_list_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let details = get_details(authority_list_hash, GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest("AuthorityList not found".into())))?;
    let record_details = match details {
        Details::Entry(_) => {
            Err(wasm_error!(WasmErrorInner::Guest("Malformed details".into())))
        }
        Details::Record(record_details) => Ok(record_details),
    }?;
    if record_details.deletes.len() > 0 {
        return Ok(None);
    }
    match record_details.updates.last() {
        Some(update) => get_latest_authority_list(update.action_address().clone()),
        None => Ok(Some(record_details.record)),
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAuthorityListInput {
    pub previous_authority_list_hash: ActionHash,
    pub updated_authority_list: AuthorityList,
}
#[hdk_extern]
pub fn update_authority_list(input: UpdateAuthorityListInput) -> ExternResult<Record> {
    let updated_authority_list_hash = update_entry(
        input.previous_authority_list_hash,
        &input.updated_authority_list,
    )?;
    let record = get(updated_authority_list_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated AuthorityList"))
            ),
        )?;
    Ok(record)
}
