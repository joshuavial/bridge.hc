pub mod approval;
pub mod authority_list;
pub mod utils;
use bridge_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let properties = Properties::new()?;
    let my_agent_key = agent_info()?.agent_latest_pubkey;
    if my_agent_key != properties.progenitor_dht_address {
        return Ok(InitCallbackResult::Pass);
    }
    let my_eth_address = properties.progenitor_eth_address;
    let percentage: u32 = properties.percentage_for_consensus;
    create_entry(
        EntryTypes::AuthorityList(AuthorityList {
            percentage_for_consensus: percentage,
            authorities: vec![(my_agent_key, my_eth_address)],
        }),
    )?;
    Ok(InitCallbackResult::Pass)
}
#[hdk_extern]
pub fn whoami(_: ()) -> ExternResult<AgentPubKey> {
    Ok(agent_info().unwrap().agent_initial_pubkey)
}
#[hdk_extern]
pub fn get_authority_list(_: ()) -> ExternResult<AuthorityList> {
    authority_list::handle_get_authority_list()
}

#[hdk_extern]
pub fn update_authority_list(new_authority_list: AuthorityList) -> ExternResult<ActionHash> {
    authority_list::handle_update_authority_list(new_authority_list)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Signal {
    EntryCreated { action: SignedActionHashed, app_entry: EntryTypes },
    EntryUpdated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
        original_app_entry: EntryTypes,
    },
    EntryDeleted { action: SignedActionHashed, original_app_entry: EntryTypes },
    LinkCreated { action: SignedActionHashed, link_type: LinkTypes },
    LinkDeleted { action: SignedActionHashed, link_type: LinkTypes },
}
#[hdk_extern(infallible)]
pub fn post_commit(committed_actions: Vec<SignedActionHashed>) {
    for action in committed_actions {
        if let Err(err) = signal_action(action) {
            error!("Error signaling new action: {:?}", err);
        }
    }
}
fn signal_action(action: SignedActionHashed) -> ExternResult<()> {
    match action.hashed.content.clone() {
        Action::Create(_create) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                emit_signal(Signal::EntryCreated {
                    action,
                    app_entry,
                })?;
            }
            Ok(())
        }
        Action::Update(update) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                if let Ok(Some(original_app_entry))
                    = get_entry_for_action(&update.original_action_address) {
                    emit_signal(Signal::EntryUpdated {
                        action,
                        app_entry,
                        original_app_entry,
                    })?;
                }
            }
            Ok(())
        }
        Action::Delete(delete) => {
            if let Ok(Some(original_app_entry))
                = get_entry_for_action(&delete.deletes_address) {
                emit_signal(Signal::EntryDeleted {
                    action,
                    original_app_entry,
                })?;
            }
            Ok(())
        }
        Action::CreateLink(create_link) => {
            if let Ok(Some(link_type))
                = LinkTypes::from_type(create_link.zome_index, create_link.link_type) {
                emit_signal(Signal::LinkCreated {
                    action,
                    link_type,
                })?;
            }
            Ok(())
        }
        Action::DeleteLink(delete_link) => {
            let record = get(
                    delete_link.link_add_address.clone(),
                    GetOptions::default(),
                )?
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("Failed to fetch CreateLink action"
                        .to_string())
                    ),
                )?;
            match record.action() {
                Action::CreateLink(create_link) => {
                    if let Ok(Some(link_type))
                        = LinkTypes::from_type(
                            create_link.zome_index,
                            create_link.link_type,
                        ) {
                        emit_signal(Signal::LinkDeleted {
                            action,
                            link_type,
                        })?;
                    }
                    Ok(())
                }
                _ => {
                    return Err(
                        wasm_error!(
                            WasmErrorInner::Guest("Create Link should exist".to_string())
                        ),
                    );
                }
            }
        }
        _ => Ok(()),
    }
}
fn get_entry_for_action(action_hash: &ActionHash) -> ExternResult<Option<EntryTypes>> {
    let record = match get_details(action_hash.clone(), GetOptions::default())? {
        Some(Details::Record(record_details)) => record_details.record,
        _ => {
            return Ok(None);
        }
    };
    let entry = match record.entry().as_option() {
        Some(entry) => entry,
        None => {
            return Ok(None);
        }
    };
    let (zome_index, entry_index) = match record.action().entry_type() {
        Some(EntryType::App(AppEntryDef { zome_index, entry_index, .. })) => {
            (zome_index, entry_index)
        }
        _ => {
            return Ok(None);
        }
    };
    Ok(
        EntryTypes::deserialize_from_type(
            zome_index.clone(),
            entry_index.clone(),
            entry,
        )?,
    )
}
