use bridge_integrity::*;
use hdk::prelude::*;
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
pub fn handle_get_authority_list() -> ExternResult<AuthorityList> {
    let action_hash = original_action_hash()?;
    let details = get_details(action_hash.to_owned(), GetOptions::default())?;
    if let Details::Record(record_detail) = details.unwrap() {
        match record_detail.record.entry() {
            RecordEntry::Present(entry) => Ok(AuthorityList::try_from(entry)?),
            _ => Err(wasm_error!("Could not access the authority list")),
        }
    } else {
        Err(wasm_error!("Could not access the authority list"))
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAuthorityListInput {
    pub previous_authority_list_hash: ActionHash,
    pub updated_authority_list: AuthorityList,
}
pub fn handle_update_authority_list(new_authority_list: AuthorityList) -> ExternResult<ActionHash> {
    let updated_authority_list_hash = update_entry(
        latest_action_hash()?,
        new_authority_list,
    )?;
    Ok(updated_authority_list_hash)
}

fn latest_action_hash() -> ExternResult<ActionHash> {
    original_action_hash()
}

fn original_action_hash() -> ExternResult<ActionHash> {
    let progenitor_pub_key = Properties::new()?.progenitor_dht_address;
    let auth_list_entry_type: EntryType = UnitEntryTypes::AuthorityList.try_into()?;
    let filter = ChainQueryFilter::new()
        .entry_type(auth_list_entry_type)
        .include_entries(true);
    let activity = get_agent_activity(
        progenitor_pub_key,
        filter,
        ActivityRequest::Full,
    )?;
    if activity.valid_activity.len() != 1 {
        Err(
            wasm_error!(
                "Invalid authority list number must be exactly 1, you found {}", activity
                .valid_activity.len()
            ),
        )
    } else {
        let (_, action_hash) = activity.valid_activity.first().unwrap();
        Ok(action_hash.to_owned())
    }
}
