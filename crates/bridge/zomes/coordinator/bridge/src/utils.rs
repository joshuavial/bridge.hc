use hdk::prelude::{*, holo_hash::ActionHashB64};

use bridge_hc_types::*;
use crate::properties::*;

use std::collections::BTreeMap;

//TODO remove - practice writing modules
pub fn get_progenitor_pub_key() -> ExternResult<AgentPubKey> {
    let properties = Properties::try_from(dna_info()?.properties)
        .map_err(|_| wasm_error!(WasmErrorInner::Guest("Malformed properties".into())))?;
    Ok(properties.progenitor_dht_address)
}

pub fn records_to_transactions(
    records: Vec<Record>,
) -> ExternResult<BTreeMap<ActionHashB64, Transaction>> {
    let transactions = records
        .into_iter()
        .map(|record| {
            let entry = record
                .entry()
                .as_option()
                .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Malformed transaction"))))?;

            let transaction = Transaction::try_from_entry(entry.clone())?;

            let hash_b64 = ActionHashB64::from(record.action_address().clone());

            Ok((hash_b64, transaction))
        })
        .collect::<ExternResult<BTreeMap<ActionHashB64, Transaction>>>()?;

    Ok(transactions)
}