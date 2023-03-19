pub mod authority_list;
pub mod utils;

use bridge_integrity::*;
use hdk::prelude::*;

// Called the first time a zome call is made to the cell containing this zome
#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    //TODO behave differently for progenitor vs anyone joining

    let my_agent_key = agent_info()?.agent_latest_pubkey;
    let properties = Properties::try_from(dna_info()?.properties).map_err(|_| {
        wasm_error!(WasmErrorInner::Guest(
            "Could not deserialize properties".into()
        ))
    })?;

    let my_eth_address = properties.progenitor_eth_address;
    let percentage: u32 = properties.percentage_for_consensus;

    //TODO move to proper validation
    match percentage {
        x if x > 50 && x <= 100 => {
            create_entry(EntryTypes::AuthorityList(AuthorityList {
                percentage_for_consensus: percentage,
                authorities: vec![(my_agent_key, my_eth_address)],
            }))?;

            Ok(InitCallbackResult::Pass)
        }
        _ => Ok(InitCallbackResult::Fail(
            "Percentage for consensus was invalid".into(),
        )),
    }
}

#[hdk_extern]
pub fn whoami(_: ()) -> ExternResult<AgentPubKey> {
    Ok(agent_info().unwrap().agent_initial_pubkey)
}

#[hdk_extern]
pub fn get_authority_list(_: ()) -> ExternResult<Record> {
    authority_list::handle_get_authority_list()
}

// Don't modify this enum if you want the scaffolding tool to generate appropriate signals for your entries and links
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Signal {
    EntryCreated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
    },
    EntryUpdated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
        original_app_entry: EntryTypes,
    },
    EntryDeleted {
        action: SignedActionHashed,
        original_app_entry: EntryTypes,
    },
}
// Whenever an action is committed, we emit a signal to the UI elements to reactively update them
#[hdk_extern(infallible)]
pub fn post_commit(committed_actions: Vec<SignedActionHashed>) {
    // Don't modify this loop if you want the scaffolding tool to generate appropriate signals for your entries and links
    for action in committed_actions {
        if let Err(err) = signal_action(action) {
            error!("Error signaling new action: {:?}", err);
        }
    }
}
// Don't modify this function if you want the scaffolding tool to generate appropriate signals for your entries and links
fn signal_action(action: SignedActionHashed) -> ExternResult<()> {
    match action.hashed.content.clone() {
        Action::Create(_create) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                emit_signal(Signal::EntryCreated { action, app_entry })?;
            }
            Ok(())
        }
        Action::Update(update) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                if let Ok(Some(original_app_entry)) =
                    get_entry_for_action(&update.original_action_address)
                {
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
            if let Ok(Some(original_app_entry)) = get_entry_for_action(&delete.deletes_address) {
                emit_signal(Signal::EntryDeleted {
                    action,
                    original_app_entry,
                })?;
            }
            Ok(())
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
        Some(EntryType::App(AppEntryDef {
            zome_index,
            entry_index,
            ..
        })) => (zome_index, entry_index),
        _ => {
            return Ok(None);
        }
    };
    Ok(EntryTypes::deserialize_from_type(
        zome_index.clone(),
        entry_index.clone(),
        entry,
    )?)
}
