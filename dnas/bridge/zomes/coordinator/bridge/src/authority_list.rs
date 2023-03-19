use bridge_integrity::*;
use hdk::prelude::*;
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
#[hdk_extern]
pub fn get_authority_list(_: ()) -> ExternResult<Option<Record>> {
    get_latest_authority_list()
}

fn get_latest_authority_list() -> ExternResult<Option<Record>> {
    //get progenitor source chain
    let properties = Properties::try_from(dna_info()?.properties)
        .map_err(|e| wasm_error!(WasmErrorInner::Guest("Malformed properties".into())))?;
    let progenitor_pub_key = properties.progenitor_dht_address;
    let auth_list_entry_type: EntryType = UnitEntryTypes::AuthorityList.try_into()?;
    let filter = ChainQueryFilter::new().entry_type(auth_list_entry_type);
    let activity = get_agent_activity(progenitor_pub_key, filter, ActivityRequest::Full)?;

    warn!("{:#?}", activity);

    //get first create get all auth list actions to get auth_list_hash
    //check one and only
    Ok(None)
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
