use hdk::prelude::*;
use bridge_integrity::*;
use types::*;

#[hdk_extern]
pub fn create_authority_list(authority_list: AuthorityList) -> ExternResult<Record> {
    let authority_list_hash = create_entry(&EntryTypes::AuthorityList(authority_list.clone()))?;
    let record = get(authority_list_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from(
            "Could not find the newly created AuthorityList"
        ))
    ))?;
    Ok(record)
}

pub fn handle_get_authority_list() -> ExternResult<AuthorityList> {
    let progenitor_pub_key = Properties::new()?.progenitor_dht_address;
    let auth_list_entry_type: EntryType = UnitEntryTypes::AuthorityList.try_into()?;
    let filter = ChainQueryFilter::new().entry_type(auth_list_entry_type).include_entries(true);

    let activity = get_agent_activity(progenitor_pub_key, filter, ActivityRequest::Full)?;

    //if not exactly one create entry error
    if activity.valid_activity.len() != 1 {
        Err(wasm_error!("Invalid authority list number must be exactly 1, you found {}", activity.valid_activity.len()))
    } else {
        let (_, action_hash) = activity.valid_activity.first().unwrap();
        let details = get_details(action_hash.to_owned(), GetOptions::default())?;
        if let Details::Record(record_detail) = details.unwrap() {
            match record_detail.record.entry() {
                RecordEntry::Present(entry) => Ok(AuthorityList::try_from(entry)?),
                _ => Err(wasm_error!("Could not access the authority list"))
            }
        } else {
            Err(wasm_error!("Could not access the authority list"))
        }
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
    let record = get(updated_authority_list_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest(String::from(
            "Could not find the newly updated AuthorityList"
        ))),
    )?;
    Ok(record)
}
