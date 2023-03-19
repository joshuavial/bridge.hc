use hdk::prelude::*;
use bridge_integrity::*;
#[hdk_extern]
pub fn create_approval(approval: Approval) -> ExternResult<Record> {
    let approval_hash = create_entry(&EntryTypes::Approval(approval.clone()))?;
    let record = get(approval_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Approval"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_approval(original_approval_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_approval_hash.clone(),
        LinkTypes::ApprovalUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_approval_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_approval_hash.clone(),
    };
    get(latest_approval_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateApprovalInput {
    pub original_approval_hash: ActionHash,
    pub previous_approval_hash: ActionHash,
    pub updated_approval: Approval,
}
#[hdk_extern]
pub fn update_approval(input: UpdateApprovalInput) -> ExternResult<Record> {
    let updated_approval_hash = update_entry(
        input.previous_approval_hash.clone(),
        &input.updated_approval,
    )?;
    create_link(
        input.original_approval_hash.clone(),
        updated_approval_hash.clone(),
        LinkTypes::ApprovalUpdates,
        (),
    )?;
    let record = get(updated_approval_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Approval"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_approval(original_approval_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_approval_hash)
}
